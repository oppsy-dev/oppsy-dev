//! Google `OAuth2` client credentials type.

use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use serde::Deserialize;

/// Google `OAuth2` client credentials.
///
/// Deserializes from the credentials JSON that Google issues when you create
/// an `OAuth2` client in the Google Cloud Console:
///
/// ```json
/// {
///   "web": {
///     "client_id": "...",
///     "client_secret": "...",
///     "auth_uri": "https://accounts.google.com/o/oauth2/auth",
///     "token_uri": "https://accounts.google.com/o/oauth2/token",
///     "redirect_uris": ["https://example.com/oauth2callback"]
///   }
/// }
/// ```
///
/// The `web` key is a deserialization detail — the fields are exposed directly
/// on this type. `redirect_uris` is parsed for format compatibility; the
/// service derives its own redirect URI from `api_url_prefix` and the provider
/// name.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// `OAuth2` client ID issued by the provider during app registration.
    pub client_id: ClientId,
    /// `OAuth2` client secret issued by the provider during app registration.
    pub client_secret: ClientSecret,
    /// Provider's `OAuth2` authorization endpoint.
    pub auth_uri: AuthUrl,
    /// Provider's `OAuth2` token exchange endpoint.
    pub token_uri: TokenUrl,
    /// Redirect URI registered with the provider.
    ///
    /// Parsed for format compatibility with the Google credentials JSON; the
    /// service computes its own redirect URI at runtime and does not read this
    /// field.
    pub redirect_uri: RedirectUrl,
}

impl<'de> Deserialize<'de> for Credentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        #[derive(Deserialize)]
        struct Outer {
            web: Inner,
        }

        #[derive(Deserialize)]
        struct Inner {
            client_id: ClientId,
            client_secret: ClientSecret,
            auth_uri: AuthUrl,
            token_uri: TokenUrl,
            redirect_uris: Vec<RedirectUrl>,
        }

        let outer = Outer::deserialize(deserializer)?;
        let [redirect_uri] = outer.web.redirect_uris.as_slice() else {
            return Err(serde::de::Error::custom(
                "redirect_uris must contain only one entry",
            ));
        };
        Ok(Self {
            client_id: outer.web.client_id,
            client_secret: outer.web.client_secret,
            auth_uri: outer.web.auth_uri,
            token_uri: outer.web.token_uri,
            redirect_uri: redirect_uri.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Credentials;

    #[test]
    fn deserializes_google_credentials_json() {
        let json = serde_json::json!({
            "web": {
                "client_id": "test-client-id",
                "client_secret": "test-client-secret",
                "auth_uri": "https://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://accounts.google.com/o/oauth2/token",
                "redirect_uris": ["https://example.com/auth/callback"]
            }
        });

        let creds: Credentials = serde_json::from_value(json).expect("valid credentials JSON");

        assert_eq!(creds.client_id.as_str(), "test-client-id");
        assert_eq!(creds.client_secret.secret(), "test-client-secret");
        assert_eq!(
            creds.auth_uri.as_str(),
            "https://accounts.google.com/o/oauth2/auth"
        );
        assert_eq!(
            creds.token_uri.as_str(),
            "https://accounts.google.com/o/oauth2/token"
        );
        assert_eq!(
            creds.redirect_uri.as_str(),
            "https://example.com/auth/callback"
        );
    }
}
