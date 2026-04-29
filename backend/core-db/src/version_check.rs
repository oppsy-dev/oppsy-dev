use std::collections::HashSet;

use sqlx::Row as _;

use crate::CoreDb;

/// Embedded at compile time — the crate always checks against its own migration set.
const ATLAS_SUM: &str = include_str!("../sqlite-migrations/atlas.sum");

#[derive(thiserror::Error, Debug)]
pub enum VersionCheckError {
    #[error("Migration hash mismatch: expected `{expected:?}`, got `{actual:?}`")]
    HashMismatch {
        expected: HashSet<String>,
        actual: HashSet<String>,
    },

    #[error("Failed to query schema revisions: {0}")]
    Database(sqlx::Error),
}

impl CoreDb {
    /// Verifies that the connected database was migrated from exactly the same
    /// migration set this crate was compiled against.
    ///
    /// Compares per-file hashes from the embedded `atlas.sum` against the
    /// `atlas_schema_revisions` table written by Atlas on `migrate apply`.
    pub(crate) async fn check_version(&self) -> Result<(), VersionCheckError> {
        let expected = parse_atlas_sum(ATLAS_SUM);

        let rows = sqlx::query("SELECT hash FROM atlas_schema_revisions")
            .fetch_all(&self.pool)
            .await
            .map_err(VersionCheckError::Database)?;

        let actual: HashSet<String> = rows.into_iter().map(|row| row.get("hash")).collect();

        if expected != actual {
            return Err(VersionCheckError::HashMismatch { expected, actual });
        }

        Ok(())
    }
}

/// Extracts per-file content hashes from an `atlas.sum` file into a set.
///
/// The first line is the directory-level integrity hash and is skipped —
/// only per-file hashes are relevant for the revision comparison.
/// The `h1:` algorithm prefix is stripped to match the format stored in
/// `atlas_schema_revisions.hash`.
fn parse_atlas_sum(content: &str) -> HashSet<String> {
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let (_, hash_str) = line.split_once(' ')?;
            let hash = hash_str.strip_prefix("h1:")?;
            Some(hash.to_string())
        })
        .collect()
}
