use oauth2::{
    AccessToken, AuthorizationCode, CsrfToken, EmptyExtraTokenFields, PkceCodeChallenge,
    PkceCodeVerifier,
    basic::{BasicTokenResponse, BasicTokenType},
};
use url::Url;

use crate::{service::auth::OAuthClient, settings::Settings, types::EmailAddress};

#[derive(Debug)]
pub struct Client {
    /// Scheme + host + port + API prefix, e.g. `http://localhost:3030/api`.
    ///
    /// Derived from `Settings::bind_address` and `Settings::api_url_prefix` at
    /// startup so the callback URL stays correct when either is overridden via
    /// `OSV_SERVICE_BIND_ADDRESS` or `OSV_SERVICE_API_URL_PREFIX`.
    callback_base_url: Url,
}

impl Client {
    #[allow(clippy::expect_used)]
    pub fn new(settings: &Settings) -> Self {
        let port = settings.bind_address.port();
        let prefix = settings.api_url_prefix.trim_end_matches('/');
        let callback_base_url = Url::parse(&format!("http://localhost:{port}{prefix}"))
            .expect("dev callback base URL built from validated settings is always valid");
        Self { callback_base_url }
    }
}

#[async_trait::async_trait]
impl OAuthClient for Client {
    #[allow(clippy::expect_used)]
    fn auth(&self) -> (Url, PkceCodeVerifier, CsrfToken) {
        let (_, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let csrf_token = CsrfToken::new_random();
        let base = self.callback_base_url.as_str().trim_end_matches('/');
        let auth_url = Url::parse(&format!(
            "{base}/v1/auth/callback/dev?code=some_code&state={}",
            csrf_token.secret()
        ))
        .expect("dev callback URL built from validated base URL is always valid");
        (auth_url, pkce_verifier, csrf_token)
    }

    async fn get_token_response(
        &self,
        _code: AuthorizationCode,
        _pkce_verifier: PkceCodeVerifier,
    ) -> anyhow::Result<BasicTokenResponse> {
        let access_token = AccessToken::new("access-token".to_string());
        Ok(BasicTokenResponse::new(
            access_token,
            BasicTokenType::Bearer,
            EmptyExtraTokenFields {},
        ))
    }

    async fn fetch_email(
        &self,
        _token: &AccessToken,
    ) -> anyhow::Result<EmailAddress> {
        Ok("john_doe@mail.com".to_string().into())
    }
}
