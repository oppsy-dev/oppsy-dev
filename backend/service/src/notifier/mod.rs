use std::{fmt::Debug, sync::Arc};

use futures::FutureExt;
use notifier::{discord::DiscordNotifier, email::EmailNotifier, webhook::WebhookNotifier};
use osv_db::types::OsvRecord;
use tracing::{error, info};

use crate::{
    db::CoreDb,
    resources::Resource,
    types::{
        ManifestId, NotificationChannel, NotificationChannelConfInner, NotificationChannelId,
        NotificationEvent, NotificationEventId, NotificationEventMeta, WorkspaceId,
    },
};

pub struct Notifier {
    webhook: Arc<WebhookNotifier>,
    discord: Arc<DiscordNotifier>,
    email: Arc<EmailNotifier>,
}

#[async_trait::async_trait]
impl Resource for Notifier {
    async fn init() -> anyhow::Result<Self> {
        Ok(Self {
            webhook: Arc::new(WebhookNotifier),
            discord: Arc::new(DiscordNotifier),
            email: Arc::new(EmailNotifier),
        })
    }
}

impl Notifier {
    pub fn spawn_osv_events(
        self: Arc<Notifier>,
        core_db: Arc<CoreDb>,
        workspace_id: WorkspaceId,
        manifest_id: ManifestId,
        records: Vec<OsvRecord>,
    ) {
        if records.is_empty() {
            return;
        }
        info!(
            workspace = %workspace_id,
            manifest = %manifest_id,
            "Spawning OSV events"
        );
        tokio::spawn(async move {
            if let Err(err) =
                Self::spawn_osv_events_inner(self, core_db, workspace_id, manifest_id, records)
                    .await
            {
                error!(error = ?err, "Cannot spawn osv notification events");
            }
        });
    }

    async fn spawn_osv_events_inner(
        self: Arc<Notifier>,
        core_db: Arc<CoreDb>,
        workspace_id: WorkspaceId,
        _manifest_id: ManifestId,
        records: Vec<OsvRecord>,
    ) -> anyhow::Result<()> {
        let channels = core_db
            .get_workspace_notification_channels(workspace_id, core_db::Pagination::all())
            .await?
            .into_iter()
            .map(TryInto::<NotificationChannel>::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        let tasks = channels
            .into_iter()
            .filter(|c| c.active)
            .map(|channel| {
                match channel.conf.inner {
                    NotificationChannelConfInner::Webhook(conf) => {
                        notify_about_osv(self.webhook.clone(), conf, channel.id, records.clone())
                            .boxed()
                    },
                    NotificationChannelConfInner::Discord(conf) => {
                        notify_about_osv(self.discord.clone(), conf, channel.id, records.clone())
                            .boxed()
                    },
                    NotificationChannelConfInner::Email(conf) => {
                        notify_about_osv(self.email.clone(), conf, channel.id, records.clone())
                            .boxed()
                    },
                }
            })
            .fold(tokio::task::JoinSet::new(), |mut tasks, t| {
                tasks.spawn(t);
                tasks
            });

        let events = tasks.join_all().await;
        core_db.add_notification_channel_events(events).await?;
        Ok(())
    }
}

async fn notify_about_osv<N, C>(
    notifier: Arc<N>,
    event_conf: C,
    channel_id: NotificationChannelId,
    records: Vec<OsvRecord>,
) -> NotificationEvent
where
    N: notifier::Notifier,
    C: Into<N::EventConf> + Debug,
{
    tracing::info!(records = records.len(), conf = ?event_conf, "Spawn OSV record notification");
    let osv_records = records.iter().map(|r| r.id.clone().into()).collect();
    let res = notifier.notify(event_conf.into(), records).await;
    let error = res.err().map(|e| e.to_string());
    let id = NotificationEventId::generate();
    NotificationEvent {
        id,
        channel_id,
        error,
        meta: NotificationEventMeta { osv_records },
    }
}
