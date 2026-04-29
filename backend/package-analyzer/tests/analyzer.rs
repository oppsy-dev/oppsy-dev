#![allow(clippy::unwrap_used)]

use std::{collections::HashSet, ops::Deref, sync::LazyLock};

use osv_db::{OsvDb, OsvGsEcosystems};
use package_analyzer::{
    ManifestId, Package, analyzer::Analyzer, cargo::CargoPackage, go::GoModule,
    gradle::GradlePackage, maven::MavenPackage, npm::NpmPackage, poetry::PoetryPackage,
    uv::UvPackage,
};
use test_case::test_case;

const MANIFEST_ID: &str = "test-manifest";

struct TestOsvDb(OsvDb);

impl Deref for TestOsvDb {
    type Target = OsvDb;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TestOsvDb {
    pub fn new() -> anyhow::Result<Self> {
        let osv_db = OsvDb::new(OsvGsEcosystems::all(), "tests/osv-db")?;
        Ok(Self(osv_db))
    }
}

static OSV_DB: LazyLock<TestOsvDb> = LazyLock::new(|| {
    std::thread::spawn(|| TestOsvDb::new().unwrap())
        .join()
        .unwrap()
});

#[test_case(
    Analyzer::<CargoPackage>::new,
    include_bytes!("packages/cargo-package/Cargo.lock"),
    &["RUSTSEC-2025-0112"]
)]
#[test_case(
    Analyzer::<NpmPackage>::new,
    include_bytes!("packages/npm-package/package-lock.json"),
    &["MAL-2026-1429", "GHSA-hqjg-pww4-pcgq"]
)]
#[test_case(
    Analyzer::<UvPackage>::new,
    include_bytes!("packages/uv-package/uv.lock"),
    &["GHSA-rcp6-88mm-9vgf", "GHSA-p2m9-wcp5-6qw3"]
)]
#[test_case(
    Analyzer::<PoetryPackage>::new,
    include_bytes!("packages/poetry-package/poetry.lock"),
    &["GHSA-rcp6-88mm-9vgf", "GHSA-p2m9-wcp5-6qw3"]
)]
#[test_case(
    Analyzer::<MavenPackage>::new,
    include_bytes!("packages/maven-package/dependencies.txt"),
    &["GHSA-599f-7c49-w659", "GHSA-jfh8-c2jp-5v3q"]
)]
#[test_case(
    Analyzer::<GradlePackage>::new,
    include_bytes!("packages/gradle-package/gradle.lockfile"),
    &["GHSA-599f-7c49-w659", "GHSA-jfh8-c2jp-5v3q"]
)]
#[test_case(
    Analyzer::<GoModule>::new,
    include_bytes!("packages/go-package/deps.json"),
    &["GHSA-8rf9-c59g-f82f", "GHSA-67q9-58vj-32qx"]
)]
#[tokio::test]
async fn test_analyzer<T: Package>(
    analyzer_gen: impl Fn() -> Analyzer<T>,
    manifest_bytes: &[u8],
    osv_record_ids: &[&str],
) {
    add_manifest_then_osv_record_check(&analyzer_gen, manifest_bytes, osv_record_ids);
    add_osv_record_then_package_check(&analyzer_gen, manifest_bytes, osv_record_ids);
}

/// Verifies that packages indexed via [`PackageAnalyzer::add_manifest`] are matched when
/// a relevant OSV record is subsequently added via [`PackageAnalyzer::add_osv_record`].
///
/// Order of operations:
/// 1. `add_manifest` — parses the manifest and indexes its packages; returns no hits
///    because no OSV records have been added yet.
/// 2. `add_osv_record` — indexes the vulnerability and cross-checks it against
///    already-known packages; returns the `(manifest_id, osv_record_id)` pairs that
///    match.
fn add_manifest_then_osv_record_check<T: Package>(
    analyzer_gen: impl Fn() -> Analyzer<T>,
    manifest_bytes: &[u8],
    osv_record_ids: &[&str],
) {
    let osv_db = &OSV_DB;
    let analyzer = analyzer_gen();

    // add_manifest first — returns empty since no OSV records are indexed yet
    let manifest_id = ManifestId::from(MANIFEST_ID);
    let record_ids = analyzer
        .add_manifest(manifest_id, manifest_bytes, osv_db)
        .unwrap();
    assert!(record_ids.is_empty());

    // add_osv_record second — matches against already-indexed packages
    for osv_record_id in osv_record_ids {
        let osv_record = osv_db
            .get_record(&osv_record_id.to_string())
            .unwrap()
            .unwrap();
        let hits = analyzer.add_osv_record(&osv_record).unwrap();
        assert_eq!(hits, vec![MANIFEST_ID.into()]);
    }
}

/// Verifies that an OSV record indexed via [`PackageAnalyzer::add_osv_record`] is matched
/// when a relevant manifest is subsequently added via [`PackageAnalyzer::add_manifest`].
///
/// Order of operations:
/// 1. `add_osv_record` — indexes the vulnerability; returns no hits because no packages
///    have been added yet.
/// 2. `add_manifest` — parses the manifest, indexes its packages, and cross-checks them
///    against already-known OSV records; returns the OSV record IDs that affect the
///    manifest's packages.
fn add_osv_record_then_package_check<T: Package>(
    analyzer_gen: impl Fn() -> Analyzer<T>,
    manifest_bytes: &[u8],
    osv_record_ids: &[&str],
) {
    let osv_db = &OSV_DB;
    let analyzer = analyzer_gen();

    // add_osv_record first — returns empty since no packages are indexed yet
    for osv_record_id in osv_record_ids {
        let osv_record = osv_db
            .get_record(&osv_record_id.to_string())
            .unwrap()
            .unwrap();
        let hits = analyzer.add_osv_record(&osv_record).unwrap();
        assert!(hits.is_empty());
    }

    // add_manifest second — matches against already-indexed OSV records
    let manifest_id = ManifestId::from(MANIFEST_ID);
    let record_ids = analyzer
        .add_manifest(manifest_id, manifest_bytes, osv_db)
        .unwrap();
    // compare them as sets
    let record_ids: HashSet<_> = record_ids.into_iter().collect();
    let expected_ids: HashSet<_> = osv_record_ids.iter().map(ToString::to_string).collect();
    assert_eq!(record_ids, expected_ids);
}
