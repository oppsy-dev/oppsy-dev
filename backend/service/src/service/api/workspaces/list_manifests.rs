use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{error_msg::ErrorMessage, limit::Limit, page::Page, page_info::PageInfo},
    },
    types::{
        ManifestId, ManifestInfo, ManifestName, ManifestTag, ManifestType, ManifestVuln,
        WorkspaceId,
    },
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

    let db_manifests = try_or_return!(
        core_db
            .get_workspace_manifests(workspace_id, page_info)
            .await
    );

    let mut manifests = Vec::with_capacity(db_manifests.len());
    for db_manifest in db_manifests {
        let manifest_id = try_or_return!(ManifestId::try_from(db_manifest.id));
        let manifest_type = try_or_return!(ManifestType::try_from(db_manifest.manifest_type));
        let vulns = try_or_return!(core_db.get_manifest_osv_vulns(manifest_id).await);
        let vulnerabilities =
            try_or_return!(vulns.into_iter().map(ManifestVuln::try_from).collect());
        manifests.push(ManifestInfo {
            id: manifest_id,
            manifest_type,
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
