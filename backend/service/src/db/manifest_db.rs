//! On-disk manifest storage.

use std::ops::Deref;

use manifest_storage::ManifestStorage;

use crate::{
    resources::{Resource, ResourceRegistry},
    settings::Settings,
};

/// Thin wrapper over [`ManifestStorage`].
#[derive(Debug)]
pub struct ManifestDb {
    storage: ManifestStorage,
}

impl Deref for ManifestDb {
    type Target = ManifestStorage;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

#[async_trait::async_trait]
impl Resource for ManifestDb {
    /// Initializes the [`Settings`] instance from environment variables to the
    /// [`ResourceRegistry`].
    ///
    /// Must be called exactly once at service startup before any call to [`Self::get`].
    ///
    /// # Errors
    /// - Returns an error if any required environment variable is absent or malformed.
    async fn init() -> anyhow::Result<Self>
    where Self: Sized {
        let settings = ResourceRegistry::get::<Settings>()?;
        Ok(Self {
            storage: ManifestStorage::new(&settings.manifest_db_path)?,
        })
    }
}
