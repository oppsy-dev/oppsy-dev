use std::path::PathBuf;

use thiserror::Error;

use crate::ManifestId;

#[derive(Debug, Error)]
pub enum ManifestStorageInitError {
    #[error("failed to create or access the storage directory: {0}")]
    Io(#[source] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ManifestPutError {
    #[error("failed to write manifest to disk at '{path}': {source}")]
    CannotWrite {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("manifest with the same id already exists {0}")]
    AlreadyExists(ManifestId),
    #[error(transparent)]
    CantConvert(#[from] common::ConvertError),
}

#[derive(Debug, Error)]
pub enum ManifestGetError {
    #[error("failed to read manifest from disk at '{path}': {source}")]
    ReadFailed {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error(transparent)]
    CantConvert(#[from] common::ConvertError),
}

#[derive(Debug, Error)]
pub enum ManifestIterError {
    #[error("failed to read storage directory: {0}")]
    ReadDir(std::io::Error),
}

#[derive(Debug, Error)]
pub enum ManifestEntryError {
    #[error("failed to read directory entry: {0}")]
    DirEntry(std::io::Error),
    #[error("directory entry has an invalid manifest id: {0}")]
    InvalidId(crate::ManifestIdError),
    #[error("failed to read manifest from disk at '{path}': {source}")]
    ReadFailed {
        path: PathBuf,
        source: std::io::Error,
    },
}
