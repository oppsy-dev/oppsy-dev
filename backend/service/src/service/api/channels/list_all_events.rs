use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{limit::Limit, page::Page, page_info::PageInfo},
    },
    types::NotificationEvent,
};

/// Response body for listing all notification events across all channels.
#[derive(Object)]
pub struct AllNotificationEventList {
    /// Notification events recorded across all channels, ordered newest first.
    pub events: Vec<NotificationEvent>,
    /// Pagination metadata reflecting the requested page and limit.
    #[oai(flatten)]
    pub page_info: PageInfo,
}

/// Endpoint responses.
#[derive(ApiResponse)]
pub enum Responses {
    /// ## OK
    ///
    /// Returns all notification events recorded across all notification channels.
    #[oai(status = 200)]
    Ok(Json<AllNotificationEventList>),
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
    let db_events = try_or_return!(core_db.get_all_notification_events(page_info).await);

    let events = db_events
        .into_iter()
        .map(TryInto::<NotificationEvent>::try_into)
        .collect::<Result<Vec<_>, _>>();
    let events = try_or_return!(events);

    Responses::Ok(Json(AllNotificationEventList { events, page_info })).into()
}
