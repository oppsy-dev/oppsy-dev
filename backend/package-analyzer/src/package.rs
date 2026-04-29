use std::{fmt::Debug, hash::Hash};

use osv_db::types::{Affected, Ecosystem, PackageName};

/// Abstraction over a single package entry parsed from a manifest file (e.g.
/// `Cargo.lock`, `package-lock.json`).
///
/// Implementors are responsible for:
/// - identifying the package by name within a specific ecosystem,
/// - determining whether a given [`OsvRecord`] affects their version, and
/// - parsing a raw manifest file into an iterator of package entries.
pub trait Package: Hash + PartialEq + Eq + Clone + Debug {
    /// Returns a [`Package`] name.
    fn name(&self) -> PackageName;

    /// Implements the evaluation algorithm from <https://ossf.github.io/osv-schema/#evaluation>.
    fn evaluate(
        &self,
        affected: &Affected,
    ) -> anyhow::Result<bool>;

    /// Parses a raw manifest file into an iterator of [`Package`] entries.
    ///
    /// # Errors
    ///
    /// - Returns an error if cannot parse `manifest_bytes`.
    fn parse_manifest(manifest_bytes: &[u8]) -> anyhow::Result<impl Iterator<Item = Self>>
    where Self: Sized;

    /// Returns `true` if the provided [`Ecosystem`] matches for the [`Package`]
    /// definition
    fn matches_ecosystem(eco: Ecosystem) -> bool;
}
