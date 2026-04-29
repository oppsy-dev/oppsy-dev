//! User identifier domain type.

use poem_openapi::{NewType, types::Example};
use serde::{Deserialize, Serialize};

use crate::types::uuid_v7::UuidV7;

/// UUID v7 identifier for a user.
#[derive(NewType, Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[oai(example)]
pub struct UserId(UuidV7);

impl UserId {
    /// Generates a new random user ID.
    pub fn generate() -> Self {
        Self(UuidV7::generate())
    }
}

impl Example for UserId {
    fn example() -> Self {
        Self(UuidV7::example())
    }
}

impl std::fmt::Display for UserId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UserId> for core_db::user::UserId {
    fn from(value: UserId) -> Self {
        value.0.into()
    }
}

impl TryFrom<uuid::Uuid> for UserId {
    type Error = <UuidV7 as TryFrom<uuid::Uuid>>::Error;

    fn try_from(value: uuid::Uuid) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}
