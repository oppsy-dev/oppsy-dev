use discord_webhook2::{message::Message, webhook::DiscordWebhook};
use reqwest::Url;
use serde::Deserialize;

use crate::Notifier;

// Discord webhook limitation <https://docs.discord.com/developers/resources/webhook#execute-webhook/>
const MAX_CONTENT_LENGTH: usize = 2000;

pub struct DiscordNotifier;

#[derive(Debug, Clone)]
pub struct DiscordEventConf {
    pub url: Url,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DiscordEventPayload {
    pub content: String,
}

#[async_trait::async_trait]
impl Notifier for DiscordNotifier {
    type EventConf = DiscordEventConf;
    type EventPayload = DiscordEventPayload;

    async fn notify(
        &self,
        conf: Self::EventConf,
        payload: Self::EventPayload,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            payload.content.len() <= MAX_CONTENT_LENGTH,
            "Discord Event Payload content exceeds {MAX_CONTENT_LENGTH}"
        );
        let discord_webhook = DiscordWebhook::from_url(conf.url);
        let message = Message::new(|m| m.content(payload.content));
        discord_webhook.send(&message).await?;
        Ok(())
    }
}
