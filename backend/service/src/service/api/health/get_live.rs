use poem_openapi::ApiResponse;

use crate::service::common::responses::WithErrorResponses;

/// Liveness probe response.
#[derive(ApiResponse)]
pub enum Response {
    /// ## No Content
    ///
    /// The service process is alive and its HTTP server is responsive.
    ///
    /// A liveness probe confirms that the service process has not deadlocked or
    /// crashed. If this endpoint stops responding, the orchestrator (e.g.
    /// Kubernetes) should restart the container.
    ///
    /// This check is intentionally minimal — it does **not** verify downstream
    /// dependencies such as the OSV database or external services. Use
    /// `/v1/health/ready` for dependency-aware readiness checks.
    #[oai(status = 204)]
    NoContent,
}

/// All responses.
pub type AllResponses = WithErrorResponses<Response>;

pub fn endpoint() -> AllResponses {
    Response::NoContent.into()
}
