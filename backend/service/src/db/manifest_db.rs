//! On-disk manifest storage.

use std::ops::Deref;

use manifest_storage::ManifestStorage;
use poem_openapi::types::{ParseFromJSON, ToJSON};

use crate::{
    resources::{Resource, ResourceRegistry},
    settings::Settings,
    types::{Manifest, ManifestId},
};

/// Thin wrapper over [`ManifestStorage`].
#[derive(Debug)]
pub struct ManifestDb(ManifestStorage);

impl Deref for ManifestDb {
    type Target = ManifestStorage;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        Ok(Self(ManifestStorage::new(&settings.manifest_db_path)?))
    }
}

impl ManifestDb {
    pub fn put(
        &self,
        manifest_id: &ManifestId,
        manifest: &Manifest,
    ) -> anyhow::Result<()> {
        let manifest_json = manifest
            .to_json()
            .ok_or(anyhow::anyhow!("Manifest must be JSON encodable"))?;
        self.0
            .put(manifest_id, &serde_json::to_vec(&manifest_json)?)?;
        Ok(())
    }

    #[allow(clippy::iter_not_returning_iterator)]
    pub fn iter(
        &self
    ) -> anyhow::Result<impl Iterator<Item = anyhow::Result<(ManifestId, Manifest)>>> {
        Ok(self.0.iter().map(|iter| {
            iter.map(|e| {
                let (id, bytes) = e?;
                let id: ManifestId = id.try_into()?;
                let json = serde_json::from_slice::<serde_json::Value>(&bytes)?;
                let manifest: Manifest = Manifest::parse_from_json(Some(json))
                    .map_err(|e| anyhow::anyhow!("{}", e.message()))?;
                anyhow::Ok((id, manifest))
            })
        })?)
    }
}
