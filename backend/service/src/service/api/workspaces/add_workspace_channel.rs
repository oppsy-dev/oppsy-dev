use core_db::workspace::errors::AddNotificationChannelForWorkspaceError;
use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    types::{NotificationChannelId, WorkspaceId},
};

/// Request body for linking a notification channel to a workspace.
#[derive(Object)]
pub struct AddWorkspaceChannelRequest {
    /// ID of the notification channel to link.
    pub channel_id: NotificationChannelId,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## No Content
    ///
    /// The notification channel was successfully linked to the workspace.
    #[oai(status = 204)]
    NoContent,

    /// ## Unprocessable Content
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    workspace_id: WorkspaceId,
    req: AddWorkspaceChannelRequest,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let channel_id = req.channel_id;

    match core_db
        .add_notification_channel_for_workspace(workspace_id, channel_id)
        .await
    {
        Ok(()) => Responses::NoContent.into(),
        Err(AddNotificationChannelForWorkspaceError::AlreadyExists { .. }) => {
            Responses::UnprocessableContent(Json(
                format!("Channel `{channel_id}` is already linked to workspace `{workspace_id}`")
                    .into(),
            ))
            .into()
        },
        Err(err) => try_or_return!(Err(err)),
    }
}
