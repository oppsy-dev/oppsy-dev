//! Data types for parsing Gradle dependency lock files.
//!
//! Gradle generates lock files when dependency locking is enabled. Each
//! configuration produces a file under `gradle/dependency-locks/`, or a single
//! `gradle.lockfile` can be used in legacy mode. The format is:
//!
//! ```text
//! # This is a Gradle generated file for dependency locking.
//! # Manual edits can break the build and are not recommended.
//! # This file is expected to be part of source control.
//! org.apache.commons:commons-text:1.9=compileClasspath,runtimeClasspath
//! org.apache.logging.log4j:log4j-core:2.14.1=compileClasspath,runtimeClasspath
//! empty=
//! ```
//!
//! Each non-comment line follows the pattern:
//!
//! ```text
//! groupId:artifactId:version=configurations
//! ```
//!
//! Lines starting with `#` are comments. The special `empty=` line indicates no
//! dependencies and is skipped. The resulting [`GradlePackage`] uses
//! `groupId:artifactId` as the package name, matching the Maven coordinate format
//! expected by the OSV database.

use osv_db::types::PackageName;

/// A resolved Gradle dependency with a `groupId:artifactId` name and version string.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GradlePackage {
    /// Package name in Maven coordinates format: `groupId:artifactId`.
    pub name: PackageName,
    /// Maven version string (e.g. `"1.9"`, `"2.14.1"`, `"3.0.0-SNAPSHOT"`).
    pub version: String,
}

/// Parses a single line from a Gradle lock file into a [`GradlePackage`].
///
/// Returns `None` for comment lines (`#`), the `empty=` sentinel, and any line
/// that does not match the expected `groupId:artifactId:version=...` structure.
pub(super) fn parse_lockfile_line(line: &str) -> Option<GradlePackage> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') || line == "empty=" {
        return None;
    }

    // Split off the configurations suffix (everything after `=`).
    let (coordinates, _configurations) = line.split_once('=')?;

    // Remaining format: groupId:artifactId:version
    let mut parts = coordinates.splitn(3, ':');
    let group_id = parts.next()?.trim();
    let artifact_id = parts.next()?.trim();
    let version = parts.next()?.trim();

    if group_id.is_empty() || artifact_id.is_empty() || version.is_empty() {
        return None;
    }

    Some(GradlePackage {
        name: format!("{group_id}:{artifact_id}"),
        version: version.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_standard_line() {
        let pkg = parse_lockfile_line(
            "org.apache.commons:commons-text:1.9=compileClasspath,runtimeClasspath",
        )
        .unwrap();
        assert_eq!(pkg.name, "org.apache.commons:commons-text");
        assert_eq!(pkg.version, "1.9");
    }

    #[test]
    fn skip_comment_and_empty_lines() {
        assert!(parse_lockfile_line("# a comment").is_none());
        assert!(parse_lockfile_line("empty=").is_none());
        assert!(parse_lockfile_line("").is_none());
    }

    #[test]
    fn skip_missing_version() {
        assert!(parse_lockfile_line("group:artifact=runtimeClasspath").is_none());
    }
}
