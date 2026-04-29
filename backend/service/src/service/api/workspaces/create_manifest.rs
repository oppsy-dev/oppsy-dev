use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    types::{ManifestId, ManifestType, WorkspaceId},
};

/// Request body for manifest creation.
#[derive(Object)]
pub struct CreateManifestRequest {
    /// The lock file ecosystem that determines which parser is used.
    pub manifest_type: ManifestType,
    /// Human-readable name for this manifest (e.g. the filename or repo path).
    pub name: String,
    /// Optional label for versioning or environment disambiguation.
    pub tag: Option<String>,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## Created
    ///
    /// The manifest slot was reserved. The body contains the server-assigned manifest ID.
    /// Upload the file content via `PUT
    /// /v1/workspaces/{workspace_id}/manifests/{manifest_id}`.
    #[oai(status = 201)]
    Created(Json<ManifestId>),
    /// ## Unprocessable Content
    ///
    /// The workspace ID does not exist or is not assigned to the authenticated user.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    _workspace_id: WorkspaceId,
    req: CreateManifestRequest,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    let manifest_id = ManifestId::generate();
    try_or_return!(
        core_db
            .add_manifest(manifest_id, req.manifest_type, req.name, req.tag)
            .await
    );
    Responses::Created(Json(manifest_id)).into()
}
