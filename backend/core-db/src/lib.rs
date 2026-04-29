//! SQLite-backed store for OPPSY's core domain entities: users, teams, and workspaces.
//!
//! The database schema is managed by Atlas — apply migrations with

pub mod manifest;
pub mod manifest_osv_vuln;
pub mod notification_channel;
pub mod notification_event;
pub mod user;
mod version_check;
pub mod workspace;

pub use common::ConvertError;
use common::ConvertTo;
use sqlx::SqlitePool;
pub use version_check::VersionCheckError;

/// Pagination parameters passed to list queries.
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    /// 0-based page index.
    pub page: u32,
    /// Maximum rows returned per page.
    pub limit: u32,
}

impl Pagination {
    /// Returns all rows from the first page; use for internal callers that are not
    /// user-facing and always need the complete result set.
    #[must_use]
    pub fn all() -> Self {
        Self {
            page: 0,
            limit: u32::MAX,
        }
    }

    /// Row offset for `OFFSET $n`: `page × limit`, saturating on overflow.
    #[must_use]
    pub fn offset(self) -> u32 {
        self.page.saturating_mul(self.limit)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("Failed to connect to the database: {0}")]
    Connect(sqlx::Error),

    #[error(transparent)]
    VersionCheck(#[from] VersionCheckError),
}

/// Connection pool for the core SQL database.
#[derive(Debug)]
pub struct CoreDb {
    pool: SqlitePool,
}

impl CoreDb {
    /// Opens a connection pool and verifies the database schema matches this crate's
    /// migrations.
    ///
    /// # Errors:
    /// - [`InitError::Connect`] if the connection pool cannot be opened.
    /// - [`InitError::VersionCheck`] if the schema does not match.
    pub async fn new(url: &str) -> Result<Self, InitError> {
        let db = Self {
            pool: SqlitePool::connect(url).await.map_err(InitError::Connect)?,
        };

        db.check_version().await?;

        Ok(db)
    }
}
