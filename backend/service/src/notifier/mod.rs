mod payload;

use std::{any::type_name, fmt::Debug, sync::Arc};

use futures::FutureExt;
use notifier::{discord::DiscordNotifier, email::EmailNotifier, webhook::WebhookNotifier};
use osv_db::types::OsvRecord;
use tracing::{error, info};

use crate::{
    db::CoreDb,
    notifier::payload::{
        osv_discord_event_payload, osv_email_event_payload, osv_webhook_event_payload,
    },
    resources::{Resource, ResourceRegistry},
    settings::Settings,
    types::{
        ManifestId, NotificationChannel, NotificationChannelConfInner, NotificationChannelId,
        NotificationEvent, NotificationEventId, NotificationEventMeta, WorkspaceId,
    },
};

pub struct Notifier {
    webhook: Arc<WebhookNotifier>,
    discord: Arc<DiscordNotifier>,
    email: Option<Arc<EmailNotifier>>,
}

#[async_trait::async_trait]
impl Resource for Notifier {
    async fn init() -> anyhow::Result<Self> {
        let settings = ResourceRegistry::get::<Settings>()?;
        let email = match settings.smtp_url.clone() {
            Some(url) => Some(Arc::new(EmailNotifier::new(url).await?)),
            None => None,
        };
        Ok(Self {
            webhook: Arc::new(WebhookNotifier),
            discord: Arc::new(DiscordNotifier),
            email,
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
        manifest_id: ManifestId,
        records: Vec<OsvRecord>,
    ) -> anyhow::Result<()> {
        let channels = core_db
            .get_workspace_notification_channels(workspace_id, core_db::Pagination::all())
            .await?
            .into_iter()
            .map(TryInto::<NotificationChannel>::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        let osv_records_ids: Vec<_> = records.iter().map(|r| r.id.clone().into()).collect();
        let manifest_info = core_db.get_manifest(manifest_id).await?;
        let workspace_info = core_db.get_workspace(workspace_id).await?;

        let meta = NotificationEventMeta {
            workspace_id,
            workspace_name: workspace_info.name.into(),
            manifest_id,
            manifest_type: manifest_info.manifest_type.try_into()?,
            manifest_name: manifest_info.name.into(),
            manifest_tag: manifest_info.tag.map(Into::into),
            osv_records: osv_records_ids,
        };

        let events = channels
            .into_iter()
            .filter(|c| c.active)
            .map(|channel| {
                Self::spawn_osv_notification(self.clone(), channel, meta.clone()).boxed()
            })
            .fold(tokio::task::JoinSet::new(), |mut tasks, t| {
                tasks.spawn(t);
                tasks
            })
            .join_all()
            .await
            .into_iter()
            .map(|(channel_id, id, res)| {
                NotificationEvent {
                    id,
                    channel_id,
                    error: res.err().map(|e| e.to_string()),
                    meta: meta.clone(),
                }
            })
            .collect();

        core_db.add_notification_channel_events(events).await?;
        Ok(())
    }

    async fn spawn_osv_notification(
        self: Arc<Notifier>,
        channel: NotificationChannel,
        meta: NotificationEventMeta,
    ) -> (
        NotificationChannelId,
        NotificationEventId,
        anyhow::Result<()>,
    ) {
        match channel.conf.inner {
            NotificationChannelConfInner::Webhook(conf) => {
                let payload = osv_webhook_event_payload(&meta);
                spawn_notification(Some(self.webhook.clone()), conf, channel.id, payload).await
            },
            NotificationChannelConfInner::Discord(conf) => {
                let payload = osv_discord_event_payload(&meta);
                spawn_notification(Some(self.discord.clone()), conf, channel.id, payload).await
            },
            NotificationChannelConfInner::Email(conf) => {
                let payload = osv_email_event_payload(&meta);
                spawn_notification(self.email.clone(), conf, channel.id, payload).await
            },
        }
    }
}

async fn spawn_notification<N, C, P>(
    notifier: Option<Arc<N>>,
    event_conf: C,
    channel_id: NotificationChannelId,
    payload: P,
) -> (
    NotificationChannelId,
    NotificationEventId,
    anyhow::Result<()>,
)
where
    N: notifier::Notifier,
    C: Into<N::EventConf> + Debug,
    P: Into<N::EventPayload> + Debug,
{
    if let Some(notifier) = notifier {
        info!(type = %type_name::<N>(), conf = ?event_conf, "Spawn notification");
        let res = notifier.notify(event_conf.into(), payload.into()).await;
        (channel_id, NotificationEventId::generate(), res)
    } else {
        error!(type = %type_name::<N>(), "Notifier not configured");
        (
            channel_id,
            NotificationEventId::generate(),
            Err(anyhow::anyhow!("Notifier not configured")),
        )
    }
}
