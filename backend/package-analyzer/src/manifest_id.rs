use std::ops::Deref;

/// A user-defined identifier for a manifest file.
///
/// This can be anything meaningful to the caller, such as a package name combined
/// with a version (e.g. `"my-app@1.2.3"`), a git commit hash, a file path, or
/// any other string that uniquely identifies the manifest within the caller's context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestId(String);

impl Deref for ManifestId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for ManifestId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for ManifestId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
