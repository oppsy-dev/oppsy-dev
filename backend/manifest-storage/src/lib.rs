mod errors;
mod manifest_id;

use std::{
    fs::OpenOptions,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use bytes::Bytes;
use common::ConvertTo;
pub use errors::{ManifestGetError, ManifestPutError, ManifestStorageInitError};
pub use manifest_id::{ManifestId, ManifestIdError};

/// A simple disk-backed storage for manifest files.
///
/// Each manifest is stored as a file inside `location`, named by its [`ManifestId`].
#[derive(Debug)]
pub struct ManifestStorage {
    location: PathBuf,
}

impl ManifestStorage {
    /// Creates a new [`ManifestStorage`] rooted at `location`.
    ///
    /// # Errors
    /// - [`ManifestStorageInitError`]
    pub fn new<P: AsRef<Path>>(location: P) -> Result<Self, ManifestStorageInitError> {
        std::fs::create_dir_all(&location).map_err(ManifestStorageInitError::Io)?;
        Ok(Self {
            location: location.as_ref().to_path_buf(),
        })
    }

    /// Writes `manifest` to disk under the given `manifest_id`.
    ///
    /// # Errors:
    /// - [`ManifestPutError`]
    pub fn put(
        &self,
        manifest_id: impl ConvertTo<ManifestId>,
        manifest: &[u8],
    ) -> Result<(), ManifestPutError> {
        let manifest_id = manifest_id.convert()?;
        let path = self.location.join(manifest_id.as_path());
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|source| {
                if source.kind() == ErrorKind::AlreadyExists {
                    ManifestPutError::AlreadyExists(manifest_id.clone())
                } else {
                    ManifestPutError::CannotWrite {
                        path: path.clone(),
                        source,
                    }
                }
            })?;
        f.write_all(manifest)
            .map_err(|source| ManifestPutError::CannotWrite { path, source })?;
        Ok(())
    }

    /// Reads the manifest stored under `manifest_id`.
    ///
    /// Returns `None` if no manifest with that id exists.
    ///
    /// # Errors:
    /// - [`ManifestGetError`]
    pub fn get(
        &self,
        manifest_id: impl ConvertTo<ManifestId>,
    ) -> Result<Option<Bytes>, ManifestGetError> {
        let manifest_id = manifest_id.convert()?;
        let path = self.location.join(manifest_id.as_path());
        let data = match std::fs::read(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
            Err(source) => return Err(ManifestGetError::ReadFailed { path, source }),
        };
        Ok(Some(Bytes::from(data)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_directory_location() {
        let result = ManifestStorage::new("./src/lib.rs");
        assert!(result.is_err());
    }

    #[test]
    fn manifest_db_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let db = ManifestStorage::new(dir.path()).unwrap();

        let content: Bytes = vec![1, 2, 3, 4, 5].into();
        let manifest_id = ManifestId::new("manifest_1").unwrap();

        assert!(db.put(manifest_id.clone(), &content).is_ok());
        assert!(db.put(manifest_id.clone(), &content).is_err());

        let content_from_db = db.get(manifest_id.clone()).unwrap();
        assert_eq!(content_from_db.unwrap(), content);

        let other_manifest_id = ManifestId::new("manifest_2").unwrap();
        assert!(db.get(other_manifest_id).unwrap().is_none());
    }
}
