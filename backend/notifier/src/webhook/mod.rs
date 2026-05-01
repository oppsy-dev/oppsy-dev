use hmac::{Hmac, Mac};
use reqwest::Url;
use sha2::Sha256;

use crate::Notifier;

pub struct WebhookNotifier;

pub struct WebhookEventConf {
    pub url: Url,
    pub secret: Option<String>,
}

pub type WebhookEventPayload = serde_json::Value;

#[async_trait::async_trait]
impl Notifier for WebhookNotifier {
    type EventConf = WebhookEventConf;
    type EventPayload = WebhookEventPayload;

    async fn notify(
        &self,
        conf: Self::EventConf,
        payload: Self::EventPayload,
    ) -> anyhow::Result<()> {
        let body = serde_json::to_vec(&payload)?;
        let signature = conf.secret.map(|s| sign_payload(&s, &body)).transpose()?;
        let mut request = reqwest::Client::new()
            .post(conf.url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);
        if let Some(sig) = signature {
            request = request.header("X-OSV-Signature", sig);
        }
        let resp = request.send().await?;
        resp.error_for_status()?;
        Ok(())
    }
}

/// Computes `sha256=<hex>` HMAC signature over `body` using `secret`.
fn sign_payload(
    secret: &str,
    body: &[u8],
) -> anyhow::Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
    mac.update(body);
    Ok(format!(
        "sha256={}",
        hex::encode(mac.finalize().into_bytes())
    ))
}
