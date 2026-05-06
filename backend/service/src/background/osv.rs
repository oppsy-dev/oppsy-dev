use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{
    db::{CoreDb, OsvDb},
    notifier::Notifier,
    resources::{Resource, ResourceRegistry},
    settings::Settings,
    types::{ManifestId, WorkspaceId},
};

pub struct OsvSync {
    pub last_state: RwLock<SyncState>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SyncState {
    pub last_sync_at: DateTime<Utc>,
    pub last_sync_err: Option<anyhow::Error>,
}

#[async_trait::async_trait]
impl Resource for OsvSync {
    async fn init() -> anyhow::Result<Self> {
        let last_state = SyncState {
            last_sync_at: Utc::now(),
            last_sync_err: None,
        };
        Ok(Self { last_state: last_state.into() })
    }
}

impl OsvSync {
    /// Background task that syncs OSV data on the interval configured by
    /// [`Settings::osv_sync_interval`] and dispatches vulnerability notifications
    /// for any manifests affected by new or updated records.
    ///
    /// Sleeps for one full interval before the first sync. [`Settings`] is registered
    /// synchronously before any tasks are spawned, so the get succeeds immediately.
    /// All other resources are guaranteed to be registered by then. Sync failures are
    /// logged and retried on the next cycle.
    pub async fn osv_sync_task(self: Arc<OsvSync>) -> anyhow::Result<()> {
        let settings = ResourceRegistry::get::<Settings>()?;
        let sync_interval = settings.osv_sync_interval;

        loop {
            tokio::time::sleep(sync_interval).await;

            let Ok(osv_db) = ResourceRegistry::get::<OsvDb>() else {
                continue;
            };
            let Ok(core_db) = ResourceRegistry::get::<CoreDb>() else {
                continue;
            };
            let Ok(notifier) = ResourceRegistry::get::<Notifier>() else {
                continue;
            };

            let last_sync_err = run_osv_sync(&osv_db, &core_db, &notifier)
                .await
                .inspect_err(|e| error!(error = ?e, "OSV sync cycle failed"))
                .err();
            let last_sync_at = Utc::now();

            let mut last_state = self.last_state.write().await;
            *last_state = SyncState {
                last_sync_at,
                last_sync_err,
            };
        }
    }
}

async fn run_osv_sync(
    osv_db: &OsvDb,
    core_db: &Arc<CoreDb>,
    notifier: &Arc<Notifier>,
) -> anyhow::Result<()> {
    info!("Running scheduled OSV sync...");
    let detected_at = Utc::now();
    let hits = osv_db.sync().await?;

    if hits.is_empty() {
        info!("OSV sync complete: no new or updated records");
        return Ok(());
    }

    info!(
        affected_manifests = hits.len(),
        "OSV sync: resolving workspaces for affected manifests..."
    );

    let manifest_to_workspace: HashMap<ManifestId, WorkspaceId> = core_db
        .get_manifest_workspace_map()
        .await?
        .into_iter()
        .map(|(m, v)| anyhow::Ok((m.try_into()?, v.try_into()?)))
        .collect::<Result<_, _>>()?;

    for (manifest_id, records) in hits {
        let workspace_id = manifest_to_workspace
            .get(&manifest_id)
            .ok_or(anyhow::anyhow!(
                "Manifest {manifest_id} is not assinged to any workspaces"
            ))?;

        core_db
            .add_manifest_osv_vuln(
                manifest_id,
                records.iter().map(|v| v.id.clone()).collect(),
                detected_at.timestamp(),
            )
            .await?;
        // spawn notifications
        notifier
            .clone()
            .spawn_osv_events(core_db.clone(), *workspace_id, manifest_id, records);
    }

    Ok(())
}
