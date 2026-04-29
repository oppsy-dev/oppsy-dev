//! Global OSV database instance.

use std::collections::HashMap;

use osv_db::{
    OsvDb as OsvDbInner, OsvGsEcosystem, OsvGsEcosystems,
    types::{OsvRecord, OsvRecordId},
};
use package_analyzer::MultiAnalyzer;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use tracing::info;

use crate::{
    resources::{Resource, ResourceRegistry},
    settings::Settings,
    types::{ManifestId, ManifestType},
};

/// 10MB download chunk size
const DOWNLOAD_CHUNK_SIZE: u64 = 10 * 1024 * 1024;

/// A wrapper around [`osv_db::OsvDb`] managed as a process-wide singleton,
/// paired with one [`Analyzer`] per supported ecosystem.
#[derive(Debug)]
pub struct OsvDb {
    inner: OsvDbInner,
    analyzer: MultiAnalyzer,
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
            // .add(OsvGsEcosystem::Go)
            // .add(OsvGsEcosystem::PyPI)
            // .add(OsvGsEcosystem::Npm)
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
        let db = Self { inner, analyzer };
        info!(ecosystems, "OSV database successfully initialised");
        Ok(db)
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
