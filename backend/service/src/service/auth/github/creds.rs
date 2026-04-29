//! GitHub `OAuth2` client credentials type.

use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use serde::Deserialize;

/// GitHub `OAuth2` client credentials.
///
/// Deserializes from a credentials JSON file in the following format:
///
/// ```json
/// {
///   "client_id": "...",
///   "client_secret": "...",
///   "auth_uri": "https://github.com/login/oauth/authorize",
///   "token_uri": "https://github.com/login/oauth/access_token",
///   "redirect_uri": "https://example.com/auth/callback"
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    /// `OAuth2` client ID issued by GitHub during app registration.
    pub client_id: ClientId,
    /// `OAuth2` client secret issued by GitHub during app registration.
    pub client_secret: ClientSecret,
    /// Provider's `OAuth2` authorization endpoint.
    pub auth_uri: AuthUrl,
    /// Provider's `OAuth2` token exchange endpoint.
    pub token_uri: TokenUrl,
    /// Redirect URI registered with GitHub.
    pub redirect_uri: RedirectUrl,
}

#[cfg(test)]
mod tests {
    use super::Credentials;

    #[test]
    fn deserializes_github_credentials_json() {
        let json = serde_json::json!({
            "client_id": "test-client-id",
            "client_secret": "test-client-secret",
            "auth_uri": "https://github.com/login/oauth/authorize",
            "token_uri": "https://github.com/login/oauth/access_token",
            "redirect_uri": "https://example.com/auth/callback"
        });

        let creds: Credentials = serde_json::from_value(json).expect("valid credentials JSON");

        assert_eq!(creds.client_id.as_str(), "test-client-id");
        assert_eq!(creds.client_secret.secret(), "test-client-secret");
        assert_eq!(
            creds.auth_uri.as_str(),
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            creds.token_uri.as_str(),
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(
            creds.redirect_uri.as_str(),
            "https://example.com/auth/callback"
        );
    }
}
