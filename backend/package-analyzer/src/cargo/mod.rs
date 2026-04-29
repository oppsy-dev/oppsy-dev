mod types;

use osv_db::types::{Affected, Ecosystem, PackageName, RangeType};
use types::CargoLock;
pub use types::CargoPackage;

use crate::{Package, semver_eval};

impl Package for CargoPackage {
    fn name(&self) -> PackageName {
        self.name.clone()
    }

    fn parse_manifest(manifest_bytes: &[u8]) -> anyhow::Result<impl Iterator<Item = Self>>
    where Self: Sized {
        let manifest_str = std::str::from_utf8(manifest_bytes)?;
        let lockfile: CargoLock = toml::from_str(manifest_str)?;
        Ok(lockfile.packages.into_iter())
    }

    fn matches_ecosystem(eco: Ecosystem) -> bool {
        matches!(eco, Ecosystem::CratesIo)
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

    const TEST_CARGO_LOCK: &[u8] = include_bytes!("test.Cargo.lock");

    #[test]
    fn parse_manifest_test() {
        let packages: Vec<_> = CargoPackage::parse_manifest(TEST_CARGO_LOCK)
            .expect("lockfile should parse")
            .collect();

        assert_eq!(packages, vec![
            CargoPackage {
                name: "anyhow".to_string(),
                version: "1.0.95".parse().unwrap(),
                source: Some("registry+https://github.com/rust-lang/crates.io-index".to_string()),
                checksum: Some(
                    "34ac096ce696dc2fcabef30516bb13c0a68a11d30131d3df6f04711467681b04".to_string()
                ),
            },
            CargoPackage {
                name: "serde".to_string(),
                version: "1.0.217".parse().unwrap(),
                source: Some("registry+https://github.com/rust-lang/crates.io-index".to_string()),
                checksum: Some(
                    "02fc353d706b7b4f42f6ac7c3c36c56e29fd91c2d7f1e0fd2c7551e7d3bd0bdb".to_string()
                ),
            },
            CargoPackage {
                name: "serde_json".to_string(),
                version: "1.0.138".parse().unwrap(),
                source: Some("registry+https://github.com/rust-lang/crates.io-index".to_string()),
                checksum: Some(
                    "d434192e7da787e94a6ea7e9670b26a036d0ca41e0b7efb2676dd32bae872949".to_string()
                ),
            },
            CargoPackage {
                name: "thiserror".to_string(),
                version: "2.0.11".parse().unwrap(),
                source: Some("registry+https://github.com/rust-lang/crates.io-index".to_string()),
                checksum: Some(
                    "d452f284b73e6d76dd36758a0c8684b1d5be31f92b89d07fd5822175732206fc".to_string()
                ),
            },
            CargoPackage {
                name: "tokio".to_string(),
                version: "1.43.0".parse().unwrap(),
                source: Some("registry+https://github.com/rust-lang/crates.io-index".to_string()),
                checksum: Some(
                    "3d61fa4ffa3de412bfea335c6ecff681de2b609ba3c77ef3e00e521813a9ed9e".to_string()
                ),
            },
        ]);
    }
}
