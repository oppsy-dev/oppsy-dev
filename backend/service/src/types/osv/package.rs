use poem_openapi::Object;

use super::ecosystem::Ecosystem;

/// Identity of an affected package within its ecosystem.
#[derive(Debug, Clone, Object)]
pub struct Package {
    /// Ecosystem name, optionally with a suffix (e.g. `"Debian:10"`).
    pub ecosystem: Ecosystem,
    /// Package name as used within the ecosystem.
    pub name: String,
    /// Optional Package URL.
    pub purl: Option<String>,
}

impl From<osv_types::Package> for Package {
    fn from(p: osv_types::Package) -> Self {
        Self {
            ecosystem: p.ecosystem.into(),
            name: p.name,
            purl: p.purl,
        }
    }
}
