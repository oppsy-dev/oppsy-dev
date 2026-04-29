pub mod errors;

use sqlx::Row;

use crate::{
    ConvertTo, CoreDb, Pagination,
    notification_channel::errors::{
        AddNotificationChannelError, DeleteNotificationChannelError, GetNotificationChannelsError,
        NotificationChannelFromRowError, UpdateNotificationChannelError,
    },
    notification_event::NotificationEventId,
    workspace::WorkspaceId,
};

pub type NotificationChannelId = uuid::Uuid;
pub type NotificationChannelName = String;
pub type NotificationChannelConf = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotificationChannel {
    pub id: NotificationChannelId,
    pub name: NotificationChannelName,
    pub conf: NotificationChannelConf,
    pub active: bool,
    /// Total number of notification events recorded for this channel.
    pub events_count: u32,
    /// Total number of linked to this channel workspaces.
    pub workspaces_count: u32,
    /// ID of the most recent notification event, or `None` if no events exist yet.
    pub latest_event_id: Option<NotificationEventId>,
}

/// Reads a row in the format `(id, name, conf, active, notification_count,
/// workspaces_count, latest_event_id)`.
impl TryFrom<sqlx::sqlite::SqliteRow> for NotificationChannel {
    type Error = NotificationChannelFromRowError;

    fn try_from(row: sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        let id = row
            .try_get(0)
            .map_err(NotificationChannelFromRowError::CannotDecodeId)?;
        let name = row
            .try_get(1)
            .map_err(NotificationChannelFromRowError::CannotDecodeName)?;
        let conf = row
            .try_get(2)
            .map_err(NotificationChannelFromRowError::CannotDecodeConf)?;
        let active = row
            .try_get(3)
            .map_err(NotificationChannelFromRowError::CannotDecodeActive)?;
        let events_count = row
            .try_get::<i64, _>(4)
            .and_then(|v| u32::try_from(v).map_err(sqlx::Error::decode))
            .map_err(NotificationChannelFromRowError::CannotDecodeEventsCount)?;
        let workspaces_count = row
            .try_get::<i64, _>(5)
            .and_then(|v| u32::try_from(v).map_err(sqlx::Error::decode))
            .map_err(NotificationChannelFromRowError::CannotDecodeEventsCount)?;
        let latest_event_id = row
            .try_get(6)
            .map_err(NotificationChannelFromRowError::CannotDecodeLatestEventId)?;
        Ok(NotificationChannel {
            id,
            name,
            conf,
            active,
            events_count,
            workspaces_count,
            latest_event_id,
        })
    }
}

impl CoreDb {
    pub async fn add_notification_channel(
        &self,
        id: impl ConvertTo<NotificationChannelId>,
        name: impl ConvertTo<NotificationChannelName>,
        conf: impl ConvertTo<NotificationChannelConf>,
        active: impl ConvertTo<bool>,
    ) -> Result<(), AddNotificationChannelError> {
        let id = id.convert()?;
        let name = name.convert()?;
        let conf = conf.convert()?;
        let active = active.convert()?;

        let res = sqlx::query(
            "INSERT INTO notification_channels (id, name, conf, active) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(&name)
        .bind(&conf)
        .bind(active)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    AddNotificationChannelError::AlreadyExists { id }
                } else {
                    AddNotificationChannelError::Database(e)
                }
            } else {
                AddNotificationChannelError::Database(e)
            }
        })?;

        if res.rows_affected() != 1 {
            return Err(AddNotificationChannelError::InvalidAffectedRowsAmount(
                res.rows_affected(),
            ));
        }

        Ok(())
    }

    pub async fn get_notification_channels(
        &self,
        pagination: impl ConvertTo<Pagination>,
    ) -> Result<Vec<NotificationChannel>, GetNotificationChannelsError> {
        let pagination = pagination.convert()?;
        let rows = sqlx::query(
            "SELECT id, name, conf, active, \
             (SELECT COUNT(*) FROM notification_events WHERE notification_events.channel_id = notification_channels.id) AS notification_count, \
             (SELECT COUNT(*) FROM workspace_notification_channels WHERE workspace_notification_channels.channel_id = notification_channels.id) AS workspaces_count, \
             (SELECT id FROM notification_events WHERE notification_events.channel_id = notification_channels.id ORDER BY id DESC LIMIT 1) AS latest_event_id \
             FROM notification_channels \
             ORDER BY id \
             LIMIT $1 OFFSET $2",
        )
        .bind(i64::from(pagination.limit))
        .bind(i64::from(pagination.offset()))
        .fetch_all(&self.pool)
        .await
        .map_err(GetNotificationChannelsError::Database)?;

        let channels = rows
            .into_iter()
            .map(ConvertTo::convert)
            .collect::<Result<_, _>>()?;
        Ok(channels)
    }

    pub async fn update_notification_channel(
        &self,
        id: impl ConvertTo<NotificationChannelId>,
        name: impl ConvertTo<NotificationChannelName>,
        conf: impl ConvertTo<NotificationChannelConf>,
        active: impl ConvertTo<bool>,
    ) -> Result<(), UpdateNotificationChannelError> {
        let id = id.convert()?;
        let name = name.convert()?;
        let conf = conf.convert()?;
        let active = active.convert()?;

        let res = sqlx::query(
            "UPDATE notification_channels SET name = $1, conf = $2, active = $3 WHERE id = $4",
        )
        .bind(&name)
        .bind(&conf)
        .bind(active)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(UpdateNotificationChannelError::Database)?;

        match res.rows_affected() {
            0 => Err(UpdateNotificationChannelError::NotFound { id }),
            1 => Ok(()),
            n => Err(UpdateNotificationChannelError::InvalidAffectedRowsAmount(n)),
        }
    }

    pub async fn delete_notification_channels(
        &self,
        id: impl ConvertTo<WorkspaceId>,
    ) -> Result<(), DeleteNotificationChannelError> {
        let id = id.convert()?;
        let res = sqlx::query("DELETE FROM notification_channels WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(DeleteNotificationChannelError::Database)?;

        match res.rows_affected() {
            0 => Err(DeleteNotificationChannelError::NotFound { id }),
            1 => Ok(()),
            n => Err(DeleteNotificationChannelError::InvalidAffectedRowsAmount(n)),
        }
    }
}
