//! Data types for parsing `package-lock.json` (lockfile versions 2 and 3).
//!
//! A `package-lock.json` file has a flat `packages` object whose keys are
//! `node_modules/`-prefixed paths. The root entry (`""`) describes the project
//! itself; every other entry is a resolved dependency:
//!
//! ```json
//! {
//!   "packages": {
//!     "": { "name": "my-app", "version": "1.0.0" },
//!     "node_modules/lodash": { "version": "4.17.18", "resolved": "...", "integrity": "..." },
//!     "node_modules/a/node_modules/b": { "version": "2.0.0", ... }
//!   }
//! }
//! ```
//!
//! [`NpmLock`] deserializes this structure. [`NpmLock::resolved_packages`] then
//! strips the `node_modules/` prefix from each key to produce a flat list of
//! [`NpmPackage`] values — one per resolved dependency, root entry excluded.

use std::collections::{HashMap, HashSet};

use osv_db::types::PackageName;
use semver::Version;
use serde::Deserialize;

/// Represents a parsed `package-lock.json` (lockfile version 2 or 3).
#[derive(Debug, Clone, Deserialize)]
pub struct NpmLock {
    pub packages: HashMap<String, NpmPackageEntry>,
}

/// An entry in the `packages` object of a `package-lock.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct NpmPackageEntry {
    pub version: Option<Version>,
}

/// A resolved npm package with a known name and version.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NpmPackage {
    pub name: PackageName,
    pub version: Version,
}

impl NpmLock {
    /// Iterates over all resolved packages, skipping the root entry (`""`).
    ///
    /// Package names are derived from the `node_modules/` path key:
    /// - `"node_modules/lodash"` → `"lodash"`
    /// - `"node_modules/@scope/pkg"` → `"@scope/pkg"`
    /// - `"node_modules/a/node_modules/b"` → `"b"`
    pub fn resolved_packages(&self) -> impl Iterator<Item = NpmPackage> {
        self.packages
            .iter()
            .filter_map(|(key, entry)| {
                if key.is_empty() {
                    return None;
                }
                let (_, name) = key.rsplit_once("node_modules/")?;
                if name.is_empty() {
                    return None;
                }
                let version = entry.version.clone()?;
                Some(NpmPackage {
                    name: name.to_string(),
                    version,
                })
            })
            .collect::<HashSet<_>>()
            .into_iter()
    }
}
