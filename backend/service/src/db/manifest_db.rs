//! On-disk manifest storage.

use std::ops::Deref;

use manifest_storage::ManifestStorage;

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
        self.0.put(manifest_id, &serde_json::to_vec(manifest)?)?;
        Ok(())
    }

    /// Returns the [`Manifest`] stored under `manifest_id`, or `None` if no
    /// matching manifest exists.
    pub fn get(
        &self,
        manifest_id: &ManifestId,
    ) -> anyhow::Result<Option<Manifest>> {
        let Some(bytes) = self.0.get(manifest_id)? else {
            return Ok(None);
        };
        let manifest: Manifest = serde_json::from_slice(&bytes)?;
        Ok(Some(manifest))
    }

    #[allow(clippy::iter_not_returning_iterator)]
    pub fn iter(
        &self
    ) -> anyhow::Result<impl Iterator<Item = anyhow::Result<(ManifestId, Manifest)>>> {
        Ok(self.0.iter()?.map(|e| {
            let (id, bytes) = e?;
            let id: ManifestId = id.try_into()?;
            let manifest: Manifest = serde_json::from_slice(&bytes)?;
            anyhow::Ok((id, manifest))
        }))
    }
}
