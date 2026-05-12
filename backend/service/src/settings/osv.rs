use std::{path::PathBuf, str::FromStr, time::Duration};

use osv_db::OsvGsEcosystem;
use serde::Deserialize;

pub(super) fn default_osv_db_path() -> PathBuf {
    PathBuf::from("./osv_db")
}

pub(super) const fn default_osv_sync_interval() -> Duration {
    Duration::from_mins(15)
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(clippy::struct_field_names)]
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
    /// Additional OSV ecosystems to track beyond the defaults.
    ///
    /// Accepts ecosystem names as defined by OSV (e.g. `"crates.io"`, `"npm"`).
    /// Defaults to an empty list (no extra ecosystems).
    /// Set `OPPSY_SERVICE_OSV_ECOSYSTEMS` to override.
    #[serde(with = "gs_ecosystems", default)]
    pub osv_ecosystems: Vec<OsvGsEcosystem>,
}

mod gs_ecosystems {
    use serde::{Deserialize, Deserializer};

    use super::{FromStr, OsvGsEcosystem};

    pub fn deserialize<'de, D>(d: D) -> Result<Vec<OsvGsEcosystem>, D::Error>
    where D: Deserializer<'de> {
        Vec::<String>::deserialize(d)?
            .into_iter()
            .map(|s| OsvGsEcosystem::from_str(&s).map_err(serde::de::Error::custom))
            .collect()
    }
}

mod duration_mins {
    use serde::{Deserialize, Deserializer};

    use super::Duration;

    pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where D: Deserializer<'de> {
        u64::deserialize(d).map(Duration::from_mins)
    }
}
