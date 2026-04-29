//! Domain types for the `OAuth2` authentication flow.

mod authorization_code;
mod oauth_state_id;

pub use authorization_code::AuthorizationCode;
pub use oauth_state_id::OAuthStateId;
