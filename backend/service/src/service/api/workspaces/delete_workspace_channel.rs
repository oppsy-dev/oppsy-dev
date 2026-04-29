use core_db::workspace::errors::DeleteNotificationChannelForWorkspaceError;
use poem_openapi::{ApiResponse, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    types::{NotificationChannelId, WorkspaceId},
};

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## No Content
    ///
    /// The notification channel was successfully removed from the workspace.
    #[oai(status = 204)]
    NoContent,

    /// ## Unprocessable Content
    ///
    /// The workspace or channel ID does not exist.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    workspace_id: WorkspaceId,
    channel_id: NotificationChannelId,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    match core_db
        .delete_notification_channel_for_workspace(workspace_id, channel_id)
        .await
    {
        Ok(()) => Responses::NoContent.into(),
        Err(DeleteNotificationChannelForWorkspaceError::NotFound { .. }) => {
            Responses::UnprocessableContent(Json(
                format!("Channel `{channel_id}` not found in workspace `{workspace_id}`").into(),
            ))
            .into()
        },
        Err(err) => try_or_return!(Err(err)),
    }
}
