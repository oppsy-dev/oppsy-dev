//! Page query parameter type.

use poem_openapi::{NewType, types::Example};

/// 1-based page number for paginated queries.
#[derive(NewType, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[oai(example)]
pub struct Page(u32);

impl From<Page> for u32 {
    fn from(value: Page) -> Self {
        value.0
    }
}

impl Example for Page {
    fn example() -> Self {
        Self::default()
    }
}
