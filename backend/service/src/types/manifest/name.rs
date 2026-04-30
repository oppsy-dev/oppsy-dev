use poem_openapi::NewType;

/// Human-readable name for a manifest (e.g. the filename or repo path).
#[derive(NewType, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestName(String);

impl From<String> for ManifestName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ManifestName> for String {
    fn from(value: ManifestName) -> Self {
        value.0
    }
}

impl std::fmt::Display for ManifestName {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
