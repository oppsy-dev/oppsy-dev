//! Error types for the authentication domain.

/// The requested provider is not supported or its client failed to initialise
/// at startup (check startup logs for details).
#[derive(Debug, thiserror::Error)]
#[error("Provider not supported or unavailable")]
pub struct ProviderNotSupportedError;

/// No PKCE verifier was found for the given OAuth state.
///
/// This usually means the state parameter is invalid, has already been
/// consumed, or the pending authorisation expired.
#[derive(Debug, thiserror::Error)]
#[error(
    "PKCE verifier not found for the given state — the request may have expired, been replayed, or the state is invalid"
)]
pub struct PkceVerifierNotFoundError;

/// Failures that can occur while decoding and verifying a JWT.
///
/// Variants are ordered to match the verification sequence defined in
/// [RFC 7519 §7.2]: structure checks, signature verification, header
/// validation, then claims deserialisation.
///
/// [RFC 7519 §7.2]: https://datatracker.ietf.org/doc/html/rfc7519#section-7.2
#[derive(Debug, thiserror::Error)]
pub enum TokenDecodeError {
    /// The JWT string does not contain the required three dot-separated
    /// segments (`header.payload.signature`).
    #[error("JWT is missing the {0} segment")]
    MissingSegment(&'static str),

    /// One of the three segments contains characters that are not valid
    /// URL-safe base64 (no-pad).
    #[error("JWT {segment} segment contains invalid base64: {source}")]
    InvalidBase64 {
        segment: &'static str,
        #[source]
        source: base64::DecodeError,
    },

    /// The decoded signature bytes could not be parsed as an Ed25519
    /// signature (must be exactly 64 bytes).
    #[error("JWT signature bytes are not a valid Ed25519 signature: {0}")]
    InvalidSignatureBytes(#[source] ed25519_dalek::SignatureError),

    /// The Ed25519 signature does not match the signing input.
    #[error("JWT signature verification failed: {0}")]
    SignatureVerificationFailed(#[source] ed25519_dalek::SignatureError),

    /// The header segment is not valid JSON.
    #[error("JWT header is not valid JSON: {0}")]
    InvalidHeaderJson(#[source] serde_json::Error),

    /// The header JSON does not match the expected `{{"alg":"Ed25519","typ":"JWT"}}`
    /// value.
    #[error("JWT header does not match the expected algorithm and type")]
    UnexpectedHeader,

    /// The payload segment is not valid JSON or the claims cannot be
    /// deserialised into the expected shape.
    #[error("JWT claims are not valid JSON: {0}")]
    InvalidClaimsJson(#[source] serde_json::Error),
}
