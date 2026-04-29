//! Pending `OAuth2` authorization state stored between the initiation and
//! callback phases of the flow.

use oauth2::PkceCodeVerifier;
use url::Url;

/// State held server-side while waiting for the provider to redirect back with
/// an authorization code.
#[derive(Debug, Clone)]
pub struct OAuthState {
    pub pkce_verifier: PkceCodeVerifierClonable,
    pub return_to: Url,
}

/// Thin wrapper over [`PkceCodeVerifier`] to implement [`Clone`]
#[derive(Debug)]
pub struct PkceCodeVerifierClonable(pub PkceCodeVerifier);

impl Clone for PkceCodeVerifierClonable {
    fn clone(&self) -> Self {
        Self(PkceCodeVerifier::new(self.0.secret().clone()))
    }
}
