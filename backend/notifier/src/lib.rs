//! Notification delivery for OSV scan results.
//!
//! Provides typed configuration and delivery for each supported notification
//! channel (webhook, Discord, email) and a [`NotifierKind`] enum that
//! identifies which channel a notifier uses.

pub mod discord;
pub mod email;
pub mod webhook;

#[async_trait::async_trait]
pub trait Notifier {
    type EventConf;

    async fn notify(
        &self,
        conf: Self::EventConf,
        payload: impl serde::Serialize + Send + Sync,
    ) -> anyhow::Result<()>;
}
