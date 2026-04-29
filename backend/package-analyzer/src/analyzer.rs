use std::collections::HashSet;

use common::ConvertTo;
use dashmap::DashMap;
use osv_db::{
    OsvDb,
    types::{OsvRecord, OsvRecordId, PackageName},
};

use crate::{ManifestId, Package};

/// Abstraction over a package-ecosystem analyzer that cross-references manifest files
/// (`Cargo.lock`, `npm.lock`, `yarn.lock` etc.) against OSV vulnerability records.
#[derive(Debug)]
pub struct Analyzer<P: Package> {
    /// Maps each known `Package` to the set of manifests that depend on it.
    pkg_manifests: DashMap<P, HashSet<ManifestId>>,
    /// Maps a package name to all known `P` versions seen across manifests.
    pkgs_by_name: DashMap<PackageName, HashSet<P>>,
    /// Maps a package name to all OSV record IDs that affect it.
    records_by_name: DashMap<PackageName, HashSet<OsvRecordId>>,
}

impl<P: Package> Analyzer<P> {
    /// Creates a new, empty [`Analyzer`].
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            pkg_manifests: DashMap::new(),
            pkgs_by_name: DashMap::new(),
            records_by_name: DashMap::new(),
        }
    }

    /// Registers an OSV vulnerability record and returns every [`ManifestId`] whose
    /// packages are affected by it, based on manifest files added so far.
    ///
    /// If no manifest files have been added yet, the record is still indexed so that
    /// future [`Self::add_manifest`] calls can match against it.
    pub fn add_osv_record(
        &self,
        osv_record: &OsvRecord,
    ) -> anyhow::Result<Vec<ManifestId>> {
        let mut hits = Vec::new();

        for affected_p in &osv_record.affected {
            let Some(ref package) = affected_p.package else {
                continue;
            };

            if !P::matches_ecosystem(package.ecosystem.ecosystem()) {
                continue;
            }

            if let Some(packages) = self.pkgs_by_name.get(&package.name) {
                for p in packages.iter() {
                    if p.evaluate(affected_p)?
                        && let Some(manifests) = self.pkg_manifests.get(p)
                    {
                        for manifest_id in manifests.iter() {
                            hits.push(manifest_id.clone());
                        }
                    }
                }
            }

            self.records_by_name
                .entry(package.name.clone())
                .or_default()
                .insert(osv_record.id.clone());
        }

        Ok(hits)
    }

    /// Parses a manifest file bytes (`Cargo.lock`, `npm.lock`, `yarn.lock` etc.) and
    /// registers all of its packages under `manifest_id`.
    ///
    /// Returns the IDs of every OSV record (from previously added records) that affects
    /// at least one package in the manifest. If no vulnerabilities are known yet, the
    /// packages are still indexed so that future [`Self::add_osv_record`] calls can match
    /// against them.
    ///
    /// # Errors
    ///
    /// - If `manifest_bytes` is not valid UTF-8 or cannot be parsed as the expected
    ///   manifest format.
    /// - If the [`OsvDb::get_record`] lookup fails.
    pub fn add_manifest(
        &self,
        manifest_id: impl ConvertTo<ManifestId>,
        manifest_bytes: &[u8],
        osv_db: &OsvDb,
    ) -> anyhow::Result<Vec<OsvRecordId>> {
        let manifest_id = manifest_id.convert()?;
        let mut records = HashSet::new();

        for p in P::parse_manifest(manifest_bytes)? {
            let name = p.name();
            if let Some(record_ids) = self.records_by_name.get(&name) {
                for record_id in record_ids.iter() {
                    if let Some(osv_record) = osv_db.get_record(record_id)? {
                        for affected_p in &osv_record.affected {
                            if p.evaluate(affected_p)? {
                                records.insert(record_id.clone());
                            }
                        }
                    }
                }
            }

            self.pkg_manifests
                .entry(p.clone())
                .or_default()
                .insert(manifest_id.clone());

            self.pkgs_by_name.entry(name).or_default().insert(p);
        }

        Ok(records.into_iter().collect())
    }
}
