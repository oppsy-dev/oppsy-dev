use std::collections::HashSet;

use bytes::Bytes;
use poem_openapi::{
    ApiResponse,
    payload::{Binary, Json},
};

use crate::{
    db::{CoreDb, ManifestDb},
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
    /// ## OK
    ///
    /// The raw lock file bytes.
    #[oai(status = 200)]
    Ok(Binary<Bytes>),
    /// ## Not Found
    ///
    /// The manifest ID does not exist or does not belong to the given workspace.
    #[oai(status = 404)]
    NotFound,
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
    manifest_id: ManifestId,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let manifest_db = try_or_return!(ResourceRegistry::get::<ManifestDb>());

    let workspace_manifests = try_or_return!(
        core_db
            .get_workspace_manifests(workspace_id, core_db::Pagination::all())
            .await
    );
    let workspace_manifest_ids: HashSet<ManifestId> = try_or_return!(
        workspace_manifests
            .into_iter()
            .map(|m| ManifestId::try_from(m.id))
            .collect::<Result<HashSet<_>, _>>()
    );
    if !workspace_manifest_ids.contains(&manifest_id) {
        return Responses::NotFound.into();
    }

    match try_or_return!(manifest_db.get(&manifest_id)) {
        Some(bytes) => Responses::Ok(Binary(bytes)).into(),
        None => Responses::NotFound.into(),
    }
}
