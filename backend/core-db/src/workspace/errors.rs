use crate::{
    ConvertError,
    notification_channel::NotificationChannelId,
    workspace::{ManifestId, WorkspaceId},
};

#[derive(thiserror::Error, Debug)]
pub enum AddNewWorkspaceError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query schema revisions: {0}")]
    Database(sqlx::Error),
    #[error("Entry with such id: {id} already exists")]
    AlreadyExists { id: WorkspaceId },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum WorkspaceFromRowError {
    #[error("Cannot decode workspace id column: {0}")]
    CannotDecodeId(sqlx::Error),
    #[error("Cannot decode workspace name column: {0}")]
    CannotDecodeName(sqlx::Error),
    #[error("Cannot decode workspace manifest_count column: {0}")]
    CannotDecodeManifestCount(sqlx::Error),
    #[error("Cannot decode workspace channel_count column: {0}")]
    CannotDecodeChannelCount(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum GetWorkspacesError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query user workspaces: {0}")]
    Database(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AddManifestForWorkspaceError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query schema revisions: {0}")]
    Database(sqlx::Error),
    #[error(
        "Entry with such workspace id: {workspace_id} and manifest id: {manifest_id} already exists"
    )]
    AlreadyExists {
        workspace_id: WorkspaceId,
        manifest_id: ManifestId,
    },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum GetWorkspaceManifestsError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query workspace manifests: {0}")]
    Database(sqlx::Error),
    #[error("Workspace with id: {id} not found")]
    NotFound { id: WorkspaceId },
}

#[derive(thiserror::Error, Debug)]
pub enum GetManifestWorkspaceMapError {
    #[error("Failed to query manifest workspace map: {0}")]
    Database(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum GetManifestTypeError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query manifest type: {0}")]
    Database(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteManifestForWorkspaceError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to delete manifest from workspace: {0}")]
    Database(sqlx::Error),
    #[error("Manifest `{manifest_id}` not found in workspace `{workspace_id}`")]
    NotFound {
        workspace_id: WorkspaceId,
        manifest_id: ManifestId,
    },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteWorkspaceError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to delete workspace: {0}")]
    Database(sqlx::Error),
    #[error("Workspace with id: {id} not found")]
    NotFound { id: WorkspaceId },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum AddNotificationChannelForWorkspaceError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to add notification channel for workspace: {0}")]
    Database(sqlx::Error),
    #[error(
        "Entry with workspace id: {workspace_id} and notification channel id: \
         {notification_channel_id} already exists"
    )]
    AlreadyExists {
        workspace_id: WorkspaceId,
        notification_channel_id: NotificationChannelId,
    },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum GetWorkspaceNotificationChannelsError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query workspace notification channels: {0}")]
    Database(sqlx::Error),
    #[error("Workspace with id: {id} not found")]
    NotFound { id: WorkspaceId },
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteNotificationChannelForWorkspaceError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to delete notification channel from workspace: {0}")]
    Database(sqlx::Error),
    #[error(
        "Notification channel `{notification_channel_id}` not found in workspace `{workspace_id}`"
    )]
    NotFound {
        workspace_id: WorkspaceId,
        notification_channel_id: NotificationChannelId,
    },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}
