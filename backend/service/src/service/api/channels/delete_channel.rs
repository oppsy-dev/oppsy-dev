use core_db::notification_channel::errors::DeleteNotificationChannelError;
use poem_openapi::{ApiResponse, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    types::NotificationChannelId,
};

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## No Content
    ///
    /// The workspace was deleted successfully.
    #[oai(status = 204)]
    NoContent,

    /// ## Unprocessable Content
    ///
    /// The workspace ID does not exist or is not assigned to the authenticated user.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(channel_id: NotificationChannelId) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    match core_db.delete_notification_channels(channel_id).await {
        Ok(()) => Responses::NoContent.into(),
        Err(DeleteNotificationChannelError::NotFound { .. }) => {
            Responses::UnprocessableContent(Json(
                format!("Notification channel `{channel_id}` not found").into(),
            ))
            .into()
        },
        Err(err) => try_or_return!(Err(err)),
    }
}
