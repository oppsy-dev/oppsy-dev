use poem_openapi::{Object, types::ToJSON};

use crate::types::{
    ManifestId, ManifestName, ManifestTag, ManifestType, OsvId, WorkspaceId, WorkspaceName,
    parse_from_json,
};

/// Contextual data describing what triggered a notification event.
#[derive(Debug, Clone, Object)]
pub struct NotificationEventMeta {
    /// Identifier of the workspace whose manifest triggered this notification.
    pub workspace_id: WorkspaceId,
    /// Display name of the workspace at the time the notification was dispatched.
    pub workspace_name: WorkspaceName,
    /// Identifier of the manifest that was scanned and produced new vulnerability hits.
    pub manifest_id: ManifestId,
    /// The lock file ecosystem type.
    pub manifest_type: ManifestType,
    /// Human-readable name of the manifest (e.g. the lock file path or repo label).
    pub manifest_name: ManifestName,
    /// Optional tag that was set on the manifest for versioning or environment
    /// disambiguation.
    pub manifest_tag: Option<ManifestTag>,
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
