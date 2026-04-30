mod common;

use core_db::{
    Pagination,
    manifest::Manifest,
    notification_channel::NotificationChannel,
    workspace::{
        WorkspaceData,
        errors::{
            AddManifestForWorkspaceError, AddNewWorkspaceError,
            AddNotificationChannelForWorkspaceError, DeleteWorkspaceError, GetWorkspaceError,
        },
    },
};

#[tokio::test]
async fn workspace_roundtrip() {
    let db = common::init_db().await;

    let workspace_id = uuid::Uuid::now_v7();
    let name = "workspace";
    db.add_new_workspace(workspace_id, name).await.unwrap();
    assert!(matches!(
        db.add_new_workspace(workspace_id, name).await.unwrap_err(),
        AddNewWorkspaceError::AlreadyExists { .. }
    ));

    db.delete_workspace(workspace_id).await.unwrap();
    assert!(matches!(
        db.delete_workspace(workspace_id).await.unwrap_err(),
        DeleteWorkspaceError::NotFound { .. }
    ));
}

#[tokio::test]
async fn get_workspace_by_id() {
    let db = common::init_db().await;

    let workspace_id = uuid::Uuid::now_v7();
    let name = "my-workspace";
    db.add_new_workspace(workspace_id, name).await.unwrap();

    let data = db.get_workspace(workspace_id).await.unwrap();
    assert_eq!(data, WorkspaceData {
        id: workspace_id,
        name: name.to_string(),
    });

    db.delete_workspace(workspace_id).await.unwrap();
    assert!(matches!(
        db.get_workspace(workspace_id).await.unwrap_err(),
        GetWorkspaceError::NotFound { .. }
    ));
}

#[test_with::file(oppsy.db)]
#[tokio::test]
async fn workspace_with_manifest() {
    let db = common::init_db().await;

    let workspace_id = uuid::Uuid::now_v7();
    db.add_new_workspace(workspace_id, "workspace")
        .await
        .unwrap();

    let manifest_id = uuid::Uuid::now_v7();
    db.add_manifest(manifest_id, "type", "manifest", None)
        .await
        .unwrap();

    db.add_manifest_for_workspace(workspace_id, manifest_id)
        .await
        .unwrap();
    assert!(matches!(
        db.add_manifest_for_workspace(workspace_id, manifest_id)
            .await
            .unwrap_err(),
        AddManifestForWorkspaceError::AlreadyExists { .. }
    ));

    let map = db.get_manifest_workspace_map().await.unwrap();
    assert_eq!(map.get(&manifest_id), Some(&workspace_id));

    let minfests = db
        .get_workspace_manifests(workspace_id, Pagination::all())
        .await
        .unwrap();
    assert_eq!(minfests, vec![Manifest {
        id: manifest_id,
        manifest_type: "type".to_string(),
        name: "manifest".to_string(),
        tag: None
    },]);
}

#[test_with::file(oppsy.db)]
#[tokio::test]
async fn workspace_with_notification_channel() {
    let db = common::init_db().await;

    let workspace_id = uuid::Uuid::now_v7();
    db.add_new_workspace(workspace_id, "workspace")
        .await
        .unwrap();

    let channel_id = uuid::Uuid::now_v7();
    let conf = serde_json::json!({ "url": "https://hooks.example.com/test" });
    db.add_notification_channel(channel_id, "test-channel", conf.clone(), true)
        .await
        .unwrap();

    db.add_notification_channel_for_workspace(workspace_id, channel_id)
        .await
        .unwrap();
    assert!(matches!(
        db.add_notification_channel_for_workspace(workspace_id, channel_id)
            .await
            .unwrap_err(),
        AddNotificationChannelForWorkspaceError::AlreadyExists { .. }
    ));

    let channels = db
        .get_workspace_notification_channels(workspace_id, Pagination::all())
        .await
        .unwrap();
    assert_eq!(channels, vec![NotificationChannel {
        id: channel_id,
        name: "test-channel".to_string(),
        conf,
        active: true,
        events_count: 0,
        workspaces_count: 1,
        latest_event_id: None,
    }]);
}
