mod osv;
#[cfg(test)]
mod tests;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use config::Config;
pub use osv::OsvSettings;
use serde::Deserialize;
use url::Url;

use crate::resources::Resource;

const ENV_VAR_PREFIX: &str = "OPPSY_SERVICE";

const fn default_bind_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 3030)
}

fn default_api_url_prefix() -> String {
    "/api".to_string()
}

fn default_manifest_db_path() -> PathBuf {
    PathBuf::from("./manifest_db")
}

fn default_frontend_path() -> PathBuf {
    PathBuf::from("./frontend")
}

pub(super) fn default_core_db_url() -> String {
    "sqlite://oppsy.db".to_string()
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
    /// Set `OPPSY_SERVICE_MANIFEST_DB_PATH` to override.
    ///
    /// [`ManifestDb`]: crate::db::manifest_db::ManifestDb
    #[serde(default = "default_manifest_db_path")]
    pub manifest_db_path: PathBuf,
    /// `SQLite` connection URL used by [`CoreDb`].
    ///
    /// Defaults to `sqlite://oppsy.db` relative to the working directory.
    /// Set `OPPSY_SERVICE_CORE_DB_URL` to override.
    #[serde(default = "default_core_db_url")]
    pub core_db_url: String,
    /// OSV-specific settings (`osv_db_path`, `osv_sync_interval`).
    /// Deserialized transparently from the same env-var namespace.
    #[serde(flatten)]
    pub osv: OsvSettings,
    /// Filesystem path to the directory containing the built frontend assets.
    ///
    /// The backend serves the React SPA from this directory at `/`, falling
    /// back to `index.html` for all unmatched paths (SPA routing). API routes
    /// under `api_url_prefix` and `/docs` always take priority.
    ///
    /// Defaults to `./frontend` relative to the working directory.
    /// Set `OPPSY_SERVICE_FRONTEND_PATH` to override.
    #[serde(default = "default_frontend_path")]
    pub frontend_path: PathBuf,
    /// SMTP server URL used for email notifications.
    ///
    /// Format: `smtp://username:password@host:port`
    /// If not set, email notifications are disabled.
    /// Set `OPPSY_SERVICE_SMTP_URL` to enable.
    #[serde(default)]
    pub smtp_url: Option<Url>,
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
                    .with_list_parse_key("osv_ecosystems"),
            )
            .build()?
            .try_deserialize()?;
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
