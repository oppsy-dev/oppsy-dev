//! OSV API — endpoints for retrieving OSV records.

mod get_record;
pub mod types;

use poem_openapi::{OpenApi, param::Path};

/// API for retrieving OSV records
pub struct Api;

#[OpenApi]
impl Api {
    /// Returns an OSV record, by the provided id of that record e.g. GHSA-2cgv-28vr-rv6j
    ///
    /// <https://ossf.github.io/osv-schema/>
    #[oai(path = "/v1/osv/:record_id", method = "get")]
    #[allow(clippy::unused_async)]
    async fn get_record(
        &self,
        /// Workspace to register the webhook for.
        record_id: Path<get_record::OsvRecordId>,
    ) -> get_record::AllResponses {
        get_record::endpoint(&record_id.0)
    }
}
