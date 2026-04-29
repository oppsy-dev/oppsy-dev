//! List workspaces the authenticated user belongs to.

use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{limit::Limit, page::Page, page_info::PageInfo},
    },
    types::WorkspaceInfo,
};

/// Response body for listing workspaces.
#[derive(Object)]
pub struct Workspaces {
    /// Workspaces the authenticated user belongs to.
    pub workspaces: Vec<WorkspaceInfo>,
    /// Pagination metadata reflecting the requested page and limit.
    #[oai(flatten)]
    pub page_info: PageInfo,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## OK
    ///
    /// Returns the list of workspaces.
    #[oai(status = 200)]
    Ok(Json<Workspaces>),

    /// ## Not Found
    ///
    /// The team ID does not exist, is not assigned to the authenticated user.
    #[oai(status = 404)]
    NotFound,
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    page: Option<Page>,
    limit: Option<Limit>,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());
    let page_info = PageInfo {
        page: page.unwrap_or_default(),
        limit: limit.unwrap_or_default(),
    };
    let workspaces = try_or_return!(core_db.get_workspaces(page_info).await);
    let workspaces = try_or_return!(
        workspaces
            .into_iter()
            .map(TryInto::<WorkspaceInfo>::try_into)
            .collect::<Result<Vec<_>, _>>()
    );

    Responses::Ok(Json(Workspaces {
        workspaces,
        page_info,
    }))
    .into()
}
