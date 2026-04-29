mod types;

use osv_db::types::{Affected, Ecosystem, PackageName, RangeType};
pub use types::{PoetryLock, PoetryPackage};

use crate::{Package, semver_eval};

impl Package for PoetryPackage {
    fn name(&self) -> PackageName {
        self.name.clone()
    }

    fn matches_ecosystem(eco: Ecosystem) -> bool {
        matches!(eco, Ecosystem::PyPI)
    }

    fn parse_manifest(manifest_bytes: &[u8]) -> anyhow::Result<impl Iterator<Item = Self>>
    where Self: Sized {
        let manifest_str = std::str::from_utf8(manifest_bytes)?;
        let lockfile: PoetryLock = toml::from_str(manifest_str)?;
        Ok(lockfile.packages.into_iter())
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

        // Check semver ranges.
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

    const TEST_POETRY_LOCK: &[u8] = include_bytes!("test.poetry.lock");

    #[test]
    fn parse_manifest_test() {
        let packages: Vec<_> = PoetryPackage::parse_manifest(TEST_POETRY_LOCK)
            .expect("lockfile should parse")
            .collect();

        assert_eq!(packages, vec![
            PoetryPackage {
                name: "requests".to_string(),
                version: "2.31.0".parse().unwrap(),
            },
            PoetryPackage {
                name: "urllib3".to_string(),
                version: "2.2.1".parse().unwrap(),
            },
            PoetryPackage {
                name: "my-git-dep".to_string(),
                version: "1.0.0".parse().unwrap(),
            },
        ]);
    }
}
