use core_db::workspace::errors::GetWorkspaceError;
use osv_types::OsvRecord;
use poem_openapi::{ApiResponse, payload::Json};
use tracing::info;

use crate::{
    db::{CoreDb, ManifestDb, OsvDb},
    notifier::Notifier,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    types::{Manifest, ManifestId, ManifestPackage, WorkspaceId},
};

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
    workspace_id: WorkspaceId,
    req: Manifest,
) -> AllResponses {
    let manifest_db = try_or_return!(ResourceRegistry::get::<ManifestDb>());
    let osv_db = try_or_return!(ResourceRegistry::get::<OsvDb>());
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let notifier = try_or_return!(ResourceRegistry::get::<Notifier>());

    match core_db.get_workspace(workspace_id).await {
        Ok(_) => {},
        Err(GetWorkspaceError::NotFound { .. }) => {
            return Responses::UnprocessableContent(Json(
                format!("Workspace `{workspace_id}` not found").into(),
            ))
            .into();
        },
        Err(err) => try_or_return!(Err(err)),
    }

    let manifest_id = ManifestId::generate();
    try_or_return!(manifest_db.put(&manifest_id, &req));
    try_or_return!(
        core_db
            .add_manifest(
                manifest_id,
                req.name,
                req.tag.map(String::from),
                serde_json::Value::Null
            )
            .await
    );
    try_or_return!(
        core_db
            .add_manifest_for_workspace(workspace_id, manifest_id)
            .await
    );

    let records = try_or_return!(scan_manifest(&osv_db, &manifest_id, req.packages));
    notifier.spawn_osv_events(core_db, workspace_id, manifest_id, records);

    Responses::Created(Json(manifest_id)).into()
}

fn scan_manifest(
    osv_db: &OsvDb,
    manifest_id: &ManifestId,
    packages: Vec<ManifestPackage>,
) -> anyhow::Result<Vec<OsvRecord>> {
    let records = osv_db.add_manifest(manifest_id, packages)?;
    info!(
        manifest_id = %manifest_id,
        found = records.len(),
        "Manifest scanned"
    );
    Ok(records)
}
