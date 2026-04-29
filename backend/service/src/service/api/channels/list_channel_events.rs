use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::{
    db::CoreDb,
    resources::ResourceRegistry,
    service::common::{
        responses::{WithErrorResponses, try_or_return},
        types::{limit::Limit, page::Page, page_info::PageInfo},
    },
    types::{NotificationChannelId, NotificationEvent},
};

/// Response body for listing notification events.
#[derive(Object)]
pub struct NotificationEventList {
    /// Notification events recorded for the workspace, ordered by channel then
    /// ascending delivery time.
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
    /// Returns all notification events recorded for this notification channel.
    #[oai(status = 200)]
    Ok(Json<NotificationEventList>),
}

/// All responses.
pub type AllResponses = WithErrorResponses<Responses>;

pub async fn endpoint(
    notification_id: NotificationChannelId,
    page: Option<Page>,
    limit: Option<Limit>,
) -> AllResponses {
    let core_db = try_or_return!(ResourceRegistry::get::<CoreDb>());

    let page_info = PageInfo {
        page: page.unwrap_or_default(),
        limit: limit.unwrap_or_default(),
    };
    let db_events = try_or_return!(
        core_db
            .get_notification_channel_events(notification_id, page_info)
            .await
    );

    let events = db_events
        .into_iter()
        .map(TryInto::<NotificationEvent>::try_into)
        .collect::<Result<Vec<_>, _>>();
    let events = try_or_return!(events);

    Responses::Ok(Json(NotificationEventList { events, page_info })).into()
}
