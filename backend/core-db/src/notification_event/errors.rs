use crate::{ConvertError, notification_channel::NotificationChannelId};

#[derive(thiserror::Error, Debug)]
pub enum NotificationEventFromRowError {
    #[error("Cannot decode notification_event id column: {0}")]
    CannotDecodeId(sqlx::Error),
    #[error("Cannot decode notification_event channel_id column: {0}")]
    CannotDecodeChannelId(sqlx::Error),
    #[error("Cannot decode notification_event error column: {0}")]
    CannotDecodeError(sqlx::Error),
    #[error("Cannot decode notification_event meta column: {0}")]
    CannotDecodeMeta(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AddNotificationEventError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to insert notification event: {0}")]
    Database(sqlx::Error),
    #[error("One or more notification events already exist")]
    AlreadyExists,
    #[error("Expected to insert at least one row, affected {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum GetChannelNotificationEventsError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query notification events: {0}")]
    Database(sqlx::Error),
    #[error("Notification channel with id {id} not found")]
    ChannelNotFound { id: NotificationChannelId },
}
