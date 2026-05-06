use chrono::{DateTime, Utc};
use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    background::OsvSync,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::error_msg::ErrorMessage,
    },
    settings::Settings,
};

#[derive(Object)]
pub struct OsvSyncStatus {
    /// Timestamp of the last completed OSV sync.
    pub last_sync_at: DateTime<Utc>,
    /// How often the OSV sync runs in seconds.
    pub sync_interval: u64,
    /// Error message from the last sync cycle, if it failed.
    pub last_sync_error: Option<ErrorMessage>,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Response {
    /// ## OSV Sync Status
    ///
    /// Returns the timestamp of the last OSV sync and the configured sync interval.
    #[oai(status = 200)]
    Ok(Json<OsvSyncStatus>),
}

pub type AllResponses = WithErrorResponses<Response>;

pub async fn endpoint() -> AllResponses {
    let osv_sync = try_or_return!(ResourceRegistry::get::<OsvSync>());
    let settings = try_or_return!(ResourceRegistry::get::<Settings>());

    let state = osv_sync.last_state.read().await;

    Response::Ok(Json(OsvSyncStatus {
        last_sync_at: state.last_sync_at,
        sync_interval: settings.osv_sync_interval.as_secs(),
        last_sync_error: state.last_sync_err.as_ref().map(|e| e.to_string().into()),
    }))
    .into()
}
