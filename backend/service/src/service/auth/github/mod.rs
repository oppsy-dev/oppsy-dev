mod creds;

use std::path::Path;

use anyhow::Context as _;
pub use creds::Credentials;
use oauth2::{
    AccessToken, AuthorizationCode, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge,
    PkceCodeVerifier, Scope,
    basic::{BasicClient, BasicTokenResponse},
};
use serde::Deserialize;
use url::Url;

use crate::{
    service::auth::{OAuthClient, USER_AGENT_VALUE, http_client::HttpClient},
    types::EmailAddress,
};

#[derive(Debug)]
pub struct Client(
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
);

impl Client {
    pub fn new(credentials_path: &Path) -> anyhow::Result<Self> {
        let file =
            std::fs::File::open(credentials_path).context("failed to open credentials file")?;
        let credentials: Credentials =
            serde_json::from_reader(file).context("failed to parse credentials file")?;
        let client = BasicClient::new(credentials.client_id)
            .set_auth_uri(credentials.auth_uri)
            .set_token_uri(credentials.token_uri)
            .set_client_secret(credentials.client_secret)
            .set_redirect_uri(credentials.redirect_uri);

        Ok(Self(client))
    }
}

#[async_trait::async_trait]
impl OAuthClient for Client {
    fn auth(&self) -> (Url, PkceCodeVerifier, CsrfToken) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .0
            .authorize_url(CsrfToken::new_random)
            // <https://docs.github.com/en/developers/apps/building-oauth-apps/scopes-for-oauth-apps>
            .add_scope(Scope::new("user:email".to_string()))
            .add_scope(Scope::new("read:user".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url, pkce_verifier, csrf_token)
    }

    async fn get_token_response(
        &self,
        code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
    ) -> anyhow::Result<BasicTokenResponse> {
        let token_result = self
            .0
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(&HttpClient::new())
            .await?;
        Ok(token_result)
    }

    async fn fetch_email(
        &self,
        token: &AccessToken,
    ) -> anyhow::Result<EmailAddress> {
        // <https://docs.github.com/en/rest/users/emails?apiVersion=2026-03-10>
        #[derive(Deserialize)]
        struct EmailEntry {
            email: String,
            primary: bool,
            verified: bool,
        }

        const USER_EMAILS_URL: &str = "https://api.github.com/user/emails";

        let entries = reqwest::Client::new()
            .get(USER_EMAILS_URL)
            .bearer_auth(token.secret())
            .header("X-GitHub-Api-Version", "2026-03-10")
            // GitHub requires a User-Agent header on all API requests.
            .header(reqwest::header::USER_AGENT, USER_AGENT_VALUE)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<EmailEntry>>()
            .await?;

        entries
            .into_iter()
            .find(|e| e.primary && e.verified)
            .map(|e| e.email.into())
            .ok_or_else(|| anyhow::anyhow!("no verified primary email found for GitHub user"))
    }
}
