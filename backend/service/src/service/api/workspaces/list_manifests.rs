use core_db::workspace::errors::GetWorkspaceManifestsError;
use poem_openapi::{ApiResponse, Object, payload::Json};
use tracing::warn;

use crate::{
    db::{CoreDb, ManifestDb, OsvDb},
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{error_msg::ErrorMessage, limit::Limit, page::Page, page_info::PageInfo},
    },
    types::{ManifestId, ManifestInfo, ManifestName, ManifestTag, OsvId, WorkspaceId},
};

/// Response body for listing manifests.
#[derive(Object)]
pub struct ManifestList {
    /// Manifests belonging to the workspace.
    pub manifests: Vec<ManifestInfo>,
    /// Pagination metadata reflecting the requested page and limit.
    #[oai(flatten)]
    pub page_info: PageInfo,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## OK
    ///
    /// Returns the manifests belonging to the workspace with their detected
    /// vulnerabilities.
    #[oai(status = 200)]
    Ok(Json<ManifestList>),
    /// ## Unprocessable Content
    ///
    /// The workspace ID does not exist or is not assigned to the authenticated user.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
    /// ## Not Found
    ///
    /// Manifest id does not exists.
    #[oai(status = 404)]
    NotFound,
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    workspace_id: WorkspaceId,
    page: Option<Page>,
    limit: Option<Limit>,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let manifest_db = try_or_return!(ResourceRegistry::get::<ManifestDb>());
    let osv_db = try_or_return!(ResourceRegistry::get::<OsvDb>());
    let page_info = PageInfo {
        page: page.unwrap_or_default(),
        limit: limit.unwrap_or_default(),
    };

    let db_manifests = match core_db
        .get_workspace_manifests(workspace_id, page_info)
        .await
    {
        Ok(v) => v,
        Err(GetWorkspaceManifestsError::NotFound { .. }) => {
            return Responses::NotFound.into();
        },
        Err(err) => try_or_return!(Err(err)),
    };

    let mut manifests = Vec::with_capacity(db_manifests.len());
    for db_manifest in db_manifests {
        let manifest_id = try_or_return!(ManifestId::try_from(db_manifest.id));
        if try_or_return!(manifest_db.get(&manifest_id)).is_none() {
            warn!(
                id=%manifest_id,
                name=db_manifest.name,
                "Manifest does not exists in the manifest storage, need to upload it first"
            );
            continue;
        }

        let vulnerabilities = osv_db
            .osv_records_for_manifest(&manifest_id)
            .into_iter()
            .map(OsvId::from)
            .collect();

        manifests.push(ManifestInfo {
            id: manifest_id,
            name: ManifestName::from(db_manifest.name),
            tag: db_manifest.tag.map(ManifestTag::from),
            vulnerabilities,
        });
    }

    Responses::Ok(Json(ManifestList {
        manifests,
        page_info,
    }))
    .into()
}
