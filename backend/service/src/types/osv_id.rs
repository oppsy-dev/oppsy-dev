use poem_openapi::{NewType, types::Example};

/// OSV vulnerability identifier (e.g. "GHSA-xxxx-xxxx-xxxx" or "CVE-xxxx-xxxx").
#[derive(NewType, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsvId(String);

impl Example for OsvId {
    fn example() -> Self {
        Self("GHSA-jw36-hf63-69r9".to_string())
    }
}

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
