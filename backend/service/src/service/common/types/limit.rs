//! Limit query parameter type.

use poem_openapi::{NewType, types::Example};

/// Maximum number of items returned per page.
#[derive(NewType, Debug, Clone, Copy, PartialEq, Eq)]
#[oai(example)]
pub struct Limit(u32);

impl From<Limit> for u32 {
    fn from(value: Limit) -> Self {
        value.0
    }
}

impl Default for Limit {
    fn default() -> Self {
        Self(20)
    }
}

impl Example for Limit {
    fn example() -> Self {
        Self::default()
    }
}
