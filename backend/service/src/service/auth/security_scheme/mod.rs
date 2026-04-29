//! poem-openapi security schemes.

mod errors;

use errors::AuthError;
use poem::Request;
use poem_openapi::{SecurityScheme, auth::ApiKey};

use crate::{
    resources::ResourceRegistry,
    service::auth::{errors::TokenDecodeError, token::Token},
    settings::Settings,
    types::UserId,
};

/// Session cookie security scheme.
///
/// Reads the `session` cookie set after a successful `OAuth2` sign-in and
/// verifies the Ed25519-signed JWT it contains. On success the [`UserId`]
/// extracted from the claims is made available to the handler.
///
/// The cookie is `HttpOnly; Secure` so it is never accessible from JavaScript
/// and is only transmitted over HTTPS. The browser attaches it automatically
/// to every same-origin request.
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "session",
    key_in = "cookie",
    checker = "verify_session_cookie"
)]
pub struct UserAuth(Token);

impl UserAuth {
    #[allow(dead_code)]
    pub fn user_id(&self) -> &UserId {
        self.0.sub()
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Set-Cookie>
pub fn set_session_cookie(token: &str) -> String {
    if cfg!(feature = "local-dev") {
        format!("session={token}; HttpOnly; SameSite=Lax; Path=/")
    } else {
        format!("session={token}; HttpOnly; SameSite=Lax; Secure; Path=/")
    }
}

#[allow(clippy::unused_async)]
async fn verify_session_cookie(
    _req: &Request,
    api_key: ApiKey,
) -> poem::Result<Token> {
    let settings =
        ResourceRegistry::get::<Settings>().map_err(|e| AuthError::ServiceUnavailable(e.into()))?;
    let token = Token::decode(&api_key.key, &settings.auth.jwt_signing_key).map_err(|e| {
        match e {
            TokenDecodeError::SignatureVerificationFailed(_)
            | TokenDecodeError::InvalidSignatureBytes(_)
            | TokenDecodeError::UnexpectedHeader => AuthError::Forbidden(e.into()),
            _ => AuthError::Unauthorized(e.into()),
        }
    })?;
    Ok(token)
}
