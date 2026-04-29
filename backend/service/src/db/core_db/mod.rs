//! In-memory storage for the core domain entities: users, teams, and workspaces.

use std::ops::Deref;

use crate::{
    resources::{Resource, ResourceRegistry},
    settings::Settings,
};

/// Thin wrapper over [`core_db::CoreDb`] so it can be registered as a [`Resource`].
#[derive(Debug)]
pub struct CoreDb(core_db::CoreDb);

impl Deref for CoreDb {
    type Target = core_db::CoreDb;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
impl Resource for CoreDb {
    async fn init() -> anyhow::Result<Self>
    where Self: Sized {
        let settings = ResourceRegistry::get::<Settings>()?;
        Ok(Self(core_db::CoreDb::new(&settings.core_db_url).await?))
    }
}
