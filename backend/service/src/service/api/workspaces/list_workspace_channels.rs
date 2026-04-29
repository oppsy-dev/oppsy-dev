use core_db::workspace::errors::GetWorkspaceNotificationChannelsError;
use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{error_msg::ErrorMessage, limit::Limit, page::Page, page_info::PageInfo},
    },
    types::{NotificationChannel, WorkspaceId},
};

/// Response body for listing workspace notification channels.
#[derive(Object)]
pub struct WorkspaceChannelList {
    /// Notification channels linked to the workspace.
    pub channels: Vec<NotificationChannel>,
    /// Pagination metadata reflecting the requested page and limit.
    #[oai(flatten)]
    pub page_info: PageInfo,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## OK
    ///
    /// Returns all notification channels linked to the workspace.
    #[oai(status = 200)]
    Ok(Json<WorkspaceChannelList>),

    /// ## Unprocessable Content
    ///
    /// The workspace ID does not exist or is not assigned to the authenticated user.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    workspace_id: WorkspaceId,
    page: Option<Page>,
    limit: Option<Limit>,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let page_info = PageInfo {
        page: page.unwrap_or_default(),
        limit: limit.unwrap_or_default(),
    };

    let db_channels = match core_db
        .get_workspace_notification_channels(workspace_id, page_info)
        .await
    {
        Ok(v) => v,
        Err(GetWorkspaceNotificationChannelsError::NotFound { .. }) => {
            return Responses::UnprocessableContent(Json(
                format!("Workspace `{workspace_id}` not found").into(),
            ))
            .into();
        },
        Err(err) => return try_or_return!(Err(err)),
    };

    let channels = db_channels
        .into_iter()
        .map(TryInto::<NotificationChannel>::try_into)
        .collect::<Result<Vec<_>, _>>();
    let channels = try_or_return!(channels);

    Responses::Ok(Json(WorkspaceChannelList {
        channels,
        page_info,
    }))
    .into()
}
