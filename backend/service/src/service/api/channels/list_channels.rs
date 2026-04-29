use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{limit::Limit, page::Page, page_info::PageInfo},
    },
    types::NotificationChannel,
};

/// Response body for listing notification channels.
#[derive(Object)]
pub struct NotificationChannelList {
    /// Notification channels linked to the workspace.
    pub channels: Vec<NotificationChannel>,
    /// Pagination metadata reflecting the requested page and limit.
    #[oai(flatten)]
    pub page_info: PageInfo,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## OK
    ///
    /// Returns all notification channels linked to the workspace.
    #[oai(status = 200)]
    Ok(Json<NotificationChannelList>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    page: Option<Page>,
    limit: Option<Limit>,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    let page_info = PageInfo {
        page: page.unwrap_or_default(),
        limit: limit.unwrap_or_default(),
    };
    let db_channels = try_or_return!(core_db.get_notification_channels(page_info).await);
    let channels = db_channels
        .into_iter()
        .map(TryInto::<NotificationChannel>::try_into)
        .collect::<Result<Vec<_>, _>>();
    let channels = try_or_return!(channels);

    Responses::Ok(Json(NotificationChannelList {
        channels,
        page_info,
    }))
    .into()
}
