use std::{collections::HashSet, hash::Hash};

use dashmap::DashMap;
use osv_analyzer::{Package, analyze};
use osv_db::OsvDb;
use osv_types::{OsvRecord, OsvRecordId, PackageName};

/// Abstraction over a package-ecosystem analyzer that cross-references manifest files
/// (`Cargo.lock`, `npm.lock`, `yarn.lock` etc.) against OSV vulnerability records.
#[derive(Debug)]
pub struct Analyzer<ManifestId: PartialEq + Eq + Hash> {
    /// Maps each known `Package` to the set of manifests that depend on it.
    pkg_manifests: DashMap<Package, HashSet<ManifestId>>,
    /// Maps a package name to all known `P` versions seen across manifests.
    pkgs_by_name: DashMap<PackageName, HashSet<Package>>,
    /// Maps a package name to all OSV record IDs that affect it.
    records_by_name: DashMap<PackageName, HashSet<OsvRecordId>>,
    /// Maps a manifest ID to all OSV record IDs that affect it.
    records_by_manifest: DashMap<ManifestId, HashSet<OsvRecordId>>,
}

impl<ManifestId: Clone + PartialEq + Eq + Hash> Analyzer<ManifestId> {
    /// Creates a new, empty [`Analyzer`].
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            pkg_manifests: DashMap::new(),
            pkgs_by_name: DashMap::new(),
            records_by_name: DashMap::new(),
            records_by_manifest: DashMap::new(),
        }
    }

    /// Returns all [`OsvRecordId`]s associated with the given `manifest_id`.
    ///
    /// The returned list reflects every vulnerability record that was matched
    /// against this manifest via [`Self::add_manifest`] or [`Self::add_osv_record`].
    /// Returns an empty [`Vec`] if the manifest ID is unknown.
    #[must_use]
    pub fn osv_records_for_manifest(
        &self,
        manifest_id: &ManifestId,
    ) -> Vec<OsvRecordId> {
        self.records_by_manifest
            .get(manifest_id)
            .map(|records| records.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns every [`OsvRecordId`] whose `affected` entries match the given
    /// package by name, ecosystem, and version.
    ///
    /// Unlike [`Self::osv_records_for_manifest`], this works at the package level —
    /// useful for surfacing which packages in a manifest each vulnerability touches.
    ///
    /// Returns an empty [`Vec`] if no recorded vulnerability affects the package.
    ///
    /// # Errors
    /// - If the [`OsvDb::get_record`] lookup fails.
    /// - If [`analyze`] fails for any candidate record.
    pub fn osv_records_for_package(
        &self,
        package: &Package,
        osv_db: &OsvDb,
    ) -> anyhow::Result<Vec<OsvRecordId>> {
        let Some(record_ids) = self.records_by_name.get(&package.name) else {
            return Ok(Vec::new());
        };

        let mut out = Vec::new();
        for record_id in record_ids.iter() {
            let Some(osv_record) = osv_db.get_record(record_id)? else {
                continue;
            };
            for affected_p in &osv_record.affected {
                if analyze(package, affected_p)? {
                    out.push(record_id.clone());
                    break;
                }
            }
        }
        Ok(out)
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

            if let Some(packages) = self.pkgs_by_name.get(&package.name) {
                for p in packages.iter() {
                    if analyze(p, affected_p)?
                        && let Some(manifests) = self.pkg_manifests.get(p)
                    {
                        for manifest_id in manifests.iter() {
                            hits.push(manifest_id.clone());
                            self.records_by_manifest
                                .entry(manifest_id.clone())
                                .or_default()
                                .insert(osv_record.id.clone());
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
        manifest_id: &ManifestId,
        packages: impl Iterator<Item = Package>,
        osv_db: &OsvDb,
    ) -> anyhow::Result<Vec<OsvRecordId>> {
        let mut records = HashSet::new();

        for p in packages {
            if let Some(record_ids) = self.records_by_name.get(&p.name) {
                for record_id in record_ids.iter() {
                    if let Some(osv_record) = osv_db.get_record(record_id)? {
                        for affected_p in &osv_record.affected {
                            if analyze(&p, affected_p)? {
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

            self.pkgs_by_name
                .entry(p.name.clone())
                .or_default()
                .insert(p);
        }

        let mut manifest_records = self
            .records_by_manifest
            .entry(manifest_id.clone())
            .or_default();
        for record_id in &records {
            manifest_records.insert(record_id.clone());
        }

        Ok(records.into_iter().collect())
    }
}
