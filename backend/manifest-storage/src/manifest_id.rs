use std::path::{Path, PathBuf};

use thiserror::Error;

/// A validated identifier for a manifest, used as the filename on disk.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestId(PathBuf);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestIdError {
    /// Rejected if the value is empty, contains path separators, or is not a plain
    /// filename.
    #[error(
        "'{0}' is not a valid manifest id — must be a single filename component with no path separators"
    )]
    NotAFileName(String),
}

impl ManifestId {
    pub fn new(value: &str) -> Result<Self, ManifestIdError> {
        let path = PathBuf::from(value);
        let mut components = path.components();
        match components.next() {
            Some(std::path::Component::Normal(_)) if components.next().is_none() => Ok(Self(path)),
            _ => Err(ManifestIdError::NotAFileName(value.to_string())),
        }
    }

    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl std::fmt::Display for ManifestId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

impl TryFrom<String> for ManifestId {
    type Error = ManifestIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        assert!(ManifestId::new("").is_err());
    }

    #[test]
    fn rejects_path_traversal() {
        assert!(ManifestId::new("../secret").is_err());
    }

    #[test]
    fn rejects_path_separator() {
        assert!(ManifestId::new("foo/bar").is_err());
    }

    #[test]
    fn accepts_valid_id() {
        assert!(ManifestId::new("my-manifest_v1.0").is_ok());
    }
}
