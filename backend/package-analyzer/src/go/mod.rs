mod types;

use osv_db::types::{Affected, Ecosystem, PackageName, RangeType};
pub use types::{GoList, GoModule};

use crate::{Package, semver_eval};

impl Package for GoModule {
    fn name(&self) -> PackageName {
        self.path.clone()
    }

    fn matches_ecosystem(eco: Ecosystem) -> bool {
        matches!(eco, Ecosystem::Go)
    }

    fn parse_manifest(manifest_bytes: &[u8]) -> anyhow::Result<impl Iterator<Item = Self>>
    where Self: Sized {
        let list: GoList = serde_json::from_slice(manifest_bytes)?;
        Ok(list.resolved_modules())
    }

    fn evaluate(
        &self,
        affected: &Affected,
    ) -> anyhow::Result<bool> {
        let Some(ref pkg) = affected.package else {
            return Ok(false);
        };

        if !Self::matches_ecosystem(pkg.ecosystem.ecosystem()) {
            return Ok(false);
        }

        if pkg.name != self.name() {
            return Ok(false);
        }

        // No version constraints means all versions are affected.
        if affected.versions.is_empty() && affected.ranges.is_empty() {
            return Ok(true);
        }

        let Some(ref version) = self.version else {
            return Ok(false);
        };

        // Check the explicit versions list by string equality.
        if affected.versions.iter().any(|v| v == &version.to_string()) {
            return Ok(true);
        }

        // Check semver ranges
        let filtered_ranges = affected
            .ranges
            .iter()
            .filter(|r| r.range_type != RangeType::GIT);
        semver_eval::included_in_ranges(version, filtered_ranges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GO_JSON: &[u8] = include_bytes!("test.go.json");

    #[test]
    fn parse_manifest_test() {
        let packages: Vec<_> = GoModule::parse_manifest(TEST_GO_JSON)
            .expect("go list output should parse")
            .collect();

        assert_eq!(packages, vec![
            GoModule {
                path: "go-cue".to_string(),
                version: None,
                replace: None,
            },
            GoModule {
                path: "cloud.google.com/go/compute/metadata".to_string(),
                version: Some("0.3.0".parse().unwrap()),
                replace: None,
            },
            GoModule {
                path: "cuelabs.dev/go/oci/ociregistry".to_string(),
                version: Some("0.0.0-20250722084951-074d06050084".parse().unwrap()),
                replace: None,
            },
            GoModule {
                path: "cuelang.org/go".to_string(),
                version: Some("0.14.0".parse().unwrap()),
                replace: None
            },
            GoModule {
                path: "github.com/cockroachdb/apd/v3".to_string(),
                version: Some("3.2.1".parse().unwrap()),
                replace: None,
            }
        ]);
    }
}
