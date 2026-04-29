pub mod event_id;
pub mod meta;

use poem_openapi::Object;

pub use self::{event_id::NotificationEventId, meta::NotificationEventMeta};
use crate::types::NotificationChannelId;

/// A recorded attempt to deliver a notification through a channel.
#[derive(Object, Debug, Clone)]
pub struct NotificationEvent {
    /// Unique identifier of this notification event.
    pub id: NotificationEventId,
    /// The channel through which delivery was attempted.
    pub channel_id: NotificationChannelId,
    /// Delivery error message; `None` means the delivery was successful.
    pub error: Option<String>,
    /// Contextual data describing what triggered this notification event.
    pub meta: NotificationEventMeta,
}

impl TryFrom<core_db::notification_event::NotificationEvent> for NotificationEvent {
    type Error = anyhow::Error;

    fn try_from(v: core_db::notification_event::NotificationEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.try_into()?,
            channel_id: v.channel_id.try_into()?,
            error: v.error,
            meta: v.meta.try_into()?,
        })
    }
}

impl TryFrom<NotificationEvent> for core_db::notification_event::NotificationEvent {
    type Error = anyhow::Error;

    fn try_from(v: NotificationEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.into(),
            channel_id: v.channel_id.into(),
            error: v.error,
            meta: v.meta.try_into()?,
        })
    }
}
