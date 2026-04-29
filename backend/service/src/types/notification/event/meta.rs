use poem_openapi::{Object, types::ToJSON};

use crate::types::{OsvId, parse_from_json};

/// Contextual data describing what triggered a notification event.
#[derive(Debug, Clone, Object)]
pub struct NotificationEventMeta {
    /// OSV record identifiers for the vulnerabilities included in this notification.
    pub osv_records: Vec<OsvId>,
}

impl TryFrom<NotificationEventMeta> for core_db::notification_event::Meta {
    type Error = anyhow::Error;

    fn try_from(value: NotificationEventMeta) -> Result<Self, Self::Error> {
        value.to_json().ok_or(anyhow::anyhow!(
            "NotificationEventMeta must convert to the JSON value"
        ))
    }
}

impl TryFrom<core_db::notification_event::Meta> for NotificationEventMeta {
    type Error = anyhow::Error;

    fn try_from(value: core_db::notification_event::Meta) -> Result<Self, Self::Error> {
        parse_from_json(value)
    }
}
