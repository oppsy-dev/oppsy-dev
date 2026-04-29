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

/// Request body for creating a notification channel.
#[derive(Debug, Object)]
pub struct CreateNotificationChannelRequest {
    /// Human-readable label for this channel.
    pub name: String,
    /// Configuration whose schema is determined by [`NotificationChannelType`].
    pub conf: NotificationChannelConf,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## Created
    ///
    /// The channel was created. The body contains the server-assigned notification
    /// channel ID.
    #[oai(status = 201)]
    Created(Json<NotificationChannelId>),
    /// ## Unprocessable Content
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(req: CreateNotificationChannelRequest) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    if !req.conf.verify_type() {
        return Responses::UnprocessableContent(Json(
            "Notification channel configuration must correspond with the type value".into(),
        ))
        .into();
    }

    let id = NotificationChannelId::generate();
    try_or_return!(
        core_db
            .add_notification_channel(id, req.name, req.conf, true)
            .await
    );
    Responses::Created(Json(id)).into()
}
