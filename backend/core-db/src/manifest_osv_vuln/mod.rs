pub mod errors;

use std::collections::HashSet;

use sqlx::Row;

use crate::{
    ConvertTo, CoreDb,
    manifest_osv_vuln::errors::{
        AddManifestOsvVulnError, GetManifestOsvVulnsError, ManifestOsvVulnFromRowError,
    },
};

pub type ManifestId = uuid::Uuid;
pub type OsvId = String;
/// Unix epoch seconds (UTC).
pub type DetectedAt = i64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsvVuln {
    pub osv_id: OsvId,
    pub detected_at: DetectedAt,
}

/// Reads a row in the format `(manifest_osv_vuln.osv_id, manifest_osv_vuln.detected_at)`.
impl TryFrom<sqlx::sqlite::SqliteRow> for OsvVuln {
    type Error = ManifestOsvVulnFromRowError;

    fn try_from(row: sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        let osv_id = row
            .try_get(0)
            .map_err(ManifestOsvVulnFromRowError::CannotDecodeOsvId)?;
        let detected_at = row
            .try_get(1)
            .map_err(ManifestOsvVulnFromRowError::CannotDecodeDetectedAt)?;
        Ok(OsvVuln {
            osv_id,
            detected_at,
        })
    }
}

impl CoreDb {
    pub async fn add_manifest_osv_vuln(
        &self,
        manifest_id: impl ConvertTo<ManifestId>,
        vulns: HashSet<impl ConvertTo<OsvId>>,
        detected_at: impl ConvertTo<DetectedAt>,
    ) -> Result<(), AddManifestOsvVulnError> {
        if vulns.is_empty() {
            return Ok(());
        }
        let manifest_id = manifest_id.convert()?;
        let detected_at = detected_at.convert()?;

        let mut qb = sqlx::QueryBuilder::new(
            "INSERT INTO manifest_osv_vuln (manifest_id, osv_id, detected_at) ",
        );
        qb.push("VALUES ");
        let mut separated = qb.separated(", ");
        for osv_id in vulns {
            let osv_id = osv_id.convert()?;
            separated.push("(");
            separated.push_bind_unseparated(manifest_id);
            separated.push_bind(osv_id);
            separated.push_bind(detected_at);
            separated.push_unseparated(")");
        }

        let res = qb.build().execute(&self.pool).await.map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    AddManifestOsvVulnError::AlreadyExists { manifest_id }
                } else {
                    AddManifestOsvVulnError::Database(e)
                }
            } else {
                AddManifestOsvVulnError::Database(e)
            }
        })?;

        if res.rows_affected() == 0 {
            return Err(AddManifestOsvVulnError::InvalidAffectedRowsAmount(
                res.rows_affected(),
            ));
        }

        Ok(())
    }

    pub async fn get_manifest_osv_vulns(
        &self,
        manifest_id: impl ConvertTo<ManifestId>,
    ) -> Result<Vec<OsvVuln>, GetManifestOsvVulnsError> {
        let manifest_id = manifest_id.convert()?;

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM manifests WHERE id = $1)")
                .bind(manifest_id)
                .fetch_one(&self.pool)
                .await
                .map_err(GetManifestOsvVulnsError::Database)?;
        if !exists {
            return Err(GetManifestOsvVulnsError::ManifestNotFound { id: manifest_id });
        }

        let rows = sqlx::query(
            "SELECT osv_id, detected_at \
             FROM manifest_osv_vuln \
             WHERE manifest_id = $1",
        )
        .bind(manifest_id)
        .fetch_all(&self.pool)
        .await
        .map_err(GetManifestOsvVulnsError::Database)?;

        let vulns = rows
            .into_iter()
            .map(ConvertTo::convert)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(vulns)
    }
}
