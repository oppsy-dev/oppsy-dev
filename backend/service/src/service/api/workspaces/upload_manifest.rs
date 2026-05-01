use bytes::Bytes;
use chrono::Utc;
use core_db::manifest::errors::GetManifestError;
use osv_db::types::OsvRecord;
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
    types::{ManifestId, ManifestType, WorkspaceId},
};

/// Maximum size of a manifest file (1 MB).
pub const MAXIMUM_MANIFEST_SIZE: usize = 1024 * 1024;

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## No Content
    ///
    /// The manifest bytes were stored and an OSV scan was scheduled.
    #[oai(status = 204)]
    NoContent,
    /// ## Unprocessable Content
    ///
    /// The manifest ID does not exist or does not belong to the given workspace.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
    /// ## Content Too Large
    ///
    /// The manifest exceeds the maximum allowed size (1 MB).
    #[oai(status = 413)]
    PayloadTooLarge,
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    workspace_id: WorkspaceId,
    manifest_id: ManifestId,
    manifest: Bytes,
) -> AllResponses {
    let manifest_db = try_or_return!(ResourceRegistry::get::<ManifestDb>());
    let osv_db = try_or_return!(ResourceRegistry::get::<OsvDb>());
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let notifier = try_or_return!(ResourceRegistry::get::<Notifier>());

    let manifest_info = match core_db.get_manifest(manifest_id).await {
        Ok(manifest) => manifest,
        Err(GetManifestError::NotFound { .. }) => {
            return Responses::UnprocessableContent(Json(
                format!("Manifest `{manifest_id}` not found in workspace `{workspace_id}`").into(),
            ))
            .into();
        },
        Err(err) => try_or_return!(Err(err)),
    };

    let manifest_type = try_or_return!(ManifestType::try_from(manifest_info.manifest_type));

    try_or_return!(manifest_db.put(&manifest_id, &manifest));
    // only after successful storing inside the manifest db, assign stored manifest with the
    // workspace
    try_or_return!(
        core_db
            .add_manifest_for_workspace(workspace_id, manifest_id)
            .await
    );

    let records = try_or_return!(scan_manifest(
        &osv_db,
        &manifest_id,
        manifest_type,
        &manifest
    ));

    let detected_at = Utc::now();
    try_or_return!(
        core_db
            .add_manifest_osv_vuln(
                manifest_id,
                records.iter().map(|v| v.id.clone()).collect(),
                detected_at.timestamp()
            )
            .await
    );

    notifier.spawn_osv_events(core_db, workspace_id, manifest_id, records);

    Responses::NoContent.into()
}

fn scan_manifest(
    osv_db: &OsvDb,
    manifest_id: &ManifestId,
    manifest_type: ManifestType,
    manifest: &[u8],
) -> anyhow::Result<Vec<OsvRecord>> {
    let records = osv_db.add_manifest(manifest_type, manifest_id, manifest)?;
    info!(
        manifest_id = %manifest_id,
        found = records.len(),
        "Manifest scanned"
    );
    Ok(records)
}
