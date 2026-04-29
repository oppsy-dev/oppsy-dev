use poem_openapi::{Enum, Object};

/// Supported vulnerability severity scoring systems.
#[derive(Debug, Clone, Enum)]
pub enum SeverityType {
    /// Common Vulnerability Scoring System v2.
    #[oai(rename = "CVSS_V2")]
    CvssV2,
    /// Common Vulnerability Scoring System v3.
    #[oai(rename = "CVSS_V3")]
    CvssV3,
    /// Common Vulnerability Scoring System v4.
    #[oai(rename = "CVSS_V4")]
    CvssV4,
    /// Ubuntu severity levels.
    Ubuntu,
}

impl From<osv_db::types::SeverityType> for SeverityType {
    fn from(t: osv_db::types::SeverityType) -> Self {
        match t {
            osv_db::types::SeverityType::CvssV2 => Self::CvssV2,
            osv_db::types::SeverityType::CvssV3 => Self::CvssV3,
            osv_db::types::SeverityType::CvssV4 => Self::CvssV4,
            osv_db::types::SeverityType::Ubuntu => Self::Ubuntu,
        }
    }
}

/// A severity rating expressed in a specific scoring system.
#[derive(Debug, Clone, Object)]
pub struct Severity {
    /// The scoring system used.
    #[oai(rename = "type")]
    pub severity_type: SeverityType,
    /// Score string whose format is defined by `SeverityType`.
    pub score: String,
}

impl From<osv_db::types::Severity> for Severity {
    fn from(s: osv_db::types::Severity) -> Self {
        Self {
            severity_type: s.severity_type.into(),
            score: s.score,
        }
    }
}
