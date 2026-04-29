use chrono::{DateTime, Utc};
use poem_openapi::Object;

use crate::types::{ManifestId, ManifestType, OsvId};

/// Metadata and detected vulnerabilities for a manifest.
#[derive(Object, Debug, Clone)]
pub struct ManifestInfo {
    /// Unique manifest identifier.
    pub id: ManifestId,
    /// The lock file ecosystem type.
    pub manifest_type: ManifestType,
    /// Human-readable name for this manifest.
    pub name: String,
    /// Optional label for versioning or environment disambiguation.
    pub tag: Option<String>,
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

impl TryFrom<core_db::manifest_osv_vuln::OsvVuln> for ManifestVuln {
    type Error = anyhow::Error;

    fn try_from(v: core_db::manifest_osv_vuln::OsvVuln) -> Result<Self, Self::Error> {
        Ok(Self {
            osv_id: OsvId::from(v.osv_id),
            detected_at: DateTime::from_timestamp(v.detected_at, 0).ok_or(anyhow::anyhow!(
                "Cannot convert detected_at to DateTime::<Utc>, detected_at: {}",
                v.detected_at
            ))?,
        })
    }
}
