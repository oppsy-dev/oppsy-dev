use poem_openapi::Object;

/// Identity of an affected package within its ecosystem.
#[derive(Debug, Clone, Object)]
pub struct Package {
    /// Ecosystem name, optionally with a suffix (e.g. `"Debian:10"`).
    pub ecosystem: String,
    /// Package name as used within the ecosystem.
    pub name: String,
    /// Optional Package URL.
    pub purl: Option<String>,
}

impl From<osv_db::types::Package> for Package {
    fn from(p: osv_db::types::Package) -> Self {
        Self {
            ecosystem: p.ecosystem.to_string(),
            name: p.name,
            purl: p.purl,
        }
    }
}
