//! Data types for parsing Maven dependency list files.
//!
//! A Maven dependency list is produced by running:
//!
//! ```sh
//! mvn dependency:list -DoutputFile=dependencies.txt -DincludeScope=runtime
//! ```
//!
//! Each line in the file follows the format:
//!
//! ```text
//! groupId:artifactId:packaging:version:scope
//! ```
//!
//! For example:
//!
//! ```text
//! org.apache.commons:commons-text:jar:1.9:compile
//! org.apache.logging.log4j:log4j-core:jar:2.14.1:compile
//! ```
//!
//! Lines starting with `#` or `[` (Maven log prefixes) and blank lines are skipped.
//! The resulting [`MavenPackage`] uses `groupId:artifactId` as the package name,
//! matching the Maven coordinate format expected by the OSV database.

use osv_db::types::PackageName;

/// A resolved Maven dependency with a `groupId:artifactId` name and version string.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MavenPackage {
    /// Package name in Maven coordinates format: `groupId:artifactId`.
    pub name: PackageName,
    /// Maven version string (e.g. `"1.9"`, `"2.14.1"`, `"3.0.0-SNAPSHOT"`).
    pub version: String,
}

/// Parses a single line from a Maven dependency list file into a [`MavenPackage`].
///
/// Returns `None` for comment lines (`#`), Maven log lines (`[`), and blank lines.
/// Also returns `None` if the line does not contain at least five colon-separated fields.
pub(super) fn parse_dependency_line(line: &str) -> Option<MavenPackage> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
        return None;
    }

    // Expected format: groupId:artifactId:packaging:version:scope
    let mut parts = line.splitn(5, ':');
    let group_id = parts.next()?.trim();
    let artifact_id = parts.next()?.trim();
    let _packaging = parts.next()?;
    let version = parts.next()?.trim();
    // The fifth field (scope) is present but not needed.
    let _scope = parts.next()?;

    if group_id.is_empty() || artifact_id.is_empty() || version.is_empty() {
        return None;
    }

    Some(MavenPackage {
        name: format!("{group_id}:{artifact_id}"),
        version: version.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_standard_line() {
        let pkg =
            parse_dependency_line("   org.apache.commons:commons-text:jar:1.9:compile   ").unwrap();
        assert_eq!(pkg.name, "org.apache.commons:commons-text");
        assert_eq!(pkg.version, "1.9");
    }

    #[test]
    fn skip_comment_lines() {
        assert!(parse_dependency_line("# a comment").is_none());
        assert!(parse_dependency_line("[INFO] something").is_none());
        assert!(parse_dependency_line("").is_none());
    }

    #[test]
    fn skip_incomplete_lines() {
        assert!(parse_dependency_line("groupId:artifactId:jar:1.0").is_none());
    }
}
