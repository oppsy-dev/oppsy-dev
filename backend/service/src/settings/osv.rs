use std::{path::PathBuf, time::Duration};

use serde::Deserialize;

pub(super) fn default_osv_db_path() -> PathBuf {
    PathBuf::from("./osv_db")
}

pub(super) const fn default_osv_sync_interval() -> Duration {
    Duration::from_mins(15)
}

pub(super) mod duration_mins {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where D: Deserializer<'de> {
        u64::deserialize(d).map(Duration::from_mins)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct OsvSettings {
    /// Filesystem path to the directory used by [`OsvDb`] for storing OSV data.
    ///
    /// Defaults to `./osv_db` relative to the working directory.
    /// Set `OPPSY_SERVICE_OSV_DB_PATH` to override.
    ///
    /// [`OsvDb`]: crate::db::osv_db::OsvDb
    #[serde(default = "default_osv_db_path")]
    pub osv_db_path: PathBuf,
    /// How often the OSV background sync task runs in minutes.
    ///
    /// Defaults to 15 minutes.
    /// Set `OPPSY_SERVICE_OSV_SYNC_INTERVAL` to override.
    ///
    /// <https://google.github.io/osv.dev/faq/>
    /// "Data Freshness: Data sources no more than 15 minutes stale, 99.5% of the time."
    #[serde(with = "duration_mins", default = "default_osv_sync_interval")]
    pub osv_sync_interval: Duration,
}
