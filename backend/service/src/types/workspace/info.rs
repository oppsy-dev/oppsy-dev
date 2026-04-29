use poem_openapi::Object;

use super::id::WorkspaceId;

/// A workspace with its identifier and display name.
#[derive(Object, Debug, Clone)]
pub struct WorkspaceInfo {
    /// Unique workspace identifier.
    pub id: WorkspaceId,
    /// Display name of the workspace.
    pub name: String,
    /// Total number of manifests associated with this workspace.
    pub manifest_count: u32,
    /// Total number of notification channels linked to this workspace.
    pub channel_count: u32,
}

impl TryFrom<core_db::workspace::Workspace> for WorkspaceInfo {
    type Error = <WorkspaceId as TryFrom<uuid::Uuid>>::Error;

    fn try_from(value: core_db::workspace::Workspace) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.try_into()?,
            name: value.name,
            manifest_count: value.manifest_count,
            channel_count: value.channel_count,
        })
    }
}
