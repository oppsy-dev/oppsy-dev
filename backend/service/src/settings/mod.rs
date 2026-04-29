#[cfg(test)]
mod tests;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

use config::Config;
use serde::Deserialize;
use url::Url;

use crate::resources::Resource;

const ENV_VAR_PREFIX: &str = "OSV_SERVICE";

const fn default_bind_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 3030)
}

fn default_api_url_prefix() -> String {
    "/api".to_string()
}

fn default_manifest_db_path() -> PathBuf {
    PathBuf::from("./manifest_db")
}

fn default_osv_db_path() -> PathBuf {
    PathBuf::from("./osv_db")
}

pub(super) fn default_core_db_url() -> String {
    "sqlite://oppsy.db".to_string()
}

const fn default_osv_sync_interval() -> Duration {
    Duration::from_mins(15)
}

mod duration_mins {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where D: Deserializer<'de> {
        u64::deserialize(d).map(Duration::from_mins)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Settings {
    /// Server binding address
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,
    /// Tracing output format
    #[serde(default)]
    pub log_format: LogFormat,
    /// Minimum log level to emit.
    #[serde(default)]
    pub log_level: LogLevel,
    /// The base path the API is served at.
    #[serde(default = "default_api_url_prefix")]
    pub api_url_prefix: String,
    /// Filesystem path to the directory used by [`ManifestDb`] for storing manifest
    /// files.
    ///
    /// Defaults to `./manifest_db` relative to the working directory.
    /// Set `OSV_SERVICE_MANIFEST_DB_PATH` to override.
    ///
    /// [`ManifestDb`]: crate::db::manifest_db::ManifestDb
    #[serde(default = "default_manifest_db_path")]
    pub manifest_db_path: PathBuf,
    /// Filesystem path to the directory used by [`OsvDb`] for storing OSV data.
    ///
    /// Defaults to `./osv_db` relative to the working directory.
    /// Set `OSV_SERVICE_OSV_DB_PATH` to override.
    ///
    /// [`OsvDb`]: crate::db::osv_db::OsvDb
    #[serde(default = "default_osv_db_path")]
    pub osv_db_path: PathBuf,
    /// `SQLite` connection URL used by [`CoreDb`].
    ///
    /// Defaults to `sqlite://oppsy.db` relative to the working directory.
    /// Set `OSV_SERVICE_CORE_DB_URL` to override.
    #[serde(default = "default_core_db_url")]
    pub core_db_url: String,
    /// How often the OSV background sync task runs in minutes.
    ///
    /// Defaults to 15 minutes.
    /// Set `OSV_SERVICE_OSV_SYNC_INTERVAL` to override.
    ///
    /// <https://google.github.io/osv.dev/faq/>
    /// "Data Freshness: Data sources no more than 15 minutes stale, 99.5% of the time."
    #[serde(with = "duration_mins", default = "default_osv_sync_interval")]
    pub osv_sync_interval: Duration,
    /// Origins allowed by the CORS middleware,
    /// a comma-separated list of allowed origins e.g.
    /// `https://app.example.com,https://staging.example.com`.
    ///
    /// **Development-only `--features = local-dev'`.** could be empty
    #[serde(default)]
    pub allowed_cors_origins: Vec<Url>,
}

#[async_trait::async_trait]
impl Resource for Settings {
    /// Initializes the [`Settings`] instance from environment variables to the
    /// [`ResourceRegistry`].
    ///
    /// Must be called exactly once at service startup before any call to [`Self::get`].
    ///
    /// # Errors
    /// - Returns an error if any required environment variable is absent or malformed.
    async fn init() -> anyhow::Result<Self>
    where Self: Sized {
        Self::load()
    }
}

impl Settings {
    /// Initializes the [`Settings`] instance from environment variables.
    /// Reads settings from environment variables without storing them.
    ///
    /// # Errors
    /// - Returns an error if any required environment variable is absent or malformed.
    fn load() -> anyhow::Result<Self> {
        let res: Settings = Config::builder()
            .add_source(
                config::Environment::with_prefix(ENV_VAR_PREFIX)
                    .try_parsing(true)
                    .list_separator(",")
                    .with_list_parse_key("allowed_cors_origins"),
            )
            .build()?
            .try_deserialize()?;
        if !cfg!(feature = "local-dev") && res.allowed_cors_origins.is_empty() {
            anyhow::bail!(
                "allowed_cors_origins is empty — all origins are permitted by the CORS \
                middleware and open-redirect protection on the auth callback is disabled. \
                This is only acceptable for local development, built with '--features local-dev' flag."
            );
        }
        Ok(res)
    }
}

/// Controls how tracing output is formatted.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// Human-readable text output (default).
    #[default]
    HumanReadable,
    /// Structured JSON output.
    Json,
}

/// Minimum log level to emit.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}
