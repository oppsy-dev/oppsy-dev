use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use reqwest::Url;
use serde::Deserialize;

use crate::Notifier;

#[derive(Debug, Clone)]
pub struct EmailNotifier {
    smtp: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailNotifier {
    pub async fn new(smtp_url: Url) -> anyhow::Result<Self> {
        let host = smtp_url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("Missing SMTP host"))?;
        let port = smtp_url
            .port()
            .ok_or_else(|| anyhow::anyhow!("Missing SMTP port"))?;
        let creds = Credentials::new(
            smtp_url.username().to_string(),
            smtp_url.password().unwrap_or_default().to_string(),
        );
        let smtp = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host)?
            .port(port)
            .credentials(creds)
            .build();
        anyhow::ensure!(
            smtp.test_connection().await?,
            "Cannot connect to SMTP server {smtp_url}"
        );
        Ok(Self { smtp })
    }
}

#[derive(Debug, Clone)]
pub struct EmailEventConfig {
    pub from: Address,
    pub to: Vec<Address>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct EmailEventPayload {
    pub subject: String,
    pub body: String,
}

#[async_trait::async_trait]
impl Notifier for EmailNotifier {
    type EventConf = EmailEventConfig;
    type EventPayload = EmailEventPayload;

    async fn notify(
        &self,
        conf: Self::EventConf,
        payload: Self::EventPayload,
    ) -> anyhow::Result<()> {
        let mut email_builder = Message::builder()
            .from(Mailbox::new(None, conf.from))
            .subject(payload.subject)
            .header(ContentType::TEXT_PLAIN);

        email_builder = conf.to.into_iter().fold(email_builder, |b, to_addr| {
            b.to(Mailbox::new(None, to_addr))
        });

        let email = email_builder.body(payload.body)?;

        self.smtp.send(email).await?;
        Ok(())
    }
}
