use poem_openapi::NewType;

/// Optional label for versioning or environment disambiguation.
#[derive(NewType, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestTag(String);

impl From<String> for ManifestTag {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ManifestTag> for String {
    fn from(value: ManifestTag) -> Self {
        value.0
    }
}

impl std::fmt::Display for ManifestTag {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
