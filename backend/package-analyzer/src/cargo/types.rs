use osv_db::types::PackageName;
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CargoLock {
    #[serde(default, rename = "package")]
    pub packages: Vec<CargoPackage>,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub struct CargoPackage {
    pub name: PackageName,
    pub version: Version,
    pub source: Option<String>,
    pub checksum: Option<String>,
}
