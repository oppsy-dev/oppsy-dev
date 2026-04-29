use poem_openapi::ApiResponse;

use crate::{
    db::{CoreDb, ManifestDb, OsvDb},
    resources::ResourceRegistry,
    service::common::responses::{WithErrorResponses, try_or_return},
};

/// Readiness probe response.
#[derive(ApiResponse)]
pub enum Response {
    /// ## No Content
    ///
    /// The service is initialised and ready to serve application traffic.
    ///
    /// A readiness probe confirms that all mandatory dependencies required to
    /// handle requests are available. When this endpoint returns a non-2xx
    /// status, the orchestrator (e.g. Kubernetes) stops routing new traffic to
    /// this instance until the probe recovers.
    ///
    /// Readiness failing does **not** cause a container restart — use
    /// `/v1/health/live` for that purpose.
    #[oai(status = 204)]
    NoContent,
}

/// All responses.
pub type AllResponses = WithErrorResponses<Response>;

pub fn endpoint() -> AllResponses {
    try_or_return!(ResourceRegistry::get::<ManifestDb>());
    try_or_return!(ResourceRegistry::get::<OsvDb>());
    try_or_return!(ResourceRegistry::get::<CoreDb>());

    Response::NoContent.into()
}
