//! Auth-specific configuration.

use std::{ops::Deref, path::PathBuf};

use ed25519_dalek::{SigningKey, pkcs8::DecodePrivateKey};
use serde::Deserialize;

pub(super) fn default_github_auth_client_path() -> PathBuf {
    PathBuf::from("./github_auth_client.json")
}

pub(super) fn default_google_auth_client_path() -> PathBuf {
    PathBuf::from("./google_auth_client.json")
}

/// Auth-specific settings.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuthSettings {
    /// Path to the JSON file containing GitHub `OAuth2` client credentials.
    ///
    /// The file must be a JSON object with `client_id`, `client_secret`, and
    /// `redirect_uri` fields. The authorization and token endpoints are fixed
    /// to GitHub's standard URLs and are not read from this file.
    ///
    /// Defaults to `./github_auth_client.json`.
    /// Set `OSV_SERVICE_GITHUB_AUTH_CLIENT_CREDS_PATH` to override.
    #[serde(default = "default_github_auth_client_path")]
    pub github_auth_client_creds_path: PathBuf,
    /// Path to the JSON file containing Google `OAuth2` client credentials.
    ///
    /// The file must be in the Google Cloud credentials JSON format (the
    /// `web` key with `client_id`, `client_secret`, `auth_uri`, `token_uri`).
    ///
    /// Defaults to `./google_auth_client.json`.
    /// Set `OSV_SERVICE_GOOGLE_AUTH_CLIENT_CREDS_PATH` to override.
    #[serde(default = "default_google_auth_client_path")]
    pub google_auth_client_creds_path: PathBuf,
    /// JWT Ed25519 signing key  in PEM format.
    /// ```text
    /// -----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAYc/lirZT2XAwNjwSiAtAoyTPeTfoDx5TrQLc4/c5Cs\n-----END PRIVATE KEY-----
    /// ```
    pub jwt_signing_key: JwtSigningKey,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JwtSigningKey(SigningKey);

impl Deref for JwtSigningKey {
    type Target = SigningKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for JwtSigningKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(Self(
            SigningKey::from_pkcs8_pem(&s).map_err(serde::de::Error::custom)?,
        ))
    }
}

#[cfg(test)]
// `openssl genpkey -algorithm ed25519 | openssl pkcs8 -topk8 -nocrypt`
pub const JWT_SIGNING_KEY_STR: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAYc/lirZT2XAwNjwSiAtAoyTPeTfoDx5TrQLc4/c5Cs\n-----END PRIVATE KEY-----";

#[cfg(test)]
pub fn jwt_signing_key() -> JwtSigningKey {
    serde_json::from_value(serde_json::json!(JWT_SIGNING_KEY_STR)).unwrap()
}
