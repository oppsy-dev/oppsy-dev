use poem_openapi::Object;

use super::{Package, Range, Severity};

/// A single affected package entry.
#[derive(Debug, Clone, Object)]
pub struct Affected {
    /// The affected package identity.
    pub package: Option<Package>,
    /// Package-level severity (only valid when root-level severity is absent).
    pub severity: Vec<Severity>,
    /// Version ranges within which the package is affected.
    pub ranges: Vec<Range>,
    /// Explicit list of affected version strings.
    pub versions: Vec<String>,
}

impl From<osv_db::types::Affected> for Affected {
    fn from(a: osv_db::types::Affected) -> Self {
        Self {
            package: a.package.map(Into::into),
            severity: a.severity.into_iter().map(Into::into).collect(),
            ranges: a.ranges.into_iter().map(Into::into).collect(),
            versions: a.versions,
        }
    }
}
