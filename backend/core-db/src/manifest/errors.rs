use crate::{ConvertError, manifest::ManifestId};

#[derive(thiserror::Error, Debug)]
pub enum AddManifestError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to insert manifest: {0}")]
    Database(sqlx::Error),
    #[error("Manifest with id: {id} already exists")]
    AlreadyExists { id: ManifestId },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum GetManifestError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Manifest with id {id} not found")]
    NotFound { id: ManifestId },
    #[error("Failed to query manifest: {0}")]
    Database(sqlx::Error),
    #[error(transparent)]
    CannotDecodeRow(#[from] ManifestFromRowError),
}

#[derive(thiserror::Error, Debug)]
pub enum ManifestFromRowError {
    #[error("Cannot decode manifest id column: {0}")]
    CannotDecodeId(sqlx::Error),
    #[error("Cannot decode manifest type column: {0}")]
    CannotDecodeType(sqlx::Error),
    #[error("Cannot decode manifest name column: {0}")]
    CannotDecodeName(sqlx::Error),
    #[error("Cannot decode manifest tag column: {0}")]
    CannotDecodeTag(sqlx::Error),
}
