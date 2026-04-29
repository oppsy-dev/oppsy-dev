use chrono::{DateTime, Utc};
use poem_openapi::Object;

use super::{Affected, Credit, Reference, Severity};

/// Root OSV vulnerability record.
///
/// See <https://ossf.github.io/osv-schema/> for the full specification.
#[derive(Debug, Clone, Object)]
pub struct OsvRecord {
    /// Unique vulnerability identifier (e.g. `RUSTSEC-2024-0001`).
    pub id: String,
    /// ISO 8601 timestamp of the last modification.
    pub modified: DateTime<Utc>,
    /// Schema version string.
    pub schema_version: Option<String>,
    /// ISO 8601 timestamp when the record was first published.
    pub published: Option<DateTime<Utc>>,
    /// ISO 8601 timestamp when the record was withdrawn, if applicable.
    pub withdrawn: Option<DateTime<Utc>>,
    /// Alternative identifiers (e.g. CVE IDs).
    pub aliases: Vec<String>,
    /// Related vulnerability IDs that are not direct aliases.
    pub related: Vec<String>,
    /// Upstream vulnerability references.
    pub upstream: Vec<String>,
    /// Brief, one-line description of the vulnerability.
    pub summary: Option<String>,
    /// Full description of the vulnerability (may use Markdown).
    pub details: Option<String>,
    /// Severity ratings at the root level.
    pub severity: Vec<Severity>,
    /// Packages and version ranges affected by this vulnerability.
    pub affected: Vec<Affected>,
    /// External references.
    pub references: Vec<Reference>,
    /// Credits for people or organizations involved in the report.
    pub credits: Vec<Credit>,
}

impl From<osv_db::types::OsvRecord> for OsvRecord {
    fn from(r: osv_db::types::OsvRecord) -> Self {
        Self {
            id: r.id,
            modified: r.modified,
            schema_version: r.schema_version,
            published: r.published,
            withdrawn: r.withdrawn,
            aliases: r.aliases,
            related: r.related,
            upstream: r.upstream,
            summary: r.summary,
            details: r.details,
            severity: r.severity.into_iter().map(Into::into).collect(),
            affected: r.affected.into_iter().map(Into::into).collect(),
            references: r.references.into_iter().map(Into::into).collect(),
            credits: r.credits.into_iter().map(Into::into).collect(),
        }
    }
}
