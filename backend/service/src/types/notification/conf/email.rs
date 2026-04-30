use poem_openapi::{Object, types::Example};

use crate::types::email::EmailAddress;

/// Configuration for an email notification channel.
#[derive(Debug, Clone, Object)]
#[oai(example)]
pub struct EmailChannelConf {
    /// Sender email address
    pub from: EmailAddress,
    /// Recipient email addresses.
    pub to: Vec<EmailAddress>,
}

impl Example for EmailChannelConf {
    fn example() -> Self {
        Self {
            from: EmailAddress::example(),
            to: vec![EmailAddress::example()],
        }
    }
}

impl From<EmailChannelConf> for notifier::email::EmailEventConfig {
    fn from(value: EmailChannelConf) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into_iter().map(Into::into).collect(),
        }
    }
}
