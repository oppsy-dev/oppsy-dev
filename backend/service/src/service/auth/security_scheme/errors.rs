//! Error types for the security schemes layer.

use poem::{IntoResponse, Response, error::ResponseError};

use crate::service::common::responses::WithErrorResponses;

/// Errors that can occur during bearer token verification.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// The settings resource has not been registered yet.
    #[error("Authentication service is temporarily unavailable")]
    ServiceUnavailable(#[source] anyhow::Error),

    /// The token is missing, structurally invalid, or cannot be deserialized.
    ///
    /// Covers [`TokenDecodeError`] variants that indicate the client sent
    /// something that is not a recognisable JWT.
    #[error("Invalid or malformed authentication token")]
    Unauthorized(#[source] anyhow::Error),

    /// The token is structurally valid but its signature or algorithm header
    /// does not pass verification — indicates a tampered or deliberately
    /// crafted token etc.
    #[error("Authentication token is not accepted")]
    Forbidden(#[source] anyhow::Error),
}

impl ResponseError for AuthError {
    fn status(&self) -> reqwest::StatusCode {
        match self {
            Self::ServiceUnavailable(_) => reqwest::StatusCode::SERVICE_UNAVAILABLE,
            Self::Unauthorized(_) => reqwest::StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => reqwest::StatusCode::FORBIDDEN,
        }
    }

    fn as_response(&self) -> Response
    where Self: std::error::Error + Send + Sync + 'static {
        match self {
            Self::ServiceUnavailable(err) => {
                WithErrorResponses::<()>::service_unavailable(
                    "Authentication is temporarily unavailable",
                    err,
                )
                .into_response()
            },
            Self::Unauthorized(err) => {
                WithErrorResponses::<()>::unauthorized(err.to_string()).into_response()
            },
            Self::Forbidden(err) => {
                WithErrorResponses::<()>::forbidden(err.to_string()).into_response()
            },
        }
    }
}
