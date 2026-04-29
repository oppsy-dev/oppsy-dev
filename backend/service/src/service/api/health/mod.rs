mod get_live;
mod get_ready;

use poem_openapi::OpenApi;

/// Health API — Kubernetes-compatible liveness and readiness probes.
///
/// These endpoints follow the probe model recommended by Kubernetes:
///
/// | Probe    | Path                   | Purpose                                            |
/// |----------|------------------------|----------------------------------------------------|
/// | Liveness | `GET /v1/health/live`  | Is the process alive? Restart on failure.          |
/// | Readiness| `GET /v1/health/ready` | Ready to serve traffic? Remove from LB on failure. |
pub struct Api;

#[OpenApi]
impl Api {
    /// Liveness probe — is the service process alive?
    ///
    /// Returns `204 No Content` while the HTTP server is responsive.
    /// This is a minimal check: it does **not** test downstream dependencies.
    ///
    /// Intended for use as a Kubernetes `livenessProbe`. A non-2xx response
    /// signals the orchestrator to restart the container.
    #[oai(path = "/v1/health/live", method = "get")]
    #[allow(clippy::unused_async)]
    async fn get_live(&self) -> get_live::AllResponses {
        get_live::endpoint()
    }

    /// Readiness probe — is the service ready to serve application traffic?
    ///
    /// Returns `204 No Content` when all mandatory dependencies are available
    /// and the service can handle requests correctly. Specifically, verifies
    /// that the manifest database and the OSV vulnerability database are both
    /// accessible.
    ///
    /// Intended for use as a Kubernetes `readinessProbe`. A non-2xx response
    /// causes the orchestrator to withhold traffic from this instance without
    /// restarting it.
    #[oai(path = "/v1/health/ready", method = "get")]
    #[allow(clippy::unused_async)]
    async fn get_ready(&self) -> get_ready::AllResponses {
        get_ready::endpoint()
    }
}
