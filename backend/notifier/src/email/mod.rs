use crate::Notifier;

pub struct EmailNotifier;

pub struct EmailEventConfig {
    pub to_addresses: Vec<String>,
}

#[async_trait::async_trait]
impl Notifier for EmailNotifier {
    type EventConf = EmailEventConfig;

    async fn notify(
        &self,
        _conf: Self::EventConf,
        payload: impl serde::Serialize + Send + Sync,
    ) -> anyhow::Result<()> {
        let _body = serde_json::to_vec(&payload)?;
        anyhow::bail!("Email notifier has not been implemented");
    }
}
