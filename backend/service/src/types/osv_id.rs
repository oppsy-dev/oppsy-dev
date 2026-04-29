use poem_openapi::NewType;

/// OSV vulnerability identifier (e.g. "GHSA-xxxx-xxxx-xxxx" or "CVE-xxxx-xxxx").
#[derive(NewType, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsvId(String);

impl From<String> for OsvId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<OsvId> for String {
    fn from(value: OsvId) -> Self {
        value.0
    }
}

impl std::fmt::Display for OsvId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
