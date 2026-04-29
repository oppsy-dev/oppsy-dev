mod common;

use core_db::{
    Pagination,
    notification_channel::{NotificationChannel, errors::AddNotificationChannelError},
    notification_event::{
        NotificationEvent,
        errors::{AddNotificationEventError, GetChannelNotificationEventsError},
    },
};

#[tokio::test]
async fn notification_channel_roundtrip() {
    let db = common::init_db().await;

    let channel_id = uuid::Uuid::now_v7();
    db.add_notification_channel(
        channel_id,
        "my-slack".to_string(),
        serde_json::json!({"webhook_url":"https://hooks.slack.com/test"}),
        true,
    )
    .await
    .unwrap();

    let channels = db
        .get_notification_channels(Pagination::all())
        .await
        .unwrap();
    let found = channels
        .iter()
        .find(|c| c.id == channel_id)
        .expect("inserted channel must appear in get_notification_channels");
    assert_eq!(found, &NotificationChannel {
        id: channel_id,
        name: "my-slack".to_string(),
        conf: serde_json::json!({"webhook_url": "https://hooks.slack.com/test"}),
        active: true,
        events_count: 0,
        workspaces_count: 0,
        latest_event_id: None,
    });

    assert!(matches!(
        db.add_notification_channel(channel_id, "dup".to_string(), serde_json::json!({}), true,)
            .await
            .unwrap_err(),
        AddNotificationChannelError::AlreadyExists { .. }
    ));

    // duplicate insert must not add a new entry
    let channels_after_dup = db
        .get_notification_channels(Pagination::all())
        .await
        .unwrap();
    assert_eq!(
        channels_after_dup
            .iter()
            .filter(|c| c.id == channel_id)
            .count(),
        1,
    );
}

#[test_with::file(oppsy.db)]
#[tokio::test]
async fn notification_events_roundtrip() {
    let db = common::init_db().await;

    let channel_id = uuid::Uuid::now_v7();
    db.add_notification_channel(
        channel_id,
        "email-ops".to_string(),
        serde_json::json!({"to":"ops@example.com"}),
        true,
    )
    .await
    .unwrap();

    let event_id_ok = uuid::Uuid::now_v7();
    db.add_notification_channel_events(vec![NotificationEvent {
        id: event_id_ok,
        channel_id,
        error: None,
        meta: serde_json::json!("GHSA-1234-5678-abcd"),
    }])
    .await
    .unwrap();

    let event_id_err = uuid::Uuid::now_v7();
    db.add_notification_channel_events(vec![NotificationEvent {
        id: event_id_err,
        channel_id,
        error: Some("connection timeout".to_string()),
        meta: serde_json::json!("GHSA-9999-0000-efgh"),
    }])
    .await
    .unwrap();

    assert!(matches!(
        db.add_notification_channel_events(vec![NotificationEvent {
            id: event_id_ok,
            channel_id,
            error: None,
            meta: serde_json::json!({}),
        }])
        .await
        .unwrap_err(),
        AddNotificationEventError::AlreadyExists
    ));

    let events = db
        .get_notification_channel_events(channel_id, Pagination::all())
        .await
        .unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].error, Some("connection timeout".to_string()));
    assert_eq!(events[1], NotificationEvent {
        id: event_id_ok,
        channel_id,
        error: None,
        meta: serde_json::json!("GHSA-1234-5678-abcd"),
    });

    let missing_channel = uuid::Uuid::now_v7();
    assert!(matches!(
        db.get_notification_channel_events(missing_channel, Pagination::all())
            .await
            .unwrap_err(),
        GetChannelNotificationEventsError::ChannelNotFound { .. }
    ));
}
