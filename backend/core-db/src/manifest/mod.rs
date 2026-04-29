pub mod errors;

use sqlx::Row;

use crate::{
    ConvertTo, CoreDb,
    manifest::errors::{AddManifestError, GetManifestError, ManifestFromRowError},
};

pub type ManifestId = uuid::Uuid;
pub type ManifestType = String;
pub type ManifestName = String;
pub type ManifestTag = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Manifest {
    pub id: ManifestId,
    pub manifest_type: ManifestType,
    pub name: ManifestName,
    pub tag: Option<ManifestTag>,
}

/// Trying to read row in the format `(manifests.id, manifests.manifest_type,
/// manifests.name, manifests.tag)`
impl TryFrom<sqlx::sqlite::SqliteRow> for Manifest {
    type Error = ManifestFromRowError;

    fn try_from(row: sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        let id = row
            .try_get(0)
            .map_err(ManifestFromRowError::CannotDecodeId)?;
        let manifest_type = row
            .try_get(1)
            .map_err(ManifestFromRowError::CannotDecodeType)?;
        let name = row
            .try_get(2)
            .map_err(ManifestFromRowError::CannotDecodeName)?;
        let tag = row
            .try_get(3)
            .map_err(ManifestFromRowError::CannotDecodeTag)?;
        Ok(Manifest {
            id,
            manifest_type,
            name,
            tag,
        })
    }
}

impl CoreDb {
    pub async fn add_manifest(
        &self,
        id: impl ConvertTo<ManifestId>,
        manifest_type: impl ConvertTo<ManifestType>,
        name: impl ConvertTo<ManifestName>,
        tag: impl ConvertTo<Option<ManifestTag>>,
    ) -> Result<(), AddManifestError> {
        let id = id.convert()?;
        let manifest_type = manifest_type.convert()?;
        let name = name.convert()?;
        let tag: Option<ManifestTag> = tag.convert()?;

        let res = sqlx::query(
            "INSERT INTO manifests (id, manifest_type, name, tag) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(&manifest_type)
        .bind(&name)
        .bind(tag.as_deref())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    AddManifestError::AlreadyExists { id }
                } else {
                    AddManifestError::Database(e)
                }
            } else {
                AddManifestError::Database(e)
            }
        })?;

        if res.rows_affected() != 1 {
            return Err(AddManifestError::InvalidAffectedRowsAmount(
                res.rows_affected(),
            ));
        }

        Ok(())
    }

    pub async fn get_manifest(
        &self,
        id: impl ConvertTo<ManifestId>,
    ) -> Result<Manifest, GetManifestError> {
        let id = id.convert()?;
        let row = sqlx::query("SELECT id, manifest_type, name, tag FROM manifests WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(GetManifestError::Database)?
            .ok_or(GetManifestError::NotFound { id })?;
        Ok(row.convert()?)
    }
}
