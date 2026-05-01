pub mod discord;
pub mod email;
pub mod r#type;
pub mod webhook;

use poem_openapi::{
    Object, Union,
    types::{Example, ToJSON},
};

pub use self::{
    discord::DiscordChannelConf, email::EmailChannelConf, r#type::NotificationChannelType,
    webhook::WebhookChannelConf,
};
use crate::types::{NotificationEventMeta, parse_from_json};

/// Channel delivery configuration combining a type discriminant with provider-specific
/// settings.
#[derive(Debug, Clone, Object)]
pub struct NotificationChannelConf {
    /// Channel kind: `email`, `webhook`, `slack`, `discord`, `telegram`, etc.
    #[oai(rename = "type")]
    pub channel_type: NotificationChannelType,
    #[oai(flatten)]
    pub inner: NotificationChannelConfInner,
}

/// Provider-specific configuration payload for a notification channel.
#[derive(Debug, Clone, Union)]
pub enum NotificationChannelConfInner {
    Email(EmailChannelConf),
    Discord(DiscordChannelConf),
    Webhook(WebhookChannelConf),
}

impl NotificationChannelConf {
    /// Returns `true` if `channel_type` is consistent with the active `inner` variant.
    pub fn verify(&self) -> anyhow::Result<()> {
        let cue_ctx = cue_rs::Ctx::new()?;
        const INVALID_TYPE_MSG: &str =
            "Notification channel configuration must correspond with the type value";
        let example_meta = NotificationEventMeta::example();
        match &self.inner {
            NotificationChannelConfInner::Webhook(conf) => {
                anyhow::ensure!(
                    self.channel_type == NotificationChannelType::Webhook,
                    INVALID_TYPE_MSG
                );
                conf.event_payload(&cue_ctx, &example_meta)?;
            },
            NotificationChannelConfInner::Discord(conf) => {
                anyhow::ensure!(
                    self.channel_type == NotificationChannelType::Discord,
                    INVALID_TYPE_MSG
                );
                conf.event_payload(&cue_ctx, &example_meta)?;
            },
            NotificationChannelConfInner::Email(conf) => {
                anyhow::ensure!(
                    self.channel_type == NotificationChannelType::Email,
                    INVALID_TYPE_MSG
                );
                conf.event_payload(&cue_ctx, &example_meta)?;
            },
        }
        Ok(())
    }
}

impl Example for NotificationChannelConf {
    fn example() -> Self {
        Self {
            channel_type: NotificationChannelType::Discord,
            inner: NotificationChannelConfInner::Discord(DiscordChannelConf::example()),
        }
    }
}

impl TryFrom<NotificationChannelConf> for core_db::notification_channel::NotificationChannelConf {
    type Error = anyhow::Error;

    fn try_from(value: NotificationChannelConf) -> Result<Self, Self::Error> {
        value.verify()?;
        value.to_json().ok_or(anyhow::anyhow!(
            "NotificationChannelConf must convert to the JSON value"
        ))
    }
}

impl TryFrom<core_db::notification_channel::NotificationChannelConf> for NotificationChannelConf {
    type Error = anyhow::Error;

    fn try_from(
        value: core_db::notification_channel::NotificationChannelConf
    ) -> Result<Self, Self::Error> {
        let res: Self = parse_from_json(value)?;
        res.verify()?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use poem_openapi::types::ParseFromJSON;
    use serde_json::json;
    use test_case::test_case;

    use super::*;

    #[test_case(
        json!({"type": "Email", "from": "bob@example.com", "to": ["alice@example.com"]}),
        NotificationChannelType::Email;
        "email single recipient"
    )]
    #[test_case(
        json!({"type": "Discord", "discord_webhook_url": "https://discord.com/api/webhooks/123/abc"}),
        NotificationChannelType::Discord;
        "discord"
    )]
    #[test_case(
        json!({"type": "Webhook", "webhook_url": "https://example.com/hook", "secret": "s3cr3t"}),
        NotificationChannelType::Webhook;
        "webhook with secret"
    )]
    #[test_case(
        json!({"type": "Webhook", "webhook_url": "https://example.com/hook", "secret": null}),
        NotificationChannelType::Webhook;
        "webhook with null secret"
    )]
    fn parse_from_json_selects_correct_variant(
        input: serde_json::Value,
        expected: NotificationChannelType,
    ) {
        let conf = NotificationChannelConf::parse_from_json(Some(input))
            .expect("should parse successfully");
        assert_eq!(conf.channel_type, expected);
    }
}
