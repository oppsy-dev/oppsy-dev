use core_db::workspace::errors::DeleteManifestForWorkspaceError;
use poem_openapi::{ApiResponse, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    types::{ManifestId, WorkspaceId},
};

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## No Content
    ///
    /// The manifest was deleted successfully.
    #[oai(status = 204)]
    NoContent,

    /// ## Unprocessable Content
    ///
    /// The workspace or manifest ID does not exist.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    workspace_id: WorkspaceId,
    manifest_id: ManifestId,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    match core_db
        .delete_manifest_for_workspace(workspace_id, manifest_id)
        .await
    {
        Ok(()) => Responses::NoContent.into(),
        Err(DeleteManifestForWorkspaceError::NotFound { .. }) => {
            Responses::UnprocessableContent(Json(
                format!("Manifest `{manifest_id}` not found in workspace `{workspace_id}`").into(),
            ))
            .into()
        },
        Err(err) => try_or_return!(Err(err)),
    }
}
