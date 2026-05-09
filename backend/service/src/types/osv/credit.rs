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

impl From<osv_types::CreditType> for CreditType {
    fn from(t: osv_types::CreditType) -> Self {
        match t {
            osv_types::CreditType::FINDER => Self::Finder,
            osv_types::CreditType::REPORTER => Self::Reporter,
            osv_types::CreditType::ANALYST => Self::Analyst,
            osv_types::CreditType::COORDINATOR => Self::Coordinator,
            osv_types::CreditType::RemediationDeveloper => Self::RemediationDeveloper,
            osv_types::CreditType::RemediationReviewer => Self::RemediationReviewer,
            osv_types::CreditType::RemediationVerifier => Self::RemediationVerifier,
            osv_types::CreditType::TOOL => Self::Tool,
            osv_types::CreditType::SPONSOR => Self::Sponsor,
            osv_types::CreditType::OTHER => Self::Other,
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

impl From<osv_types::Credit> for Credit {
    fn from(c: osv_types::Credit) -> Self {
        Self {
            name: c.name,
            contact: c.contact,
            credit_type: c.credit_type.map(Into::into),
        }
    }
}
