use reqwest::Url;
use webhook::{client::WebhookClient, models::Message};

use crate::Notifier;

// Discord webhook limitation <https://docs.discord.com/developers/resources/webhook#execute-webhook/>
const MAX_CONTENT_LENGTH: usize = 2000;

pub struct DiscordNotifier;

#[derive(Debug, Clone)]
pub struct DiscordEventConf {
    pub url: Url,
}

#[derive(Debug, Clone)]
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

        let client = WebhookClient::new(conf.url.as_str());
        let mut message = Message::new();
        message.content(&payload.content);
        let res = client
            .send_message(&message)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        anyhow::ensure!(res, "Discord message has not been delivered");
        Ok(())
    }
}
