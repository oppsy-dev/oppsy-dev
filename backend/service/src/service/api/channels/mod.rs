mod create_channel;
mod delete_channel;
mod list_channel_events;
mod list_channels;
mod update_channel;

use poem_openapi::{
    OpenApi,
    param::{Path, Query},
    payload::Json,
};

use crate::{
    service::{
        common::types::{limit::Limit, page::Page},
    },
    types::NotificationChannelId,
};

/// Notification Channels API.
pub struct Api;

#[OpenApi]
impl Api {
    /// Create a new notification channel.
    ///
    /// The server assigns a UUID v7 notification channel ID and returns it in the
    /// response body.
    #[oai(path = "/v1/channels", method = "post")]
    async fn create_channel(
        &self,
        body: Json<create_channel::CreateNotificationChannelRequest>,
    ) -> create_channel::AllResponses {
        create_channel::endpoint(body.0).await
    }

    /// List notification channels.
    #[oai(path = "/v1/channels", method = "get")]
    async fn list_channels(
        &self,
        /// Page number (1-based, default: 1).
        page: Query<Option<Page>>,
        /// Maximum items per page (default: 20).
        limit: Query<Option<Limit>>,
    ) -> list_channels::AllResponses {
        list_channels::endpoint(page.0, limit.0).await
    }

    /// Update a notification channel.
    ///
    /// Replaces the channel's name and configuration.
    #[oai(path = "/v1/channels/:channel_id", method = "patch")]
    async fn update_channel(
        &self,
        /// Notification channel to update.
        channel_id: Path<NotificationChannelId>,
        body: Json<update_channel::UpdateNotificationChannelRequest>,
    ) -> update_channel::AllResponses {
        update_channel::endpoint(channel_id.0, body.0).await
    }

    /// Delete a notification channel.
    ///
    /// Permanently removes the notification channel and all associated data.
    #[oai(path = "/v1/channels/:channel_id", method = "delete")]
    async fn delete_channel(
        &self,
        /// Notification channel to delete.
        notification_id: Path<NotificationChannelId>,
    ) -> delete_channel::AllResponses {
        delete_channel::endpoint(notification_id.0).await
    }

    /// List notification events for a notification channel.
    ///
    /// Returns all notification delivery attempts, ordered from newest to oldest.
    #[oai(path = "/v1/channels/:channel_id/events", method = "get")]
    async fn list_notification_events(
        &self,
        /// Notification channel to list events for.
        notification_id: Path<NotificationChannelId>,
        /// Page number (1-based, default: 1).
        page: Query<Option<Page>>,
        /// Maximum items per page (default: 20).
        limit: Query<Option<Limit>>,
    ) -> list_channel_events::AllResponses {
        list_channel_events::endpoint(notification_id.0, page.0, limit.0).await
    }
}
