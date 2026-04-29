pub mod errors;

use sqlx::Row;

use crate::{
    ConvertTo, CoreDb, Pagination,
    notification_channel::NotificationChannelId,
    notification_event::errors::{
        AddNotificationEventError, GetChannelNotificationEventsError, NotificationEventFromRowError,
    },
};

pub type NotificationEventId = uuid::Uuid;
/// Unix epoch seconds (UTC).
pub type NotifiedAt = i64;
pub type Meta = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotificationEvent {
    pub id: NotificationEventId,
    pub channel_id: NotificationChannelId,
    /// `None` means the delivery was successful.
    pub error: Option<String>,
    pub meta: Meta,
}

/// Reads a row in the format `(id, channel_id, error, origin)`.
impl TryFrom<sqlx::sqlite::SqliteRow> for NotificationEvent {
    type Error = NotificationEventFromRowError;

    fn try_from(row: sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        let id = row
            .try_get(0)
            .map_err(NotificationEventFromRowError::CannotDecodeId)?;
        let channel_id = row
            .try_get(1)
            .map_err(NotificationEventFromRowError::CannotDecodeChannelId)?;
        let error = row
            .try_get(2)
            .map_err(NotificationEventFromRowError::CannotDecodeError)?;
        let meta = row
            .try_get(3)
            .map_err(NotificationEventFromRowError::CannotDecodeMeta)?;
        Ok(NotificationEvent {
            id,
            channel_id,
            error,
            meta,
        })
    }
}

impl CoreDb {
    pub async fn add_notification_channel_events(
        &self,
        events: Vec<impl ConvertTo<NotificationEvent>>,
    ) -> Result<(), AddNotificationEventError> {
        if events.is_empty() {
            return Ok(());
        }
        let mut qb = sqlx::QueryBuilder::new(
            "INSERT INTO notification_events (id, channel_id, error, meta) ",
        );
        qb.push("VALUES ");
        let mut separated = qb.separated(", ");
        for event in events {
            let event = event.convert()?;
            separated.push("(");
            separated.push_bind_unseparated(event.id);
            separated.push_bind(event.channel_id);
            separated.push_bind(event.error);
            separated.push_bind(event.meta);
            separated.push_unseparated(")");
        }

        let res = qb.build().execute(&self.pool).await.map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    AddNotificationEventError::AlreadyExists
                } else {
                    AddNotificationEventError::Database(e)
                }
            } else {
                AddNotificationEventError::Database(e)
            }
        })?;

        if res.rows_affected() == 0 {
            return Err(AddNotificationEventError::InvalidAffectedRowsAmount(
                res.rows_affected(),
            ));
        }

        Ok(())
    }

    pub async fn get_notification_channel_events(
        &self,
        channel_id: impl ConvertTo<NotificationChannelId>,
        pagination: impl ConvertTo<Pagination>,
    ) -> Result<Vec<NotificationEvent>, GetChannelNotificationEventsError> {
        let channel_id = channel_id.convert()?;
        let pagination = pagination.convert()?;

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM notification_channels WHERE id = $1)")
                .bind(channel_id)
                .fetch_one(&self.pool)
                .await
                .map_err(GetChannelNotificationEventsError::Database)?;
        if !exists {
            return Err(GetChannelNotificationEventsError::ChannelNotFound { id: channel_id });
        }

        let rows = sqlx::query(
            "SELECT id, channel_id, error, meta \
             FROM notification_events \
             WHERE channel_id = $1 \
             ORDER BY id DESC \
             LIMIT $2 OFFSET $3",
        )
        .bind(channel_id)
        .bind(i64::from(pagination.limit))
        .bind(i64::from(pagination.offset()))
        .fetch_all(&self.pool)
        .await
        .map_err(GetChannelNotificationEventsError::Database)?;

        let events = rows
            .into_iter()
            .map(ConvertTo::convert)
            .collect::<Result<_, _>>()?;
        Ok(events)
    }
}
