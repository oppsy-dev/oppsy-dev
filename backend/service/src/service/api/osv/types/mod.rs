//! poem-openapi response types mirroring the OSV schema.
//!
//! `From` impls on each type convert from `osv_db::types` at the response boundary.

mod affected;
mod credit;
mod package;
mod range;
mod record;
mod reference;
mod severity;

pub use affected::Affected;
pub use credit::Credit;
pub use package::Package;
pub use range::Range;
pub use record::OsvRecord;
pub use reference::Reference;
pub use severity::Severity;
