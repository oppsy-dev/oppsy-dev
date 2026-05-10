use poem::http::HeaderValue;
use poem_openapi::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{
        Example, ParseFromJSON, ParseFromMultipartField, ParseFromParameter, ToHeader, ToJSON, Type,
    },
};

#[derive(Debug, Clone)]
pub struct Ecosystem(osv_types::EcosystemWithSuffix);

impl From<osv_types::EcosystemWithSuffix> for Ecosystem {
    fn from(value: osv_types::EcosystemWithSuffix) -> Self {
        Self(value)
    }
}

impl From<Ecosystem> for osv_types::EcosystemWithSuffix {
    fn from(value: Ecosystem) -> Self {
        value.0
    }
}

impl Example for Ecosystem {
    #[allow(clippy::unwrap_used)]
    fn example() -> Self {
        Self("npm".parse().unwrap())
    }
}

impl Type for Ecosystem {
    type RawElementValueType = Self;
    type RawValueType = Self;

    const IS_REQUIRED: bool = true;

    fn name() -> std::borrow::Cow<'static, str> {
        concat!("string", "_", "email").into()
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new("string")))
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

impl ParseFromJSON for Ecosystem {
    fn parse_from_json(value: Option<serde_json::Value>) -> poem_openapi::types::ParseResult<Self> {
        if let Some(value) = value
            && let Some(value) = value.as_str()
        {
            Self::parse_from_parameter(value)
        } else {
            Err(poem_openapi::types::ParseError::expected_input())
        }
    }
}

impl ParseFromParameter for Ecosystem {
    fn parse_from_parameter(value: &str) -> poem_openapi::types::ParseResult<Self> {
        value
            .parse()
            .map_err(poem_openapi::types::ParseError::custom)
            .map(Self)
    }
}

impl ParseFromMultipartField for Ecosystem {
    async fn parse_from_multipart(
        field: Option<poem::web::Field>
    ) -> poem_openapi::types::ParseResult<Self> {
        if let Some(field) = field {
            let text = field.text().await?;
            Self::parse_from_parameter(&text)
        } else {
            Err(poem_openapi::types::ParseError::expected_input())
        }
    }
}

impl ToJSON for Ecosystem {
    fn to_json(&self) -> Option<serde_json::Value> {
        serde_json::to_value(&self.0).ok()
    }
}

impl ToHeader for Ecosystem {
    fn to_header(&self) -> Option<poem::http::HeaderValue> {
        HeaderValue::from_str(&self.0.to_string()).ok()
    }
}
