//! Public API types shared across the service.

mod email;
mod manifest;
mod notification;
mod osv_id;
mod user_id;
mod uuid_v7;
mod workspace;

pub use manifest::{
    id::ManifestId,
    info::{ManifestInfo, ManifestVuln},
    name::ManifestName,
    tag::ManifestTag,
    r#type::ManifestType,
};
pub use notification::{
    NotificationChannel, NotificationChannelConf, NotificationChannelConfInner,
    NotificationChannelId,
    event::{NotificationEvent, NotificationEventId, NotificationEventMeta},
};
pub use osv_id::OsvId;
pub use workspace::{id::WorkspaceId, info::WorkspaceInfo, name::WorkspaceName};

fn parse_from_json<T: poem_openapi::types::ParseFromJSON>(
    json: serde_json::Value
) -> anyhow::Result<T> {
    T::parse_from_json(Some(json)).map_err(|e| anyhow::anyhow!("{}", e.message()))
}
