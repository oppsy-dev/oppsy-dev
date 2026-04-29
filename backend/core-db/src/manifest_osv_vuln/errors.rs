use crate::{ConvertError, manifest_osv_vuln::ManifestId};

#[derive(thiserror::Error, Debug)]
pub enum AddManifestOsvVulnError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to insert manifest OSV vulnerability: {0}")]
    Database(sqlx::Error),
    #[error("Vulnerability for manifest id: {manifest_id} already exists")]
    AlreadyExists { manifest_id: ManifestId },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum GetManifestOsvVulnsError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query manifest OSV vulnerabilities: {0}")]
    Database(sqlx::Error),
    #[error("Manifest with id: {id} not found")]
    ManifestNotFound { id: ManifestId },
}

#[derive(thiserror::Error, Debug)]
pub enum ManifestOsvVulnFromRowError {
    #[error("Cannot decode manifest_id column: {0}")]
    CannotDecodeManifestId(sqlx::Error),
    #[error("Cannot decode osv_id column: {0}")]
    CannotDecodeOsvId(sqlx::Error),
    #[error("Cannot decode detected_at column: {0}")]
    CannotDecodeDetectedAt(sqlx::Error),
}
