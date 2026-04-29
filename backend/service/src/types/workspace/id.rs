use poem_openapi::{NewType, types::Example};

use crate::types::uuid_v7::UuidV7;

/// UUID v7 identifier for a workspace.
#[derive(NewType, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[oai(example)]
pub struct WorkspaceId(UuidV7);

impl WorkspaceId {
    /// Generates a new random workspace ID.
    pub fn generate() -> Self {
        Self(UuidV7::generate())
    }
}

impl Example for WorkspaceId {
    fn example() -> Self {
        Self(UuidV7::example())
    }
}

impl std::fmt::Display for WorkspaceId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<WorkspaceId> for core_db::workspace::WorkspaceId {
    fn from(value: WorkspaceId) -> Self {
        value.0.into()
    }
}

impl TryFrom<uuid::Uuid> for WorkspaceId {
    type Error = <UuidV7 as TryFrom<uuid::Uuid>>::Error;

    fn try_from(value: uuid::Uuid) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}
