//! `OAuth2` authorization code wrapper with poem-openapi support.

use std::borrow::Cow;

use poem_openapi::{
    registry::{MetaSchema, MetaSchemaRef, Registry},
    types::{ParseError, ParseFromParameter, Type},
};

/// The authorization code issued by the `OAuth2` provider in the callback.
/// <https://datatracker.ietf.org/doc/html/rfc6749#appendix-A.11>
#[derive(Debug, Clone)]
pub struct AuthorizationCode(oauth2::AuthorizationCode);

impl From<AuthorizationCode> for oauth2::AuthorizationCode {
    fn from(value: AuthorizationCode) -> Self {
        value.0
    }
}
impl Type for AuthorizationCode {
    type RawElementValueType = Self;
    type RawValueType = Self;

    const IS_REQUIRED: bool = true;

    fn name() -> Cow<'static, str> {
        "string".into()
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new("string")))
    }

    fn register(_registry: &mut Registry) {}

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(std::iter::once(self))
    }
}

impl ParseFromParameter for AuthorizationCode {
    fn parse_from_parameter(value: &str) -> poem_openapi::types::ParseResult<Self> {
        if value.is_empty() {
            return Err(ParseError::custom("authorization code must not be empty"));
        }
        Ok(Self(oauth2::AuthorizationCode::new(value.to_owned())))
    }
}
