use poem_openapi::{Enum, Object};

/// Versioning scheme for a range.
#[derive(Debug, Clone, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RangeType {
    /// Git commit hashes.
    Git,
    /// Semantic versioning.
    Semver,
    /// Ecosystem-specific versioning.
    Ecosystem,
}

impl From<osv_db::types::RangeType> for RangeType {
    fn from(t: osv_db::types::RangeType) -> Self {
        match t {
            osv_db::types::RangeType::GIT => Self::Git,
            osv_db::types::RangeType::SEMVER => Self::Semver,
            osv_db::types::RangeType::ECOSYSTEM => Self::Ecosystem,
        }
    }
}

/// A version event bounding an affected range.
/// Exactly one of the four fields is populated.
#[derive(Debug, Clone, Object)]
pub struct Event {
    /// Inclusive version at which the vulnerability was introduced.
    pub introduced: Option<String>,
    /// Exclusive version at which the fix was released.
    pub fixed: Option<String>,
    /// Last inclusive version that is affected.
    pub last_affected: Option<String>,
    /// Exclusive upper bound regardless of other events.
    pub limit: Option<String>,
}

impl From<osv_db::types::Event> for Event {
    fn from(e: osv_db::types::Event) -> Self {
        match e {
            osv_db::types::Event::Introduced { introduced } => {
                Self {
                    introduced: Some(introduced),
                    fixed: None,
                    last_affected: None,
                    limit: None,
                }
            },
            osv_db::types::Event::Fixed { fixed } => {
                Self {
                    introduced: None,
                    fixed: Some(fixed),
                    last_affected: None,
                    limit: None,
                }
            },
            osv_db::types::Event::LastAffected { last_affected } => {
                Self {
                    introduced: None,
                    fixed: None,
                    last_affected: Some(last_affected),
                    limit: None,
                }
            },
            osv_db::types::Event::Limit { limit } => {
                Self {
                    introduced: None,
                    fixed: None,
                    last_affected: None,
                    limit: Some(limit),
                }
            },
        }
    }
}

/// A version range describing when a package is vulnerable.
#[derive(Debug, Clone, Object)]
#[allow(clippy::struct_field_names)]
pub struct Range {
    /// The versioning scheme.
    #[oai(rename = "type")]
    pub range_type: RangeType,
    /// Repository URL — required when `range_type` is `GIT`.
    pub repo: Option<String>,
    /// Ordered list of version events defining the affected range.
    pub events: Vec<Event>,
}

impl From<osv_db::types::Range> for Range {
    fn from(r: osv_db::types::Range) -> Self {
        Self {
            range_type: r.range_type.into(),
            repo: r.repo,
            events: r.events.into_iter().map(Into::into).collect(),
        }
    }
}
