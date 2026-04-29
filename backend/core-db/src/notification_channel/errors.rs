use crate::{ConvertError, notification_channel::NotificationChannelId};

#[derive(thiserror::Error, Debug)]
pub enum NotificationChannelFromRowError {
    #[error("Cannot decode notification_channel id column: {0}")]
    CannotDecodeId(sqlx::Error),
    #[error("Cannot decode notification_channel name column: {0}")]
    CannotDecodeName(sqlx::Error),
    #[error("Cannot decode notification_channel type column: {0}")]
    CannotDecodeType(sqlx::Error),
    #[error("Cannot decode notification_channel conf column: {0}")]
    CannotDecodeConf(sqlx::Error),
    #[error("Cannot decode notification_channel active column: {0}")]
    CannotDecodeActive(sqlx::Error),
    #[error("Cannot decode notification_channel notification_count column: {0}")]
    CannotDecodeEventsCount(sqlx::Error),
    #[error("Cannot decode notification_channel workspaces_count column: {0}")]
    CannotDecodeWorkspacesCount(sqlx::Error),
    #[error("Cannot decode notification_channel latest_event_id column: {0}")]
    CannotDecodeLatestEventId(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AddNotificationChannelError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to insert notification channel: {0}")]
    Database(sqlx::Error),
    #[error("Notification channel with id {id} already exists")]
    AlreadyExists { id: NotificationChannelId },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum GetNotificationChannelsError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query workspace notification channels: {0}")]
    Database(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteNotificationChannelError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to delete notification channel: {0}")]
    Database(sqlx::Error),
    #[error("Notification channel with id: {id} not found")]
    NotFound { id: NotificationChannelId },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateNotificationChannelError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to update notification channel: {0}")]
    Database(sqlx::Error),
    #[error("Notification channel with id: {id} not found")]
    NotFound { id: NotificationChannelId },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}
