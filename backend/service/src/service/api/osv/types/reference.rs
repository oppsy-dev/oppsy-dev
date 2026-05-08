use poem_openapi::{Enum, Object};

/// Classification of an external reference.
#[derive(Debug, Clone, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReferenceType {
    /// A published security advisory.
    Advisory,
    /// An article or blog post.
    Article,
    /// A tool or method for detecting the vulnerability.
    Detection,
    /// A discussion thread.
    Discussion,
    /// The original vulnerability report.
    Report,
    /// A patch or commit that fixes the vulnerability.
    Fix,
    /// A patch or commit that introduced the vulnerability.
    Introduced,
    /// A git commit or tag.
    Git,
    /// The package in a registry.
    Package,
    /// Evidence supporting the existence of the vulnerability.
    Evidence,
    /// Any other web resource.
    Web,
}

impl From<osv_types::ReferenceType> for ReferenceType {
    fn from(t: osv_types::ReferenceType) -> Self {
        match t {
            osv_types::ReferenceType::ADVISORY => Self::Advisory,
            osv_types::ReferenceType::ARTICLE => Self::Article,
            osv_types::ReferenceType::DETECTION => Self::Detection,
            osv_types::ReferenceType::DISCUSSION => Self::Discussion,
            osv_types::ReferenceType::REPORT => Self::Report,
            osv_types::ReferenceType::FIX => Self::Fix,
            osv_types::ReferenceType::INTRODUCED => Self::Introduced,
            osv_types::ReferenceType::GIT => Self::Git,
            osv_types::ReferenceType::PACKAGE => Self::Package,
            osv_types::ReferenceType::EVIDENCE => Self::Evidence,
            osv_types::ReferenceType::WEB => Self::Web,
        }
    }
}

/// An external reference for the vulnerability.
#[derive(Debug, Clone, Object)]
pub struct Reference {
    /// Classification of this reference.
    #[oai(rename = "type")]
    pub reference_type: ReferenceType,
    /// URI of the reference.
    pub url: String,
}

impl From<osv_types::Reference> for Reference {
    fn from(r: osv_types::Reference) -> Self {
        Self {
            reference_type: r.reference_type.into(),
            url: r.url,
        }
    }
}
