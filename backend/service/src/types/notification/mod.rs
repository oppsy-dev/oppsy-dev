pub mod channel_id;
pub mod conf;
pub mod event;

use poem_openapi::Object;

pub use self::{
    channel_id::NotificationChannelId,
    conf::{NotificationChannelConf, NotificationChannelConfInner},
    event::NotificationEventId,
};

/// A notification channel belonging to a workspace.
#[derive(Object)]
pub struct NotificationChannel {
    /// Unique identifier of the channel.
    pub id: NotificationChannelId,
    /// Human-readable label.
    pub name: String,
    /// Configuration whose schema is determined by `channel_type`.
    pub conf: NotificationChannelConf,
    /// Whether the channel is active and will receive notifications.
    pub active: bool,
    /// Total number of notification events recorded for this channel.
    pub events_count: u32,
    /// Total number of linked to this channel workspaces.
    pub workspaces_count: u32,
    /// ID of the most recent notification event, or `None` if no events exist yet.
    pub latest_event_id: Option<NotificationEventId>,
}

impl TryFrom<core_db::notification_channel::NotificationChannel> for NotificationChannel {
    type Error = anyhow::Error;

    fn try_from(
        v: core_db::notification_channel::NotificationChannel
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.try_into()?,
            name: v.name,
            conf: v.conf.try_into()?,
            active: v.active,
            events_count: v.events_count,
            workspaces_count: v.workspaces_count,
            latest_event_id: v.latest_event_id.map(TryInto::try_into).transpose()?,
        })
    }
}
