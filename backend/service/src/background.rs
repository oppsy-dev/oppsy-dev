use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use tracing::{error, info};

use crate::{
    db::{CoreDb, OsvDb},
    notifier::Notifier,
    resources::ResourceRegistry,
    settings::Settings,
    types::{ManifestId, WorkspaceId},
};

/// Background task that syncs OSV data on the interval configured by
/// [`Settings::osv_sync_interval`] and dispatches vulnerability notifications
/// for any manifests affected by new or updated records.
///
/// Sleeps for one full interval before the first sync. [`Settings`] is registered
/// synchronously before any tasks are spawned, so the get succeeds immediately.
/// All other resources are guaranteed to be registered by then. Sync failures are
/// logged and retried on the next cycle.
pub async fn osv_sync_task() -> anyhow::Result<()> {
    let settings: Arc<Settings> = ResourceRegistry::get::<Settings>()?;
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

        if let Err(err) = run_osv_sync(&osv_db, &core_db, &notifier).await {
            error!(error = ?err, "OSV sync cycle failed");
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
        notifier.clone().spawn_osv_events(
            core_db.clone(),
            *workspace_id,
            manifest_id,
            records,
        );
    }

    Ok(())
}
