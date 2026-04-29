//! Data types for parsing `go list -m -json all | jq -s '.' > deps.json` output.
//!
//! The `go list -m -json all` command outputs a stream of concatenated JSON objects,
//! one per module. Each object corresponds to the `Module` struct from the Go toolchain:
//!
//! ```text
//! {"Path":"github.com/gorilla/mux","Version":"v1.8.0","GoVersion":"1.13"}
//! {"Path":"golang.org/x/text","Version":"v0.3.0","Replace":{"Path":"golang.org/x/text","Version":"v0.4.0"}}
//! ```

use semver::Version;
use serde::{Deserialize, Deserializer};

/// Data type for parsing `go list -m -json all | jq -s '.' > deps.json` output.
#[derive(Debug, Clone, Deserialize)]
pub struct GoList(Vec<GoModule>);

/// A single module entry from `go list -m -json` output.
#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub struct GoModule {
    /// The module path (e.g. `github.com/gorilla/mux`).
    #[serde(rename = "Path")]
    pub path: String,
    /// The resolved module version (e.g. `v1.8.0`), or empty for the main module.
    #[serde(
        rename = "Version",
        default,
        deserialize_with = "go_module_version_deserialize"
    )]
    pub version: Option<Version>,
    /// The replacement module, if this module was replaced via a `replace` directive.
    ///
    /// When set, vulnerability matching uses `replace.path` and `replace.version`
    /// instead of `self.path` and `self.version`.
    #[serde(rename = "Replace")]
    pub replace: Option<Box<GoModule>>,
}

impl GoList {
    /// Iterates over the [`GoModule`] entries, substituting the `replace` modules
    pub fn resolved_modules(self) -> impl Iterator<Item = GoModule> {
        let mut res = Vec::new();
        for mut walk_m in self.0 {
            while let Some(replaced_n) = walk_m.replace {
                walk_m = *replaced_n;
            }
            res.push(walk_m);
        }
        res.into_iter()
    }
}

fn go_module_version_deserialize<'de, D: Deserializer<'de>>(
    d: D
) -> Result<Option<Version>, D::Error> {
    let version_str = Option::<String>::deserialize(d)?;
    let Some(version_str) = version_str else {
        return Ok(None);
    };
    let version_str = version_str
        .strip_prefix('v')
        .ok_or(serde::de::Error::custom(
            "Go module entry `version` field must start with the 'v' prefix",
        ))?;
    Ok(Some(version_str.parse().map_err(serde::de::Error::custom)?))
}
