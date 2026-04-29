use std::fmt::Display;

/// A validated email address used to identify a user account.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl From<String> for EmailAddress {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for EmailAddress {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<EmailAddress> for core_db::user::EmailAddress {
    fn from(value: EmailAddress) -> Self {
        value.0
    }
}
