//! JWT token issuance for the BFF authentication layer.
//!
//! After a successful `OAuth2` callback the backend wraps the upstream token
//! response into a self-contained [`Token`] and hands it back to the client. The claim
//! layout follows `docs/auth/jwt_claims.schema.json`.

use std::{sync::LazyLock, time::Duration};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ed25519_dalek::{Signature, Signer, Verifier};
use oauth2::{
    AccessToken, RefreshToken, Scope, TokenResponse,
    basic::{BasicTokenResponse, BasicTokenType},
};
use serde::{Deserialize, Serialize};

use crate::{
    service::auth::{AuthProvider, errors::TokenDecodeError},
    settings::auth::JwtSigningKey,
    types::UserId,
};

/// Typed JWT wrapping the upstream `OAuth2` token response.
///
/// Holds the decoded header and claims. Call [`Token::encode`] to produce the
/// signed JWT string for transmission to the client.
#[derive(Debug, Clone)]
pub struct Token(Claims);

/// JWT claim set as defined in `jwt_claims.schema.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - [`UserId`]
    sub: UserId,
    /// Identifies the upstream `OAuth2` provider.
    provider: AuthProvider,
    /// The `OAuth2` token fields from [RFC 6749 §5.1].
    ///
    /// [RFC 6749 §5.1]: https://datatracker.ietf.org/doc/html/rfc6749#section-5.1
    oauth: OAuthClaims,
}

/// The `oauth` envelope inside [`JwtClaims`], mapping directly to
/// [RFC 6749 §5.1] fields.
///
/// [RFC 6749 §5.1]: https://datatracker.ietf.org/doc/html/rfc6749#section-5.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClaims {
    access_token: AccessToken,
    token_type: BasicTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<RefreshToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<Scope>>,
}

static HEADER: LazyLock<serde_json::Value> =
    LazyLock::new(|| serde_json::json!({"alg": "Ed25519", "typ": "JWT"}));

impl Token {
    /// Builds a [`Token`] from the upstream `OAuth2` token response.
    ///
    /// Constructs the RS256 header and claim set. Use [`Token::encode`] to
    /// sign and produce the JWT string.
    pub fn new(
        response: &BasicTokenResponse,
        provider: AuthProvider,
        sub: UserId,
    ) -> Self {
        let claims = Claims {
            sub,
            provider,
            oauth: OAuthClaims {
                access_token: response.access_token().clone(),
                token_type: response.token_type().clone(),
                expires_in: response.expires_in(),
                refresh_token: response.refresh_token().cloned(),
                scope: response.scopes().cloned(),
            },
        };
        Self(claims)
    }

    /// Decodes and verifies a JWT string, returning the contained claims.
    ///
    /// Signature verification is performed before the payload is deserialized,
    /// as required by [RFC 7519 §7.2].
    ///
    /// [RFC 7519 §7.2]: https://datatracker.ietf.org/doc/html/rfc7519#section-7.2
    ///
    /// # Errors
    ///
    /// Returns an error if the JWT is malformed, the signature is invalid, or
    /// the claims cannot be deserialized.
    pub fn decode(
        jwt: &str,
        key: &JwtSigningKey,
    ) -> Result<Self, TokenDecodeError> {
        let mut parts = jwt.splitn(3, '.');
        let header_b64 = parts
            .next()
            .ok_or(TokenDecodeError::MissingSegment("header"))?;
        let payload_b64 = parts
            .next()
            .ok_or(TokenDecodeError::MissingSegment("payload"))?;
        let sig_b64 = parts
            .next()
            .ok_or(TokenDecodeError::MissingSegment("signature"))?;

        // Verify signature before touching claims (RFC 7519 §7.2 step 8).
        let signing_input = format!("{header_b64}.{payload_b64}");
        let sig_bytes = URL_SAFE_NO_PAD.decode(sig_b64).map_err(|e| {
            TokenDecodeError::InvalidBase64 {
                segment: "signature",
                source: e,
            }
        })?;
        let signature =
            Signature::from_slice(&sig_bytes).map_err(TokenDecodeError::InvalidSignatureBytes)?;
        key.verifying_key()
            .verify(signing_input.as_bytes(), &signature)
            .map_err(TokenDecodeError::SignatureVerificationFailed)?;

        // Validate the protected header.
        let header_bytes = URL_SAFE_NO_PAD.decode(header_b64).map_err(|e| {
            TokenDecodeError::InvalidBase64 {
                segment: "header",
                source: e,
            }
        })?;
        let header: serde_json::Value =
            serde_json::from_slice(&header_bytes).map_err(TokenDecodeError::InvalidHeaderJson)?;
        if header != *HEADER {
            return Err(TokenDecodeError::UnexpectedHeader);
        }

        // Deserialize claims.
        let payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64).map_err(|e| {
            TokenDecodeError::InvalidBase64 {
                segment: "payload",
                source: e,
            }
        })?;
        let claims: Claims =
            serde_json::from_slice(&payload_bytes).map_err(TokenDecodeError::InvalidClaimsJson)?;

        Ok(Self(claims))
    }

    /// Signs the token and returns the encoded JWT string.
    ///
    /// Produces a compact serialization as defined in [RFC 7519 §3]:
    /// `BASE64URL(header) || '.' || BASE64URL(payload) || '.' || BASE64URL(signature)`
    ///
    /// The header is always `{"alg":"EdDSA","typ":"JWT"}`.
    ///
    /// [RFC 7519 §3]: https://datatracker.ietf.org/doc/html/rfc7519#section-3
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails.
    pub fn encode(
        &self,
        key: &JwtSigningKey,
    ) -> anyhow::Result<String> {
        let header_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&*HEADER)?);
        let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&self.0)?);
        let signing_input = format!("{header_b64}.{payload_b64}");
        let sig_b64 = URL_SAFE_NO_PAD.encode(key.sign(signing_input.as_bytes()).to_bytes());
        Ok(format!("{signing_input}.{sig_b64}"))
    }

    pub fn sub(&self) -> &UserId {
        &self.0.sub
    }
}

#[cfg(test)]
mod tests {
    use oauth2::{AccessToken, basic::BasicTokenType};

    use super::*;
    use crate::settings::auth::jwt_signing_key;

    fn test_claims() -> Claims {
        Claims {
            sub: UserId::generate(),
            provider: crate::service::auth::AuthProvider::Github,
            oauth: OAuthClaims {
                access_token: AccessToken::new("test_access_token".to_string()),
                token_type: BasicTokenType::Bearer,
                expires_in: None,
                refresh_token: None,
                scope: None,
            },
        }
    }

    #[test]
    fn token_encode_decode_roundtrip() {
        let key = jwt_signing_key();
        let claims = test_claims();
        let token = Token(claims.clone());
        let jwt = token.encode(&key).unwrap();
        // Compact serialization always has exactly 3 dot-separated segments.
        assert_eq!(jwt.split('.').count(), 3);
        let decoded = Token::decode(&jwt, &key).unwrap();
        assert_eq!(decoded.0.provider, claims.provider);

        // Validate decoded claims against the JWT claims JSON Schema.
        let schema_str = std::fs::read_to_string("../../docs/auth/jwt_claims.schema.json").unwrap();
        let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
        let claims_value = serde_json::to_value(&decoded.0).unwrap();
        let validator = jsonschema::validator_for(&schema).unwrap();
        let validation = validator.validate(&claims_value);
        assert!(
            validation.is_ok(),
            "decoded claims failed schema validation: {validation:?}"
        );
    }

    #[test]
    fn decode_rejects_tampered_payload() {
        let key = jwt_signing_key();
        let jwt = Token(test_claims()).encode(&key).unwrap();
        // Replace the payload segment with a different base64 value.
        let mut parts: Vec<&str> = jwt.split('.').collect();
        parts[1] = "dGFtcGVyZWQ";
        let tampered = parts.join(".");
        assert!(Token::decode(&tampered, &key).is_err());
    }

    #[test]
    fn decode_rejects_unexpected_header() {
        let key = jwt_signing_key();
        let header_b64 =
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(&serde_json::json!({})).unwrap());
        let payload_b64 = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&test_claims()).unwrap());
        let signing_input = format!("{header_b64}.{payload_b64}");
        let sig_b64 = URL_SAFE_NO_PAD.encode(key.sign(signing_input.as_bytes()).to_bytes());
        let jwt_corrupt_header = format!("{signing_input}.{sig_b64}");
        assert!(Token::decode(&jwt_corrupt_header, &key).is_err());
    }
}
