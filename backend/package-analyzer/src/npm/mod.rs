mod types;

use osv_db::types::{Affected, Ecosystem, PackageName, RangeType};
use types::NpmLock;
pub use types::NpmPackage;

use crate::{Package, semver_eval};

impl Package for NpmPackage {
    fn name(&self) -> PackageName {
        self.name.clone()
    }

    fn parse_manifest(manifest_bytes: &[u8]) -> anyhow::Result<impl Iterator<Item = Self>>
    where Self: Sized {
        let manifest_str = std::str::from_utf8(manifest_bytes)?;
        let lockfile: NpmLock = serde_json::from_str(manifest_str)?;
        // Collect eagerly so the iterator outlives `lockfile`.
        let packages = lockfile.resolved_packages().collect::<Vec<_>>();
        Ok(packages.into_iter())
    }

    fn matches_ecosystem(eco: Ecosystem) -> bool {
        matches!(eco, Ecosystem::Npm)
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

        // Check the explicit versions list by string equality.
        if affected
            .versions
            .iter()
            .any(|v| v == &self.version.to_string())
        {
            return Ok(true);
        }

        // Check semver ranges
        let filtered_ranges = affected
            .ranges
            .iter()
            .filter(|r| r.range_type != RangeType::GIT);
        semver_eval::included_in_ranges(&self.version, filtered_ranges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_NPM_LOCK: &[u8] = include_bytes!("test.package.json");

    #[test]
    fn parse_manifest_test() {
        let mut packages: Vec<_> = NpmPackage::parse_manifest(TEST_NPM_LOCK)
            .expect("lockfile should parse")
            .collect();
        packages.sort_by(|a, b| a.name.cmp(&b.name));

        let expected = vec![
            NpmPackage {
                name: "@alloc/quick-lru".to_string(),
                version: "5.2.0".parse().unwrap(),
            },
            NpmPackage {
                name: "@babel/code-frame".to_string(),
                version: "7.29.0".parse().unwrap(),
            },
            NpmPackage {
                name: "acorn".to_string(),
                version: "8.16.0".parse().unwrap(),
            },
            NpmPackage {
                name: "eslint-visitor-keys".to_string(),
                version: "3.4.3".parse().unwrap(),
            },
            NpmPackage {
                name: "next".to_string(),
                version: "16.1.6".parse().unwrap(),
            },
        ];
        assert_eq!(packages, expected);
    }
}
