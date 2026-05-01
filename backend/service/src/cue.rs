use std::ops::Deref;

use crate::resources::Resource;

/// Thin wrapper around [`cue_rs::Ctx`]
pub struct CueCtx(cue_rs::Ctx);

impl Deref for CueCtx {
    type Target = cue_rs::Ctx;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait::async_trait]
impl Resource for CueCtx {
    async fn init() -> anyhow::Result<Self> {
        Ok(Self(cue_rs::Ctx::new()?))
    }
}
