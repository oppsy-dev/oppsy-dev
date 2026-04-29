//! Public API types shared across the service.

mod email;
mod manifest_id;
mod manifest_info;
mod manifest_type;
mod notification;
mod osv_id;
mod user_id;
mod uuid_v7;
mod workspace;

pub use manifest_id::ManifestId;
pub use manifest_info::{ManifestInfo, ManifestVuln};
pub use manifest_type::ManifestType;
pub use notification::{
    NotificationChannel, NotificationChannelConf, NotificationChannelConfInner,
    NotificationChannelId,
    event::{NotificationEvent, NotificationEventId, NotificationEventMeta},
};
pub use osv_id::OsvId;
pub use workspace::{id::WorkspaceId, info::WorkspaceInfo};

fn parse_from_json<T: poem_openapi::types::ParseFromJSON>(
    json: serde_json::Value
) -> anyhow::Result<T> {
    T::parse_from_json(Some(json)).map_err(|e| anyhow::anyhow!("{}", e.message()))
}
