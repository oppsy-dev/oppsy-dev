use poem_openapi::{Object, types::Example};
use url::Url;

/// Configuration for a Discord notification channel.
#[derive(Debug, Clone, Object)]
#[oai(example)]
pub struct DiscordChannelConf {
    /// Discord incoming webhook URL.
    pub discord_webhook_url: Url,
}

impl Example for DiscordChannelConf {
    fn example() -> Self {
        #[allow(clippy::unwrap_used)]
        Self {
            discord_webhook_url: Url::parse("https://example.com/webhooks/oppsy").unwrap(),
        }
    }
}

impl From<DiscordChannelConf> for notifier::discord::DiscordEventConf {
    fn from(value: DiscordChannelConf) -> Self {
        Self {
            url: value.discord_webhook_url,
        }
    }
}
