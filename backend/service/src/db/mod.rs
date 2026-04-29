//! Stateful storage layer for the service.

pub mod core_db;
pub mod manifest_db;
pub mod osv_db;

pub use core_db::CoreDb;
pub use manifest_db::ManifestDb;
pub use osv_db::OsvDb;
