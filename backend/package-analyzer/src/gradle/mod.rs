mod types;

use osv_db::types::{Affected, Ecosystem, PackageName, RangeType};
pub use types::GradlePackage;
use types::parse_lockfile_line;

use crate::{Package, maven::version};

impl Package for GradlePackage {
    fn name(&self) -> PackageName {
        self.name.clone()
    }

    fn matches_ecosystem(eco: Ecosystem) -> bool {
        // Gradle projects publish to Maven Central and use the Maven ecosystem.
        matches!(eco, Ecosystem::Maven)
    }

    fn parse_manifest(manifest_bytes: &[u8]) -> anyhow::Result<impl Iterator<Item = Self>>
    where Self: Sized {
        let manifest_str = std::str::from_utf8(manifest_bytes)?;
        let packages = manifest_str
            .lines()
            .filter_map(parse_lockfile_line)
            .collect::<Vec<_>>();
        Ok(packages.into_iter())
    }

    fn evaluate(
        &self,
        affected: &Affected,
    ) -> anyhow::Result<bool> {
        let Some(ref pkg) = affected.package else {
            return Ok(false);
        };

        anyhow::ensure!(
            Self::matches_ecosystem(pkg.ecosystem.ecosystem()),
            "Provided `osv_record` does not match for {}",
            std::any::type_name::<Self>()
        );

        if pkg.name != self.name() {
            return Ok(false);
        }

        // No version constraints means all versions are affected.
        if affected.versions.is_empty() && affected.ranges.is_empty() {
            return Ok(true);
        }

        // Check the explicit versions list by string equality.
        if affected.versions.iter().any(|v| v == &self.version) {
            return Ok(true);
        }

        // Check ECOSYSTEM ranges using Maven version comparison.
        let ecosystem_ranges = affected
            .ranges
            .iter()
            .filter(|r| r.range_type == RangeType::ECOSYSTEM);

        for range in ecosystem_ranges {
            if version::ecosystem_range_contains(&self.version, &range.events) {
                return Ok(true);
            }
        }

        // Check SEMVER ranges where the version can be parsed as semver.
        let semver_ranges = affected
            .ranges
            .iter()
            .filter(|r| r.range_type == RangeType::SEMVER);

        if let Ok(semver_version) = normalize_as_semver(&self.version)
            && crate::semver_eval::included_in_ranges(&semver_version, semver_ranges)?
        {
            return Ok(true);
        }

        Ok(false)
    }
}

/// Attempts to parse a Maven version string as a [`semver::Version`].
///
/// Many Maven packages use `MAJOR.MINOR.PATCH` versioning that is directly semver-
/// compatible. Two-component versions like `"1.5"` are normalized to `"1.5.0"`.
fn normalize_as_semver(version: &str) -> anyhow::Result<semver::Version> {
    if let Ok(v) = semver::Version::parse(version) {
        return Ok(v);
    }
    // Normalize "MAJOR.MINOR" → "MAJOR.MINOR.0"
    if version.chars().filter(|&c| c == '.').count() == 1 {
        let normalized = format!("{version}.0");
        if let Ok(v) = semver::Version::parse(&normalized) {
            return Ok(v);
        }
    }
    anyhow::bail!("cannot parse '{version}' as semver")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LOCKFILE: &[u8] = include_bytes!("test.gradle.lockfile");

    #[test]
    fn parse_manifest_test() {
        let mut packages: Vec<_> = GradlePackage::parse_manifest(TEST_LOCKFILE)
            .expect("lockfile should parse")
            .collect();
        packages.sort_by(|a, b| a.name.cmp(&b.name));

        let expected = vec![
            GradlePackage {
                name: "com.example:safe-lib".to_string(),
                version: "3.0.0".to_string(),
            },
            GradlePackage {
                name: "org.apache.commons:commons-text".to_string(),
                version: "1.9".to_string(),
            },
            GradlePackage {
                name: "org.apache.logging.log4j:log4j-core".to_string(),
                version: "2.14.1".to_string(),
            },
        ];
        assert_eq!(packages, expected);
    }
}
