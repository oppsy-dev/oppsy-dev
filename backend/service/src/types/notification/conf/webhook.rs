use poem_openapi::{Object, types::Example};
use url::Url;

/// Configuration for a webhook notification channel.
#[derive(Debug, Clone, Object)]
#[oai(example)]
pub struct WebhookChannelConf {
    /// URL the backend will POST vulnerability scan results to.
    pub webhook_url: Url,
    /// Optional HMAC-SHA256 secret used to sign the payload.
    pub secret: Option<String>,
}

impl Example for WebhookChannelConf {
    fn example() -> Self {
        #[allow(clippy::unwrap_used)]
        Self {
            webhook_url: Url::parse("https://example.com/webhooks/oppsy").unwrap(),
            secret: None,
        }
    }
}

impl From<WebhookChannelConf> for notifier::webhook::WebhookEventConf {
    fn from(value: WebhookChannelConf) -> Self {
        Self {
            url: value.webhook_url,
            secret: value.secret,
        }
    }
}
