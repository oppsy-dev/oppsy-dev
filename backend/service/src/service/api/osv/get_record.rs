use poem_openapi::{ApiResponse, NewType, payload::Json, types::Example};

use super::types::OsvRecord;
use crate::{
    db::OsvDb,
    resources::ResourceRegistry,
    service::common::responses::{WithErrorResponses, try_or_return},
};

#[derive(Debug, NewType)]
#[oai(example = true)]
pub struct OsvRecordId(osv_db::types::OsvRecordId);

impl Example for OsvRecordId {
    fn example() -> Self {
        OsvRecordId("GHSA-2cgv-28vr-rv6j".to_string())
    }
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Response {
    /// ## OSV Record
    ///
    /// Returns an OSV record defined by this schema <https://ossf.github.io/osv-schema/>.
    #[oai(status = 200)]
    Ok(Json<Box<OsvRecord>>),

    /// ## Not Found
    ///
    /// OSV record not found.
    #[oai(status = 404)]
    NotFound,
}

/// All responses.
pub type AllResponses = WithErrorResponses<Response>;

pub fn endpoint(id: &OsvRecordId) -> AllResponses {
    let osv_db = try_or_return!(ResourceRegistry::get::<OsvDb>());
    if let Some(record) = try_or_return!(osv_db.get_record(&id.0)) {
        Response::Ok(Json(Box::new(OsvRecord::from(record)))).into()
    } else {
        Response::NotFound.into()
    }
}
