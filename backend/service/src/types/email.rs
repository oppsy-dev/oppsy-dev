use std::str::FromStr;

use poem::http::HeaderValue;
use poem_openapi::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{
        Example, ParseFromJSON, ParseFromMultipartField, ParseFromParameter, ToHeader, ToJSON, Type,
    },
};
use serde::Deserialize;

/// A validated email address used to identify a user account.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(lettre::Address);

impl Example for EmailAddress {
    #[allow(clippy::unwrap_used)]
    fn example() -> Self {
        Self("john_doe@mail.com".parse().unwrap())
    }
}

impl From<EmailAddress> for core_db::user::EmailAddress {
    fn from(value: EmailAddress) -> Self {
        value.0.to_string()
    }
}

impl From<EmailAddress> for lettre::Address {
    fn from(value: EmailAddress) -> Self {
        value.0
    }
}

impl Type for EmailAddress {
    type RawElementValueType = Self;
    type RawValueType = Self;

    const IS_REQUIRED: bool = true;

    fn name() -> std::borrow::Cow<'static, str> {
        concat!("string", "_", "email").into()
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "email")))
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }

    fn raw_element_iter<'a>(
        &'a self
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }

    #[inline]
    fn is_empty(&self) -> bool {
        false
    }

    #[inline]
    fn is_none(&self) -> bool {
        false
    }
}

impl ParseFromJSON for EmailAddress {
    fn parse_from_json(value: Option<serde_json::Value>) -> poem_openapi::types::ParseResult<Self> {
        match value {
            Some(value) => {
                let res = lettre::Address::deserialize(value)?;
                Ok(Self(res))
            },
            None => Err(poem_openapi::types::ParseError::expected_input()),
        }
    }
}

impl ParseFromParameter for EmailAddress {
    fn parse_from_parameter(value: &str) -> poem_openapi::types::ParseResult<Self> {
        let res = lettre::Address::from_str(value)?;
        Ok(Self(res))
    }
}

impl ParseFromMultipartField for EmailAddress {
    async fn parse_from_multipart(
        field: Option<poem::web::Field>
    ) -> poem_openapi::types::ParseResult<Self> {
        match field {
            Some(field) => Ok(Self(field.text().await?.parse()?)),
            None => Err(poem_openapi::types::ParseError::expected_input()),
        }
    }
}

impl ToJSON for EmailAddress {
    fn to_json(&self) -> Option<serde_json::Value> {
        serde_json::to_value(&self.0).ok()
    }
}

impl ToHeader for EmailAddress {
    fn to_header(&self) -> Option<poem::http::HeaderValue> {
        HeaderValue::from_str(self.0.as_ref()).ok()
    }
}
