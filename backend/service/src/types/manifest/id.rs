use poem_openapi::{
    NewType,
    types::{Example, ParseFromParameter},
};

use crate::types::uuid_v7::UuidV7;

/// UUID v7 identifier for a manifest.
#[derive(NewType, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[oai(example)]
pub struct ManifestId(UuidV7);

impl ManifestId {
    /// Generates a new random manifest ID.
    pub fn generate() -> Self {
        Self(UuidV7::generate())
    }
}

impl Example for ManifestId {
    fn example() -> Self {
        Self(UuidV7::example())
    }
}

impl TryFrom<&ManifestId> for manifest_storage::ManifestId {
    type Error = manifest_storage::ManifestIdError;

    fn try_from(value: &ManifestId) -> Result<Self, Self::Error> {
        Self::new(&value.0.to_string())
    }
}

impl TryFrom<manifest_storage::ManifestId> for ManifestId {
    type Error = anyhow::Error;

    fn try_from(value: manifest_storage::ManifestId) -> Result<Self, Self::Error> {
        let str = value.as_path().to_str().ok_or(anyhow::anyhow!(
            "Cannot covert to 'manifest_storage::ManifestId' string"
        ))?;
        UuidV7::parse_from_parameter(str)
            .map_err(|e| anyhow::anyhow!("{}", e.message()))
            .map(Self)
    }
}

impl std::fmt::Display for ManifestId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ManifestId> for core_db::workspace::ManifestId {
    fn from(value: ManifestId) -> Self {
        value.0.into()
    }
}

impl TryFrom<core_db::workspace::ManifestId> for ManifestId {
    type Error = anyhow::Error;

    fn try_from(value: core_db::workspace::ManifestId) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
