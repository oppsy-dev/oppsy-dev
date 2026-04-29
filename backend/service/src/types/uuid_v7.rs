use poem_openapi::{
    registry::MetaSchemaRef,
    types::{
        Example, ParseFromJSON, ParseFromMultipartField, ParseFromParameter, ToHeader, ToJSON, Type,
    },
};
use serde::{Deserialize, Serialize};
use uuid::{Uuid, Version, uuid};

const EXAMPLE: &str = "019d4dd6-511f-70cf-810b-89ddf58ead9c";

/// Wrapper around [`uuid::Uuid`] that enforces UUID v7 (time-ordered, monotonic) format.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UuidV7(Uuid);

impl UuidV7 {
    /// Generates a new UUID v7.
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Example for UuidV7 {
    fn example() -> Self {
        Self(uuid!(EXAMPLE))
    }
}

impl std::fmt::Display for UuidV7 {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UuidV7> for Uuid {
    fn from(value: UuidV7) -> Self {
        value.0
    }
}

impl TryFrom<Uuid> for UuidV7 {
    type Error = anyhow::Error;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            value.get_version() == Some(Version::SortRand),
            "Must be a UUID v7"
        );
        Ok(Self(value))
    }
}

impl Type for UuidV7 {
    type RawElementValueType = Self;
    type RawValueType = Self;

    const IS_REQUIRED: bool = true;

    fn name() -> std::borrow::Cow<'static, str> {
        Uuid::name()
    }

    fn schema_ref() -> MetaSchemaRef {
        Uuid::schema_ref()
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
        self.0.is_empty()
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

impl ParseFromJSON for UuidV7 {
    fn parse_from_json(value: Option<serde_json::Value>) -> poem_openapi::types::ParseResult<Self> {
        let uuid =
            Uuid::parse_from_json(value).map_err(poem_openapi::types::ParseError::propagate)?;
        Ok(uuid.try_into()?)
    }
}

impl ParseFromParameter for UuidV7 {
    fn parse_from_parameter(value: &str) -> poem_openapi::types::ParseResult<Self> {
        let uuid = Uuid::parse_from_parameter(value)
            .map_err(poem_openapi::types::ParseError::propagate)?;
        Ok(uuid.try_into()?)
    }
}

impl ParseFromMultipartField for UuidV7 {
    async fn parse_from_multipart(
        field: Option<poem::web::Field>
    ) -> poem_openapi::types::ParseResult<Self> {
        let uuid = Uuid::parse_from_multipart(field)
            .await
            .map_err(poem_openapi::types::ParseError::propagate)?;
        Ok(uuid.try_into()?)
    }
}

impl ToJSON for UuidV7 {
    fn to_json(&self) -> Option<serde_json::Value> {
        self.0.to_json()
    }
}

impl ToHeader for UuidV7 {
    fn to_header(&self) -> Option<poem::http::HeaderValue> {
        self.0.to_header()
    }
}
