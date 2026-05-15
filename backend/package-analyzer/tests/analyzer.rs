#![allow(clippy::unwrap_used)]

use std::{collections::HashSet, ops::Deref, sync::LazyLock};

use osv_analyzer::Package;
use osv_db::{OsvDb, OsvGsEcosystems};
use package_analyzer::Analyzer;
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

fn pkg(
    name: &str,
    version: &str,
    ecosystem: &str,
) -> Package {
    Package {
        name: name.to_owned(),
        version: version.to_owned(),
        ecosystem: ecosystem.parse().unwrap(),
    }
}

#[test_case(
    vec![
        pkg("wasmtime", "38.0.0", "crates.io")
    ],
    &["RUSTSEC-2025-0112"]
)]
#[test_case(
    vec![
        pkg("org.apache.commons:commons-text", "1.9", "Maven"),
        pkg("org.apache.logging.log4j:log4j-core", "2.14.1", "Maven")
    ],
    &["GHSA-599f-7c49-w659", "GHSA-jfh8-c2jp-5v3q"]
)]
#[tokio::test]
async fn test_analyzer(
    packages: Vec<Package>,
    osv_record_ids: &[&str],
) {
    add_manifest_then_osv_record_check(packages.clone(), osv_record_ids);
    add_osv_record_then_package_check(packages, osv_record_ids);
}

/// Verifies that packages indexed via [`Analyzer::add_manifest`] are matched when
/// a relevant OSV record is subsequently added via [`Analyzer::add_osv_record`].
///
/// Order of operations:
/// 1. `add_manifest` — parses the manifest and indexes its packages; returns no hits
///    because no OSV records have been added yet.
/// 2. `add_osv_record` — indexes the vulnerability and cross-checks it against
///    already-known packages; returns the `(manifest_id, osv_record_id)` pairs that
///    match.
fn add_manifest_then_osv_record_check(
    packages: Vec<Package>,
    osv_record_ids: &[&str],
) {
    let osv_db = &OSV_DB;
    let analyzer = Analyzer::new();

    let record_ids = analyzer
        .add_manifest(&MANIFEST_ID, packages.into_iter(), osv_db)
        .unwrap();
    assert!(record_ids.is_empty());

    // add_osv_record second — matches against already-indexed packages
    for osv_record_id in osv_record_ids {
        let osv_record = osv_db
            .get_record(&osv_record_id.to_string())
            .unwrap()
            .unwrap();
        let hits = analyzer.add_osv_record(&osv_record).unwrap();
        assert_eq!(hits, vec![MANIFEST_ID]);
    }
}

/// Verifies that an OSV record indexed via [`Analyzer::add_osv_record`] is matched
/// when a relevant manifest is subsequently added via [`Analyzer::add_manifest`].
///
/// Order of operations:
/// 1. `add_osv_record` — indexes the vulnerability; returns no hits because no packages
///    have been added yet.
/// 2. `add_manifest` — parses the manifest, indexes its packages, and cross-checks them
///    against already-known OSV records; returns the OSV record IDs that affect the
///    manifest's packages.
fn add_osv_record_then_package_check(
    packages: Vec<Package>,
    osv_record_ids: &[&str],
) {
    let osv_db = &OSV_DB;
    let analyzer = Analyzer::new();

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
    let record_ids = analyzer
        .add_manifest(&MANIFEST_ID, packages.into_iter(), osv_db)
        .unwrap();
    // compare them as sets
    let record_ids: HashSet<_> = record_ids.into_iter().collect();
    let expected_ids: HashSet<_> = osv_record_ids.iter().map(ToString::to_string).collect();
    assert_eq!(record_ids, expected_ids);
}

/// Verifies that [`Analyzer::osv_records_for_package`] returns the same OSV record
/// IDs that [`Analyzer::add_manifest`] attributed to the package, and reports an
/// empty list for packages that the analyzer has never seen.
#[test_case(
    vec![pkg("wasmtime", "38.0.0", "crates.io")],
    &["RUSTSEC-2025-0112"]
)]
#[test_case(
    vec![
        pkg("org.apache.commons:commons-text", "1.9", "Maven"),
        pkg("org.apache.logging.log4j:log4j-core", "2.14.1", "Maven"),
    ],
    &["GHSA-599f-7c49-w659", "GHSA-jfh8-c2jp-5v3q"]
)]
#[tokio::test]
async fn test_osv_records_for_package(
    packages: Vec<Package>,
    osv_record_ids: &[&str],
) {
    let osv_db = &OSV_DB;
    let analyzer: Analyzer<&'static str> = Analyzer::new();
    for osv_record_id in osv_record_ids {
        let osv_record = osv_db
            .get_record(&osv_record_id.to_string())
            .unwrap()
            .unwrap();
        analyzer.add_osv_record(&osv_record).unwrap();
    }
    analyzer
        .add_manifest(&MANIFEST_ID, packages.clone().into_iter(), osv_db)
        .unwrap();

    // The union of per-package OSV record IDs matches the full input set.
    let mut all_returned: HashSet<String> = HashSet::new();
    for package in &packages {
        let ids = analyzer.osv_records_for_package(package, osv_db).unwrap();
        for id in &ids {
            all_returned.insert(id.clone());
        }
    }
    let expected_ids: HashSet<String> = osv_record_ids.iter().map(ToString::to_string).collect();
    assert_eq!(all_returned, expected_ids);

    // A package that the analyzer has never seen returns nothing.
    let unknown = pkg("definitely-not-a-package", "0.0.0", "crates.io");
    assert!(
        analyzer
            .osv_records_for_package(&unknown, osv_db)
            .unwrap()
            .is_empty()
    );
}
