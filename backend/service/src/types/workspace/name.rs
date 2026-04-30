use poem_openapi::NewType;

/// Display name of a workspace.
#[derive(NewType, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkspaceName(String);

impl From<String> for WorkspaceName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<WorkspaceName> for String {
    fn from(value: WorkspaceName) -> Self {
        value.0
    }
}

impl std::fmt::Display for WorkspaceName {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
