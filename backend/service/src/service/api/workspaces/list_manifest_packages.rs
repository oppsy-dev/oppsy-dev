use poem_openapi::{ApiResponse, Object, payload::Json};
use tracing::{debug, warn};

use crate::{
    db::{CoreDb, ManifestDb, OsvDb},
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{error_msg::ErrorMessage, limit::Limit, page::Page, page_info::PageInfo},
    },
    types::{ManifestId, ManifestPackage, OsvId, WorkspaceId},
};

/// A package parsed from a manifest, augmented with the OSV records that affect it.
#[derive(Object, Debug, Clone)]
pub struct ManifestPackageWithVulns {
    /// Package as it was uploaded by the client.
    #[oai(flatten)]
    pub package: ManifestPackage,
    /// OSV vulnerability identifiers that match this package.
    ///
    /// Empty when no known vulnerability affects the package.
    pub osv_ids: Vec<OsvId>,
}

/// Response body for listing the packages parsed from a manifest.
#[derive(Object)]
pub struct ManifestPackageList {
    /// Packages parsed from the manifest, in the order they were uploaded.
    pub packages: Vec<ManifestPackageWithVulns>,
    /// Total number of packages in the manifest before pagination is applied.
    ///
    /// Reflects the post-filter count when `vulnerable_only=true` so the client
    /// can paginate accurately.
    pub total: u32,
    /// Pagination metadata reflecting the requested page and limit.
    #[oai(flatten)]
    pub page_info: PageInfo,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## OK
    ///
    /// Returns the packages parsed from the manifest, paginated.
    #[oai(status = 200)]
    Ok(Json<ManifestPackageList>),
    /// ## Not Found
    ///
    /// Either the workspace does not exist, the manifest does not belong to it,
    /// or the manifest's stored package list is missing.
    #[oai(status = 404)]
    NotFound,
    /// ## Unprocessable Content
    ///
    /// The pagination math overflows a 32-bit counter.
    #[oai(status = 422)]
    UnprocessableContent(Json<ErrorMessage>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    workspace_id: WorkspaceId,
    manifest_id: ManifestId,
    page: Option<Page>,
    limit: Option<Limit>,
    vulnerable_only: Option<bool>,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let manifest_db = try_or_return!(ResourceRegistry::get::<ManifestDb>());
    let osv_db = try_or_return!(ResourceRegistry::get::<OsvDb>());
    let page_info = PageInfo {
        page: page.unwrap_or_default(),
        limit: limit.unwrap_or_default(),
    };
    let vulnerable_only = vulnerable_only.unwrap_or(false);

    // Workspace ownership: one row each, no full-table scan.
    let in_workspace = try_or_return!(
        core_db
            .is_manifest_in_workspace(workspace_id, manifest_id)
            .await
    );
    if !in_workspace {
        return Responses::NotFound.into();
    }

    // Load the stored manifest (parsed packages) from on-disk storage.
    let Some(manifest) = try_or_return!(manifest_db.get(&manifest_id)) else {
        warn!(
            id = %manifest_id,
            "Manifest is registered in core db but missing from manifest storage",
        );
        return Responses::NotFound.into();
    };
    let package_count = manifest.packages.len();
    debug!(
        manifest_id = %manifest_id,
        package_count,
        vulnerable_only,
        "Resolving OSV ids for manifest packages",
    );

    // Surface the pagination overflow before doing any OSV work — we already know
    // the upper bound on `total` is `manifest.packages.len()`. The `vulnerable_only`
    // path narrows this later but cannot exceed it.
    if u32::try_from(package_count).is_err() {
        return Responses::UnprocessableContent(Json(
            format!("Manifest has more than {} packages", u32::MAX).into(),
        ))
        .into();
    }

    let page_size = usize::try_from(u32::from(page_info.limit)).unwrap_or(usize::MAX);
    let page_index = usize::try_from(u32::from(page_info.page)).unwrap_or(usize::MAX);

    let (page_packages, total_count) = if vulnerable_only {
        // Filtering changes `total`, so we must materialize all matches up front.
        let mut all: Vec<ManifestPackageWithVulns> = try_or_return!(
            manifest
                .packages
                .into_iter()
                .map(|pkg| {
                    let osv_ids = osv_ids_for(&osv_db, &pkg)?;
                    Ok(ManifestPackageWithVulns {
                        package: pkg,
                        osv_ids,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()
        );
        all.retain(|p| !p.osv_ids.is_empty());
        let total = all.len();
        let start = page_index.saturating_mul(page_size).min(total);
        (all.into_iter().skip(start).take(page_size).collect(), total)
    } else {
        // No filter — `total` is known, so only the requested page needs OSV lookups.
        let total = manifest.packages.len();
        let start = page_index.saturating_mul(page_size).min(total);
        let page_packages: Vec<ManifestPackageWithVulns> = try_or_return!(
            manifest
                .packages
                .into_iter()
                .skip(start)
                .take(page_size)
                .map(|pkg| {
                    let osv_ids = osv_ids_for(&osv_db, &pkg)?;
                    Ok(ManifestPackageWithVulns {
                        package: pkg,
                        osv_ids,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()
        );
        (page_packages, total)
    };

    // `total_count` is bounded by `package_count` (which fit in `u32` above), so
    // this cast is infallible.
    let total = u32::try_from(total_count).unwrap_or(u32::MAX);

    Responses::Ok(Json(ManifestPackageList {
        packages: page_packages,
        total,
        page_info,
    }))
    .into()
}

/// Resolves the OSV record IDs that affect `pkg`, mapping them onto the
/// service-facing [`OsvId`] type.
fn osv_ids_for(
    osv_db: &OsvDb,
    pkg: &ManifestPackage,
) -> anyhow::Result<Vec<OsvId>> {
    let osv_records = osv_db.osv_records_for_package(&osv_analyzer::Package::from(pkg))?;
    Ok(osv_records.into_iter().map(OsvId::from).collect())
}
