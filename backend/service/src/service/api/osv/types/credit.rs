use poem_openapi::{Enum, Object};

/// The role a credited party played.
#[derive(Debug, Clone, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreditType {
    /// Discovered the vulnerability.
    Finder,
    /// Reported the vulnerability.
    Reporter,
    /// Analyzed the vulnerability.
    Analyst,
    /// Coordinated the disclosure.
    Coordinator,
    /// Developed the remediation.
    RemediationDeveloper,
    /// Reviewed the remediation.
    RemediationReviewer,
    /// Verified the remediation.
    RemediationVerifier,
    /// A tool used in the process.
    Tool,
    /// Sponsored the work.
    Sponsor,
    /// Any other role.
    Other,
}

impl From<osv_db::types::CreditType> for CreditType {
    fn from(t: osv_db::types::CreditType) -> Self {
        match t {
            osv_db::types::CreditType::FINDER => Self::Finder,
            osv_db::types::CreditType::REPORTER => Self::Reporter,
            osv_db::types::CreditType::ANALYST => Self::Analyst,
            osv_db::types::CreditType::COORDINATOR => Self::Coordinator,
            osv_db::types::CreditType::RemediationDeveloper => Self::RemediationDeveloper,
            osv_db::types::CreditType::RemediationReviewer => Self::RemediationReviewer,
            osv_db::types::CreditType::RemediationVerifier => Self::RemediationVerifier,
            osv_db::types::CreditType::TOOL => Self::Tool,
            osv_db::types::CreditType::SPONSOR => Self::Sponsor,
            osv_db::types::CreditType::OTHER => Self::Other,
        }
    }
}

/// A credit entry for a person or organization.
#[derive(Debug, Clone, Object)]
#[allow(clippy::struct_field_names)]
pub struct Credit {
    /// Name of the credited person or organization.
    pub name: String,
    /// Contact URIs or handles for the credited party.
    pub contact: Vec<String>,
    /// The role this party played.
    #[oai(rename = "type")]
    pub credit_type: Option<CreditType>,
}

impl From<osv_db::types::Credit> for Credit {
    fn from(c: osv_db::types::Credit) -> Self {
        Self {
            name: c.name,
            contact: c.contact,
            credit_type: c.credit_type.map(Into::into),
        }
    }
}
