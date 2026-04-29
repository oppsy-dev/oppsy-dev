//! Runtime authentication state managed as a process-wide [`Resource`].
//!
//! [`Auth`] will hold the CSRF token store, the user store, and the
//! session store once the `OAuth2` flow is implemented. For now it is an empty
//! placeholder that satisfies the [`Resource`] contract so it can be
//! registered at startup alongside the other services.

#[cfg(feature = "local-dev")]
mod dev;
pub mod errors;
mod github;
mod google;
mod http_client;
pub mod security_scheme;
pub mod state;
pub mod token;
pub mod types;

use std::{collections::HashMap, fmt::Debug, time::Duration};

use core_db::user::errors::GetUserIdError;
pub use errors::{PkceVerifierNotFoundError, ProviderNotSupportedError};
use moka::future::Cache;
use oauth2::{
    AccessToken, AuthorizationCode, CsrfToken, PkceCodeVerifier, TokenResponse,
    basic::BasicTokenResponse,
};
use poem_openapi::Enum;
use url::Url;

use crate::{
    db::CoreDb,
    resources::{Resource, ResourceRegistry},
    service::auth::{
        state::{OAuthState, PkceCodeVerifierClonable},
        token::Token,
        types::OAuthStateId,
    },
    settings::Settings,
    types::{EmailAddress, UserId},
};

const USER_AGENT_VALUE: &str = "oppsy.dev";

/// How long a pending `OAuth2` state entry lives before it is evicted.
///
/// Keeps the server-side CSRF store from growing unbounded when users
/// abandon the flow mid-way.
const OAUTH_STATE_TTL: Duration = Duration::from_mins(30);
/// Maximum limit of the simultaneous amount of initialised `OAuth2` sessions.
const OAUTH_FLOWS_LIMIT: u64 = 10_000;

/// Supported `OAuth2` providers.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[oai(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthProvider {
    /// Sign in with GitHub.
    Github,
    /// Sign in with Google.
    Google,
    #[cfg(feature = "local-dev")]
    /// Fake `OAuth2` client, for local development only
    Dev,
}

/// Process-wide authentication state.
///
/// Intended to own:
/// - pending CSRF state (keyed by the random `state` parameter)
/// - authenticated user records
/// - active session tokens
#[derive(Debug)]
pub struct Auth {
    clients: HashMap<AuthProvider, Box<dyn OAuthClient>>,
    states: Cache<OAuthStateId, OAuthState>,
}

/// Abstracts the two-step `OAuth2` authorization flow for any provider.
#[async_trait::async_trait]
pub trait OAuthClient: Debug + Send + Sync {
    /// Builds the provider's authorization URL and returns the PKCE verifier
    /// and CSRF token that must be persisted until the callback is received.
    fn auth(&self) -> (Url, PkceCodeVerifier, CsrfToken);

    /// Exchanges an authorization code for a token response.
    async fn get_token_response(
        &self,
        code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
    ) -> anyhow::Result<BasicTokenResponse>;

    /// Fetches the email address associated with `token` from the provider's
    /// userinfo endpoint.
    #[allow(dead_code)]
    async fn fetch_email(
        &self,
        token: &AccessToken,
    ) -> anyhow::Result<EmailAddress>;
}

fn boxed_client<T: OAuthClient + 'static>(c: T) -> Box<dyn OAuthClient> {
    Box::new(c)
}

#[async_trait::async_trait]
impl Resource for Auth {
    async fn init() -> anyhow::Result<Self> {
        let settings = ResourceRegistry::get::<Settings>()?;
        let clients = [
            (
                AuthProvider::Google,
                google::Client::new(&settings.auth.google_auth_client_creds_path)
                    .map(boxed_client),
            ),
            (
                AuthProvider::Github,
                github::Client::new(&settings.auth.github_auth_client_creds_path)
                    .map(boxed_client),
            ),
            #[cfg(feature = "local-dev")]
            (
                AuthProvider::Dev,
                Ok(boxed_client(dev::Client::new(&settings))),
            ),
        ]
        .into_iter()
        .filter_map(|(provider, client)| {
            let client = client
                .inspect_err(|err| tracing::error!(error=?err, provider=?provider, "Client failed to initialise - sign-in with such provider will be unavailable"))
                .ok()?;
            Some((provider, client))
        })
        .collect();

        Ok(Self {
            clients,
            states: Cache::builder()
                .max_capacity(OAUTH_FLOWS_LIMIT)
                .time_to_live(OAUTH_STATE_TTL)
                .build(),
        })
    }
}

impl Auth {
    /// Builds the authorization redirect URL for the given provider.
    pub async fn auth(
        &self,
        provider: AuthProvider,
        redirect_to: Url,
    ) -> anyhow::Result<Url> {
        let client = self
            .clients
            .get(&provider)
            .ok_or(ProviderNotSupportedError)?;
        let (url, pkce_verifier, csrf_token) = client.auth();
        self.states
            .insert(csrf_token.into(), OAuthState {
                pkce_verifier: PkceCodeVerifierClonable(pkce_verifier),
                return_to: redirect_to,
            })
            .await;
        Ok(url)
    }

    /// Exchanges the authorization code for a token.
    /// If its a first login, registers user in [`CoreDb`] by the associated user's email.
    pub async fn get_token(
        &self,
        provider: AuthProvider,
        state_id: &OAuthStateId,
        code: AuthorizationCode,
        core_db: &CoreDb,
    ) -> anyhow::Result<(Token, Url)> {
        let oauth_state = self
            .states
            .remove(state_id)
            .await
            .ok_or(PkceVerifierNotFoundError)?;
        let client = self
            .clients
            .get(&provider)
            .ok_or(ProviderNotSupportedError)?;

        let token_resp = client
            .get_token_response(code, oauth_state.pkce_verifier.0)
            .await?;
        let email = client.fetch_email(token_resp.access_token()).await?;

        let user_id: UserId = match core_db.get_user_id(email.clone()).await {
            Ok(user_id) => user_id.try_into()?,
            Err(GetUserIdError::NotFound(_)) => {
                let user_id = UserId::generate();
                core_db.add_new_user(user_id, email).await?;
                user_id
            },
            Err(err) => return Err(err.into()),
        };

        Ok((
            Token::new(&token_resp, provider, user_id),
            oauth_state.return_to,
        ))
    }
}
