use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::responses::{WithErrorResponses, try_or_return},
    types::{WorkspaceId, WorkspaceInfo, WorkspaceName},
};

/// Request body for workspace creation.
#[derive(Object)]
pub struct CreateWorkspaceRequest {
    /// Workspace name
    pub name: WorkspaceName,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## OK
    ///
    /// The workspace was created. The body contains the server-assigned workspace ID.
    #[oai(status = 200)]
    Ok(Json<WorkspaceInfo>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(req: CreateWorkspaceRequest) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    let id = WorkspaceId::generate();
    try_or_return!(
        core_db
            .add_new_workspace(id, String::from(req.name.clone()))
            .await
    );
    Responses::Ok(Json(WorkspaceInfo {
        id,
        name: req.name,
        manifest_count: 0,
        channel_count: 0,
    }))
    .into()
}
