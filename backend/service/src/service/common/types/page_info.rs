use poem_openapi::{Object, types::Example};

use super::{limit::Limit, page::Page};

/// Pagination metadata echoed back in every paginated response.
#[derive(Object, Debug, Clone, Copy)]
#[oai(example)]
pub struct PageInfo {
    /// Current page number (0-based).
    pub page: Page,
    /// Maximum number of items per page.
    pub limit: Limit,
}

impl From<PageInfo> for core_db::Pagination {
    fn from(value: PageInfo) -> Self {
        Self {
            page: value.page.into(),
            limit: value.limit.into(),
        }
    }
}

impl Example for PageInfo {
    fn example() -> Self {
        Self {
            page: Page::example(),
            limit: Limit::example(),
        }
    }
}
