use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use osv_db::{OsvDb as OsvDbInner, OsvGsEcosystems};
use osv_types::{OsvRecord, OsvRecordId};
use package_analyzer::Analyzer;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    db::{CoreDb, ManifestDb},
    notifier::Notifier,
    resources::{Resource, ResourceRegistry},
    settings::Settings,
    types::{ManifestId, ManifestPackage, WorkspaceId},
};

/// 10MB download chunk size
const DOWNLOAD_CHUNK_SIZE: u64 = 10 * 1024 * 1024;

/// A wrapper around [`osv_db::OsvDb`] managed as a process-wide singleton,
/// paired with one [`Analyzer`] per supported ecosystem.
#[derive(Debug)]
pub struct OsvDb {
    inner: OsvDbInner,
    analyzer: Analyzer<ManifestId>,
    pub sync: RwLock<OsvSyncState>,
}

#[derive(Debug)]
pub struct OsvSyncState {
    pub last_sync_at: DateTime<Utc>,
    pub last_sync_err: Option<anyhow::Error>,
}

#[async_trait::async_trait]
impl Resource for OsvDb {
    async fn init() -> anyhow::Result<Self>
    where Self: Sized {
        Self::init().await
    }
}

impl OsvDb {
    async fn init() -> anyhow::Result<Self> {
        let settings = ResourceRegistry::get::<Settings>()?;

        let ecosystems = settings
            .osv
            .osv_ecosystems
            .iter()
            .fold(OsvGsEcosystems::all(), |r, e| r.add(*e));
        let inner = OsvDbInner::new(ecosystems, &settings.osv.osv_db_path)?;

        download_latest(&inner).await?;
        let records = read_all_records(&inner)?;

        let manifest_db = ResourceRegistry::get::<ManifestDb>()?;
        let analyzer = build_analyzer(&manifest_db, &inner, &records)?;

        let ecosystems = inner.ecosystems().to_string();
        info!(ecosystems, "OSV database successfully initialised");

        Ok(Self {
            inner,
            analyzer,
            sync: OsvSyncState {
                last_sync_at: Utc::now(),
                last_sync_err: None,
            }
            .into(),
        })
    }

    /// Background task that syncs OSV data on the interval configured by
    /// [`Settings::osv_sync_interval`] and dispatches vulnerability notifications
    /// for any manifests affected by new or updated records.
    pub async fn sync_task(self: Arc<Self>) -> anyhow::Result<()> {
        let settings = ResourceRegistry::get::<Settings>()?;

        loop {
            tokio::time::sleep(settings.osv.osv_sync_interval).await;

            let Ok(core_db) = ResourceRegistry::get::<CoreDb>() else {
                continue;
            };
            let Ok(notifier) = ResourceRegistry::get::<Notifier>() else {
                continue;
            };

            let last_sync_err = run_sync(&self, &core_db, &notifier)
                .await
                .inspect_err(|e| tracing::error!(error = ?e, "OSV sync cycle failed"))
                .err();
            let last_sync_at = Utc::now();

            let mut sync = self.sync.write().await;
            *sync = OsvSyncState {
                last_sync_at,
                last_sync_err,
            };
        }
    }

    /// Returns all OSV records that have been matched against the given manifest.
    ///
    /// Returns an empty [`Vec`] if the manifest ID is unknown or has no matched records.
    pub fn osv_records_for_manifest(
        &self,
        manifest_id: &ManifestId,
    ) -> Vec<OsvRecordId> {
        self.analyzer.osv_records_for_manifest(manifest_id)
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

    /// Syncs the local OSV database with upstream and returns every
    /// `(manifest_id, record_id)` pair where a synced record affects a
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
                            acc.entry(manifest_id).or_default().push(record.clone());
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

    /// Registers a manifest with the analyzer and returns every OSV record that
    /// matches one of its packages.
    ///
    /// # Errors
    /// - If the manifest cannot be parsed.
    /// - If an [`Analyzer`] lock is poisoned.
    pub fn add_manifest(
        &self,
        manifest_id: &ManifestId,
        packages: Vec<ManifestPackage>,
    ) -> anyhow::Result<Vec<OsvRecord>> {
        let record_ids = self.analyzer.add_manifest(
            manifest_id,
            packages.into_iter().map(Into::into),
            &self.inner,
        )?;
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

async fn download_latest(inner: &OsvDbInner) -> anyhow::Result<()> {
    info!(
        ecosystems = %inner.ecosystems().to_string(),
        "Downloading OSV database..."
    );
    inner.download_latest(DOWNLOAD_CHUNK_SIZE).await?;
    Ok(())
}

fn read_all_records(inner: &OsvDbInner) -> anyhow::Result<Vec<OsvRecord>> {
    info!(path = %inner.location().display(), "Reading OSV records...");
    let inner = inner.clone();
    tokio::task::block_in_place(move || {
        anyhow::Ok(inner.records()?.par_bridge().collect::<Result<_, _>>()?)
    })
}

fn build_analyzer(
    manifest_db: &ManifestDb,
    inner: &OsvDbInner,
    records: &[OsvRecord],
) -> anyhow::Result<Analyzer<ManifestId>> {
    info!(osv_records = records.len(), "Building analyzer indexes...");
    tokio::task::block_in_place(|| {
        let analyzer = Analyzer::new();

        manifest_db.iter()?.par_bridge().try_for_each(|m| {
            let (manifest_id, manifest) = m?;
            analyzer.add_manifest(
                &manifest_id,
                manifest.packages.into_iter().map(Into::into),
                inner,
            )?;
            anyhow::Ok(())
        })?;

        records.par_iter().try_for_each(|r| {
            drop(analyzer.add_osv_record(r)?);
            anyhow::Ok(())
        })?;

        anyhow::Ok(analyzer)
    })
}

async fn run_sync(
    osv_db: &OsvDb,
    core_db: &Arc<CoreDb>,
    notifier: &Arc<Notifier>,
) -> anyhow::Result<()> {
    info!("Running scheduled OSV sync...");
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

        notifier
            .clone()
            .spawn_osv_events(core_db.clone(), *workspace_id, manifest_id, records);
    }

    Ok(())
}
