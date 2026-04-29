use core_db::notification_channel::errors::UpdateNotificationChannelError;
use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    types::{NotificationChannelConf, NotificationChannelId},
};

/// Request body for updating a notification channel.
#[derive(Debug, Object)]
pub struct UpdateNotificationChannelRequest {
    /// Human-readable label for this channel.
    pub name: String,
    /// Configuration whose schema is determined by [`NotificationChannelType`].
    pub conf: NotificationChannelConf,
    /// Whether the channel is active and should deliver notifications.
    pub active: bool,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## No Content
    ///
    /// The channel was updated successfully.
    #[oai(status = 204)]
    NoContent,

    /// ## Unprocessable Content
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    channel_id: NotificationChannelId,
    req: UpdateNotificationChannelRequest,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    if !req.conf.verify_type() {
        return Responses::UnprocessableContent(Json(
            "Notification channel configuration must correspond with the type value".into(),
        ))
        .into();
    }

    let conf =
        try_or_return!(core_db::notification_channel::NotificationChannelConf::try_from(req.conf));

    match core_db
        .update_notification_channel(channel_id, req.name, conf, req.active)
        .await
    {
        Ok(()) => Responses::NoContent.into(),
        Err(UpdateNotificationChannelError::NotFound { .. }) => {
            Responses::UnprocessableContent(Json(
                format!("Notification channel `{channel_id}` not found").into(),
            ))
            .into()
        },
        Err(err) => try_or_return!(Err(err)),
    }
}
