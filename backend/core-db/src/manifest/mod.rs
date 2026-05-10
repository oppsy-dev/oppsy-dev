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
pub type ManifestMeta = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Manifest {
    pub id: ManifestId,
    pub name: ManifestName,
    pub tag: Option<ManifestTag>,
    pub meta: ManifestMeta,
}

/// Trying to read row in the format `(manifests.id, manifests.name, manifests.tag,
/// manifests.meta)`
impl TryFrom<sqlx::sqlite::SqliteRow> for Manifest {
    type Error = ManifestFromRowError;

    fn try_from(row: sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        let id = row
            .try_get(0)
            .map_err(ManifestFromRowError::CannotDecodeId)?;
        let name = row
            .try_get(1)
            .map_err(ManifestFromRowError::CannotDecodeName)?;
        let tag = row
            .try_get(2)
            .map_err(ManifestFromRowError::CannotDecodeTag)?;
        let meta = row
            .try_get(3)
            .map_err(ManifestFromRowError::CannotDecodeMeta)?;
        Ok(Manifest {
            id,
            name,
            tag,
            meta,
        })
    }
}

impl CoreDb {
    pub async fn add_manifest(
        &self,
        id: impl ConvertTo<ManifestId>,
        name: impl ConvertTo<ManifestName>,
        tag: impl ConvertTo<Option<ManifestTag>>,
        meta: impl ConvertTo<ManifestMeta>,
    ) -> Result<(), AddManifestError> {
        let id = id.convert()?;
        let name = name.convert()?;
        let tag = tag.convert()?;
        let meta = meta.convert()?;

        let res =
            sqlx::query("INSERT INTO manifests (id, name, tag, meta) VALUES ($1, $2, $3, $4)")
                .bind(id)
                .bind(&name)
                .bind(tag.as_deref())
                .bind(meta)
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
        let row = sqlx::query("SELECT id, name, tag, meta FROM manifests WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(GetManifestError::Database)?
            .ok_or(GetManifestError::NotFound { id })?;
        Ok(row.convert()?)
    }
}
