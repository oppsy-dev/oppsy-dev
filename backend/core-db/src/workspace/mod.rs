pub mod errors;

use std::collections::HashMap;

use sqlx::Row;

use crate::{
    ConvertTo, CoreDb, Pagination,
    manifest::Manifest,
    notification_channel::{NotificationChannel, NotificationChannelId},
    workspace::errors::{
        AddManifestForWorkspaceError, AddNewWorkspaceError,
        AddNotificationChannelForWorkspaceError, DeleteManifestForWorkspaceError,
        DeleteNotificationChannelForWorkspaceError, DeleteWorkspaceError,
        GetManifestWorkspaceMapError, GetWorkspaceManifestsError,
        GetWorkspaceNotificationChannelsError, GetWorkspacesError, WorkspaceFromRowError,
    },
};

pub type WorkspaceId = uuid::Uuid;
pub type WorkspaceName = String;
pub type ManifestId = uuid::Uuid;
pub type ManifestType = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: WorkspaceName,
    pub manifest_count: u32,
    pub channel_count: u32,
}

/// Trying to read row in the format `(workspaces.id, workspaces.name, manifest_count,
/// channel_count)`
impl TryFrom<sqlx::sqlite::SqliteRow> for Workspace {
    type Error = WorkspaceFromRowError;

    fn try_from(row: sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        let id = row
            .try_get(0)
            .map_err(WorkspaceFromRowError::CannotDecodeId)?;
        let name = row
            .try_get(1)
            .map_err(WorkspaceFromRowError::CannotDecodeName)?;
        let manifest_count = row
            .try_get::<i64, _>(2)
            .and_then(|v| u32::try_from(v).map_err(sqlx::Error::decode))
            .map_err(WorkspaceFromRowError::CannotDecodeManifestCount)?;
        let channel_count = row
            .try_get::<i64, _>(3)
            .and_then(|v| u32::try_from(v).map_err(sqlx::Error::decode))
            .map_err(WorkspaceFromRowError::CannotDecodeChannelCount)?;
        Ok(Workspace {
            id,
            name,
            manifest_count,
            channel_count,
        })
    }
}

impl CoreDb {
    pub async fn add_new_workspace(
        &self,
        id: impl ConvertTo<WorkspaceId>,
        name: impl ConvertTo<WorkspaceName>,
    ) -> Result<(), AddNewWorkspaceError> {
        let id = id.convert()?;
        let name = name.convert()?;
        let res = sqlx::query("INSERT INTO workspaces (id, name) VALUES ($1, $2)")
            .bind(id)
            .bind(&name)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                        AddNewWorkspaceError::AlreadyExists { id }
                    } else {
                        AddNewWorkspaceError::Database(e)
                    }
                } else {
                    AddNewWorkspaceError::Database(e)
                }
            })?;
        if res.rows_affected() != 1 {
            return Err(AddNewWorkspaceError::InvalidAffectedRowsAmount(
                res.rows_affected(),
            ));
        }

        Ok(())
    }

    pub async fn get_workspaces(
        &self,
        pagination: impl ConvertTo<Pagination>,
    ) -> Result<Vec<Workspace>, GetWorkspacesError> {
        let pagination = pagination.convert()?;
        let rows = sqlx::query(
            "SELECT id, name, \
             (SELECT COUNT(*) FROM workspace_manifests WHERE workspace_manifests.workspace_id = workspaces.id) AS manifest_count, \
             (SELECT COUNT(*) FROM workspace_notification_channels WHERE workspace_notification_channels.workspace_id = workspaces.id) AS channel_count \
             FROM workspaces \
             ORDER BY workspaces.id \
             LIMIT $1 OFFSET $2",
        )
        .bind(i64::from(pagination.limit))
        .bind(i64::from(pagination.offset()))
        .fetch_all(&self.pool)
        .await
        .map_err(GetWorkspacesError::Database)?;

        let res = rows
            .into_iter()
            .map(ConvertTo::convert)
            .collect::<Result<_, _>>()?;
        Ok(res)
    }

    pub async fn get_manifest_workspace_map(
        &self
    ) -> Result<HashMap<ManifestId, WorkspaceId>, GetManifestWorkspaceMapError> {
        let rows = sqlx::query("SELECT manifest_id, workspace_id FROM workspace_manifests")
            .fetch_all(&self.pool)
            .await
            .map_err(GetManifestWorkspaceMapError::Database)?;

        rows.into_iter()
            .map(|row| {
                let manifest_id: ManifestId = row
                    .try_get(0)
                    .map_err(GetManifestWorkspaceMapError::Database)?;
                let workspace_id: WorkspaceId = row
                    .try_get(1)
                    .map_err(GetManifestWorkspaceMapError::Database)?;
                Ok((manifest_id, workspace_id))
            })
            .collect()
    }

    pub async fn add_manifest_for_workspace(
        &self,
        workspace_id: impl ConvertTo<WorkspaceId>,
        manifest_id: impl ConvertTo<ManifestId>,
    ) -> Result<(), AddManifestForWorkspaceError> {
        let workspace_id = workspace_id.convert()?;
        let manifest_id = manifest_id.convert()?;
        let res = sqlx::query(
            "INSERT INTO workspace_manifests (workspace_id, manifest_id) VALUES ($1, $2)",
        )
        .bind(workspace_id)
        .bind(manifest_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    AddManifestForWorkspaceError::AlreadyExists {
                        workspace_id,
                        manifest_id,
                    }
                } else {
                    AddManifestForWorkspaceError::Database(e)
                }
            } else {
                AddManifestForWorkspaceError::Database(e)
            }
        })?;
        if res.rows_affected() != 1 {
            return Err(AddManifestForWorkspaceError::InvalidAffectedRowsAmount(
                res.rows_affected(),
            ));
        }

        Ok(())
    }

    pub async fn get_workspace_manifests(
        &self,
        id: impl ConvertTo<WorkspaceId>,
        pagination: impl ConvertTo<Pagination>,
    ) -> Result<Vec<Manifest>, GetWorkspaceManifestsError> {
        let id = id.convert()?;
        let pagination = pagination.convert()?;
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM workspaces WHERE id = $1)")
                .bind(id)
                .fetch_one(&self.pool)
                .await
                .map_err(GetWorkspaceManifestsError::Database)?;
        if !exists {
            return Err(GetWorkspaceManifestsError::NotFound { id });
        }

        let rows = sqlx::query(
            "SELECT manifests.id, manifests.manifest_type, manifests.name, manifests.tag \
             FROM manifests \
             JOIN workspace_manifests ON workspace_manifests.manifest_id = manifests.id \
             WHERE workspace_manifests.workspace_id = $1 \
             ORDER BY manifests.id DESC \
             LIMIT $2 OFFSET $3",
        )
        .bind(id)
        .bind(i64::from(pagination.limit))
        .bind(i64::from(pagination.offset()))
        .fetch_all(&self.pool)
        .await
        .map_err(GetWorkspaceManifestsError::Database)?;

        let res = rows
            .into_iter()
            .map(ConvertTo::convert)
            .collect::<Result<_, _>>()?;
        Ok(res)
    }

    pub async fn delete_manifest_for_workspace(
        &self,
        workspace_id: impl ConvertTo<WorkspaceId>,
        manifest_id: impl ConvertTo<ManifestId>,
    ) -> Result<(), DeleteManifestForWorkspaceError> {
        let workspace_id = workspace_id.convert()?;
        let manifest_id = manifest_id.convert()?;

        let res = sqlx::query(
            "DELETE FROM workspace_manifests WHERE workspace_id = $1 AND manifest_id = $2",
        )
        .bind(workspace_id)
        .bind(manifest_id)
        .execute(&self.pool)
        .await
        .map_err(DeleteManifestForWorkspaceError::Database)?;

        match res.rows_affected() {
            0 => {
                return Err(DeleteManifestForWorkspaceError::NotFound {
                    workspace_id,
                    manifest_id,
                });
            },
            1 => {},
            n => {
                return Err(DeleteManifestForWorkspaceError::InvalidAffectedRowsAmount(
                    n,
                ));
            },
        }

        sqlx::query("DELETE FROM manifests WHERE id = $1")
            .bind(manifest_id)
            .execute(&self.pool)
            .await
            .map_err(DeleteManifestForWorkspaceError::Database)?;

        Ok(())
    }

    pub async fn delete_workspace(
        &self,
        id: impl ConvertTo<WorkspaceId>,
    ) -> Result<(), DeleteWorkspaceError> {
        let id = id.convert()?;
        let res = sqlx::query("DELETE FROM workspaces WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(DeleteWorkspaceError::Database)?;

        match res.rows_affected() {
            0 => Err(DeleteWorkspaceError::NotFound { id }),
            1 => Ok(()),
            n => Err(DeleteWorkspaceError::InvalidAffectedRowsAmount(n)),
        }
    }

    pub async fn add_notification_channel_for_workspace(
        &self,
        workspace_id: impl ConvertTo<WorkspaceId>,
        notification_channel_id: impl ConvertTo<NotificationChannelId>,
    ) -> Result<(), AddNotificationChannelForWorkspaceError> {
        let workspace_id = workspace_id.convert()?;
        let notification_channel_id = notification_channel_id.convert()?;
        let res = sqlx::query(
            "INSERT INTO workspace_notification_channels (workspace_id, channel_id) \
             VALUES ($1, $2)",
        )
        .bind(workspace_id)
        .bind(notification_channel_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    AddNotificationChannelForWorkspaceError::AlreadyExists {
                        workspace_id,
                        notification_channel_id,
                    }
                } else {
                    AddNotificationChannelForWorkspaceError::Database(e)
                }
            } else {
                AddNotificationChannelForWorkspaceError::Database(e)
            }
        })?;

        if res.rows_affected() != 1 {
            return Err(
                AddNotificationChannelForWorkspaceError::InvalidAffectedRowsAmount(
                    res.rows_affected(),
                ),
            );
        }

        Ok(())
    }

    pub async fn get_workspace_notification_channels(
        &self,
        id: impl ConvertTo<WorkspaceId>,
        pagination: impl ConvertTo<Pagination>,
    ) -> Result<Vec<NotificationChannel>, GetWorkspaceNotificationChannelsError> {
        let id = id.convert()?;
        let pagination = pagination.convert()?;
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM workspaces WHERE id = $1)")
                .bind(id)
                .fetch_one(&self.pool)
                .await
                .map_err(GetWorkspaceNotificationChannelsError::Database)?;
        if !exists {
            return Err(GetWorkspaceNotificationChannelsError::NotFound { id });
        }

        let rows = sqlx::query(
            "SELECT notification_channels.id, notification_channels.name, \
             notification_channels.conf, notification_channels.active, \
             (SELECT COUNT(*) FROM notification_events WHERE notification_events.channel_id = notification_channels.id) AS notification_count, \
             (SELECT COUNT(*) FROM workspace_notification_channels WHERE workspace_notification_channels.channel_id = notification_channels.id) AS workspaces_count, \
             (SELECT id FROM notification_events WHERE notification_events.channel_id = notification_channels.id ORDER BY id DESC LIMIT 1) AS latest_event_id \
             FROM notification_channels \
             JOIN workspace_notification_channels \
               ON workspace_notification_channels.channel_id = notification_channels.id \
             WHERE workspace_notification_channels.workspace_id = $1 \
             ORDER BY notification_channels.id \
             LIMIT $2 OFFSET $3",
        )
        .bind(id)
        .bind(i64::from(pagination.limit))
        .bind(i64::from(pagination.offset()))
        .fetch_all(&self.pool)
        .await
        .map_err(GetWorkspaceNotificationChannelsError::Database)?;

        let channels = rows
            .into_iter()
            .map(ConvertTo::convert)
            .collect::<Result<_, _>>()?;
        Ok(channels)
    }

    pub async fn delete_notification_channel_for_workspace(
        &self,
        workspace_id: impl ConvertTo<WorkspaceId>,
        notification_channel_id: impl ConvertTo<NotificationChannelId>,
    ) -> Result<(), DeleteNotificationChannelForWorkspaceError> {
        let workspace_id = workspace_id.convert()?;
        let notification_channel_id = notification_channel_id.convert()?;

        let res = sqlx::query(
            "DELETE FROM workspace_notification_channels \
             WHERE workspace_id = $1 AND notification_channel_id = $2",
        )
        .bind(workspace_id)
        .bind(notification_channel_id)
        .execute(&self.pool)
        .await
        .map_err(DeleteNotificationChannelForWorkspaceError::Database)?;

        match res.rows_affected() {
            0 => {
                return Err(DeleteNotificationChannelForWorkspaceError::NotFound {
                    workspace_id,
                    notification_channel_id,
                });
            },
            1 => {},
            n => {
                return Err(
                    DeleteNotificationChannelForWorkspaceError::InvalidAffectedRowsAmount(n),
                );
            },
        }
        Ok(())
    }
}
