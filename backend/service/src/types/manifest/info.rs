use chrono::{DateTime, Utc};
use poem_openapi::Object;

use super::{name::ManifestName, tag::ManifestTag};
use crate::types::{ManifestId, OsvId};

/// Metadata and detected vulnerabilities for a manifest.
#[derive(Object, Debug, Clone)]
pub struct ManifestInfo {
    /// Unique manifest identifier.
    pub id: ManifestId,
    /// Human-readable name for this manifest.
    pub name: ManifestName,
    /// Optional label for versioning or environment disambiguation.
    pub tag: Option<ManifestTag>,
    /// Vulnerabilities detected when this manifest was last scanned.
    pub vulnerabilities: Vec<ManifestVuln>,
}

/// A detected OSV vulnerability associated with a manifest.
#[derive(Object, Debug, Clone)]
pub struct ManifestVuln {
    /// The OSV vulnerability identifier (e.g. "GHSA-xxxx-xxxx-xxxx").
    pub osv_id: OsvId,
    /// Timestamp when the vulnerability was first detected.
    pub detected_at: DateTime<Utc>,
}
