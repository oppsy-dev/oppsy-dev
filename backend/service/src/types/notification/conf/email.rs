use poem_openapi::{Object, types::Example};

use crate::types::email::EmailAddress;

/// Configuration for an email notification channel.
#[derive(Debug, Clone, Object)]
#[oai(example)]
pub struct EmailChannelConf {
    /// Recipient email addresses.
    pub to_addresses: Vec<EmailAddress>,
}

impl Example for EmailChannelConf {
    fn example() -> Self {
        Self {
            to_addresses: vec![EmailAddress::example()],
        }
    }
}

impl From<EmailChannelConf> for notifier::email::EmailEventConfig {
    fn from(value: EmailChannelConf) -> Self {
        Self {
            to_addresses: value.to_addresses.into_iter().map(Into::into).collect(),
        }
    }
}
