mod types;

use osv_db::types::{Affected, Ecosystem, PackageName, RangeType};
pub use types::{UvLock, UvPackage, UvSource};

use crate::{Package, semver_eval};

impl Package for UvPackage {
    fn name(&self) -> PackageName {
        self.name.clone()
    }

    fn matches_ecosystem(eco: Ecosystem) -> bool {
        matches!(eco, Ecosystem::PyPI)
    }

    fn parse_manifest(manifest_bytes: &[u8]) -> anyhow::Result<impl Iterator<Item = Self>>
    where Self: Sized {
        let manifest_str = std::str::from_utf8(manifest_bytes)?;
        let lockfile: UvLock = toml::from_str(manifest_str)?;
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

    const TEST_UV_LOCK: &[u8] = include_bytes!("test.uv.lock");

    #[test]
    fn parse_manifest_test() {
        let packages: Vec<_> = UvPackage::parse_manifest(TEST_UV_LOCK)
            .expect("lockfile should parse")
            .collect();

        assert_eq!(packages, vec![
            UvPackage {
                name: "my-app".to_string(),
                version: "0.1.0".parse().unwrap(),
                source: UvSource::Virtual {
                    virtual_path: ".".to_string()
                }
            },
            UvPackage {
                name: "requests".to_string(),
                version: "2.31.0".parse().unwrap(),
                source: UvSource::Registry {
                    registry: "https://pypi.org/simple".to_string()
                }
            },
            UvPackage {
                name: "urllib3".to_string(),
                version: "2.2.1".parse().unwrap(),
                source: UvSource::Registry {
                    registry: "https://pypi.org/simple".to_string()
                }
            },
            UvPackage {
                name: "my-local-lib".to_string(),
                version: "0.3.0".parse().unwrap(),
                source: UvSource::Path {
                    path: "../my-local-lib".to_string()
                }
            },
            UvPackage {
                name: "my-git-dep".to_string(),
                version: "1.0.0".parse().unwrap(),
                source: UvSource::Git {
                    git: "https://github.com/example/repo".to_string()
                }
            }
        ]);
    }
}
