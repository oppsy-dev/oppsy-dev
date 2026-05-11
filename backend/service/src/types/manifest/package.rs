use poem_openapi::Object;

use crate::types::osv::Ecosystem;

/// Manifest's package definition.
#[derive(Object, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManifestPackage {
    /// Manifest's package name.
    pub name: String,
    /// Manifest's Package version (e.g. `0.1.0`).
    pub version: String,
    /// The OSV ecosystem this package belongs to (e.g. `"crates.io"`, `"PyPI"`, `"npm"`).
    pub ecosystem: Ecosystem,
}

impl From<ManifestPackage> for osv_analyzer::Package {
    fn from(value: ManifestPackage) -> Self {
        Self {
            name: value.name,
            version: value.version,
            ecosystem: value.ecosystem.into(),
        }
    }
}
