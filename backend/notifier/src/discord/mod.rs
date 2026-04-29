use reqwest::Url;

use crate::Notifier;

pub struct DiscordNotifier;

pub struct DiscordEventConf {
    pub url: Url,
}

#[async_trait::async_trait]
impl Notifier for DiscordNotifier {
    type EventConf = DiscordEventConf;

    async fn notify(
        &self,
        conf: Self::EventConf,
        payload: impl serde::Serialize + Send + Sync,
    ) -> anyhow::Result<()> {
        let body = serde_json::to_vec(&payload)?;
        let request = reqwest::Client::new()
            .post(conf.url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);
        let resp = request.send().await?;
        resp.error_for_status()?;
        Ok(())
    }
}
