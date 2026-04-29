use poem_openapi::{Object, types::Example};

/// Configuration for an email notification channel.
#[derive(Debug, Clone, Object)]
#[oai(example)]
pub struct EmailChannelConf {
    /// Recipient email addresses.
    pub to_addresses: Vec<String>,
}

impl Example for EmailChannelConf {
    fn example() -> Self {
        Self {
            to_addresses: vec!["john_doe@mail.com".to_string()],
        }
    }
}

impl From<EmailChannelConf> for notifier::email::EmailEventConfig {
    fn from(value: EmailChannelConf) -> Self {
        Self {
            to_addresses: value.to_addresses,
        }
    }
}
