use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use osv_db::{
    OsvDb as OsvDbInner, OsvGsEcosystem, OsvGsEcosystems,
    types::{OsvRecord, OsvRecordId},
};
use package_analyzer::MultiAnalyzer;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{
    db::CoreDb,
    notifier::Notifier,
    resources::{Resource, ResourceRegistry},
    settings::Settings,
    types::{ManifestId, ManifestType, WorkspaceId},
};

/// 10MB download chunk size
const DOWNLOAD_CHUNK_SIZE: u64 = 10 * 1024 * 1024;

/// A wrapper around [`osv_db::OsvDb`] managed as a process-wide singleton,
/// paired with one [`Analyzer`] per supported ecosystem.
#[derive(Debug)]
pub struct OsvDb {
    inner: OsvDbInner,
    analyzer: MultiAnalyzer,
    pub sync: RwLock<OsvSyncState>,
}

#[derive(Debug)]
pub struct OsvSyncState {
    pub last_sync_at: DateTime<Utc>,
    pub last_sync_err: Option<anyhow::Error>,
}

#[async_trait::async_trait]
impl Resource for OsvDb {
    /// Initializes the [`Settings`] instance from environment variables to the
    /// [`ResourceRegistry`].
    ///
    /// Must be called exactly once at service startup before any call to [`Self::get`].
    ///
    /// # Errors
    /// - Returns an error if any required environment variable is absent or malformed.
    async fn init() -> anyhow::Result<Self>
    where Self: Sized {
        Self::init().await
    }
}

impl OsvDb {
    /// Initialises the global [`OsvDb`] instance rooted at the configured path.
    ///
    /// Must be called exactly once at service startup before any call to [`Self::get`].
    ///
    /// # Errors
    /// - Returns an error if the database directory cannot be opened.
    /// - Returns an error if `init` has already been called.
    async fn init() -> anyhow::Result<Self> {
        let settings = ResourceRegistry::get::<Settings>()?;
        let ecosystems = OsvGsEcosystems::all()
            .add(OsvGsEcosystem::Go)
            .add(OsvGsEcosystem::PyPI)
            .add(OsvGsEcosystem::Npm)
            .add(OsvGsEcosystem::CratesIo);

        // TODO: properly check if the data is existed already
        let inner = OsvDbInner::new(ecosystems, &settings.osv_db_path)?;
        info!(
            ecosystems = %inner.ecosystems().to_string(),
            "Downloading OSV database..."
        );
        inner.download_latest(DOWNLOAD_CHUNK_SIZE).await?;

        info!(path = %inner.location().display(), "Reading OSV records...");
        let records = tokio::task::spawn_blocking({
            let inner = inner.clone();
            move || read_records_parallel(&inner)
        })
        .await??;
        info!(
            total = records.len(),
            "Read OSV records, building indexes..."
        );

        let analyzer = tokio::task::spawn_blocking(move || {
            let analyzer = MultiAnalyzer::new();
            records.par_iter().try_for_each(|r| {
                let hits = analyzer.add_osv_record(r)?;
                anyhow::ensure!(
                    hits.is_empty(),
                    "unexpected manifest hits while indexing OSV records at startup"
                );
                anyhow::Ok(())
            })?;
            anyhow::Ok(analyzer)
        })
        .await??;

        let ecosystems = inner.ecosystems().to_string();
        let db = Self {
            inner,
            analyzer,
            sync: OsvSyncState {
                last_sync_at: Utc::now(),
                last_sync_err: None,
            }
            .into(),
        };
        info!(ecosystems, "OSV database successfully initialised");
        Ok(db)
    }

    /// Background task that syncs OSV data on the interval configured by
    /// [`Settings::osv_sync_interval`] and dispatches vulnerability notifications
    /// for any manifests affected by new or updated records.
    ///
    /// Sleeps for one full interval before the first sync. [`Settings`] is registered
    /// synchronously before any tasks are spawned, so the get succeeds immediately.
    /// All other resources are guaranteed to be registered by then. Sync failures are
    /// logged and retried on the next cycle.
    pub async fn sync_task(self: Arc<Self>) -> anyhow::Result<()> {
        let settings = ResourceRegistry::get::<Settings>()?;

        loop {
            tokio::time::sleep(settings.osv_sync_interval).await;

            let Ok(core_db) = ResourceRegistry::get::<CoreDb>() else {
                continue;
            };
            let Ok(notifier) = ResourceRegistry::get::<Notifier>() else {
                continue;
            };

            let last_sync_err = run_sync(&self, &core_db, &notifier)
                .await
                .inspect_err(|e| error!(error = ?e, "OSV sync cycle failed"))
                .err();
            let last_sync_at = Utc::now();

            let mut sync = self.sync.write().await;
            *sync = OsvSyncState {
                last_sync_at,
                last_sync_err,
            };
        }
    }

    /// Looks up a single OSV record by its ID.
    ///
    /// Returns `Ok(None)` if no record matching `id` exists in the local database.
    ///
    /// # Errors
    /// - If the record file cannot be opened or deserialized.
    pub fn get_record(
        &self,
        id: &OsvRecordId,
    ) -> anyhow::Result<Option<OsvRecord>> {
        Ok(self.inner.get_record(id)?)
    }

    /// Syncs the local OSV database with upstream and applies newly added or updated
    /// records to the analyzer in parallel.
    ///
    /// Downloads only records modified since [`osv_db::OsvDb::last_modified`] and
    /// updates the local disk cache. Each new record is then applied to the
    /// [`MultiAnalyzer`] using rayon.
    ///
    /// Returns every `(manifest_id, record_id)` pair where a synced record affects a
    /// manifest previously registered via [`Self::add_manifest`].
    ///
    /// # Errors
    /// - If the upstream sync request fails.
    /// - If a synced record cannot be deserialized.
    pub async fn sync(&self) -> anyhow::Result<HashMap<ManifestId, Vec<OsvRecord>>> {
        // [`osv_db::OsvDb::sync`] could return duplicated OsvRecords.
        let new_records_iter = self.inner.sync().await?;
        tokio::task::block_in_place(|| {
            let new_records: Vec<OsvRecord> = new_records_iter.collect::<Result<_, _>>()?;
            info!(
                count = new_records.len(),
                "Applying synced OSV records to analyzer..."
            );

            let manifest_records = new_records
                .par_iter()
                .try_fold(
                    HashMap::<ManifestId, Vec<OsvRecord>>::new,
                    |mut acc, record| {
                        for manifest_id in self.analyzer.add_osv_record(record)? {
                            acc.entry(manifest_id.try_into()?)
                                .or_default()
                                .push(record.clone());
                        }
                        anyhow::Ok(acc)
                    },
                )
                .try_reduce(HashMap::new, |mut a, b| {
                    for (k, v) in b {
                        a.entry(k).or_default().extend(v);
                    }
                    anyhow::Ok(a)
                })?;

            Ok(manifest_records)
        })
    }

    /// Registers a manifest with the appropriate ecosystem [`Analyzer`] and returns
    /// every OSV record ID that matches one of its packages.
    ///
    /// Dispatches to the correct analyzer based on `manifest_type`, acquires a write
    /// lock, and delegates to [`Analyzer::add_manifest`].
    ///
    /// # Errors
    /// - If the manifest cannot be parsed for the given `manifest_type`.
    /// - If an [`Analyzer`] lock is poisoned.
    pub fn add_manifest(
        &self,
        manifest_type: ManifestType,
        manifest_id: &ManifestId,
        manifest_bytes: &[u8],
    ) -> anyhow::Result<Vec<OsvRecord>> {
        let record_ids =
            self.analyzer
                .add_manifest(manifest_type, manifest_id, manifest_bytes, &self.inner)?;
        record_ids
            .iter()
            .map(|id| {
                self.inner
                    .get_record(id)?
                    .ok_or(anyhow::anyhow!("OSV record for id '{id}' not found"))
            })
            .collect::<anyhow::Result<_>>()
    }
}

/// Reads all OSV record JSON files from `records_dir` in parallel using rayon.
///
/// Each file is read and deserialized independently, so the work scales with
/// the number of available CPU cores.
fn read_records_parallel(db: &osv_db::OsvDb) -> anyhow::Result<Vec<OsvRecord>> {
    let records = db.records()?.par_bridge().collect::<Result<_, _>>()?;
    Ok(records)
}

async fn run_sync(
    osv_db: &OsvDb,
    core_db: &Arc<CoreDb>,
    notifier: &Arc<Notifier>,
) -> anyhow::Result<()> {
    info!("Running scheduled OSV sync...");
    let detected_at = Utc::now();
    let hits = osv_db.sync().await?;

    if hits.is_empty() {
        info!("OSV sync complete: no new or updated records");
        return Ok(());
    }

    info!(
        affected_manifests = hits.len(),
        "OSV sync: resolving workspaces for affected manifests..."
    );

    let manifest_to_workspace: HashMap<ManifestId, WorkspaceId> = core_db
        .get_manifest_workspace_map()
        .await?
        .into_iter()
        .map(|(m, v)| anyhow::Ok((m.try_into()?, v.try_into()?)))
        .collect::<Result<_, _>>()?;

    for (manifest_id, records) in hits {
        let workspace_id = manifest_to_workspace
            .get(&manifest_id)
            .ok_or(anyhow::anyhow!(
                "Manifest {manifest_id} is not assinged to any workspaces"
            ))?;

        core_db
            .add_manifest_osv_vuln(
                manifest_id,
                records.iter().map(|v| v.id.clone()).collect(),
                detected_at.timestamp(),
            )
            .await?;
        // spawn notifications
        notifier
            .clone()
            .spawn_osv_events(core_db.clone(), *workspace_id, manifest_id, records);
    }

    Ok(())
}
