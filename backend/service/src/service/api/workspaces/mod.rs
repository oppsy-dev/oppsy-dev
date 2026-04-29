mod add_workspace_channel;
mod create_manifest;
mod create_workspace;
mod delete_manifest;
mod delete_workspace;
mod delete_workspace_channel;
mod get_manifest_raw;
mod list_manifests;
mod list_workspace_channels;
mod list_workspaces;
mod upload_manifest;

use poem::{Body, error::ReadBodyError};
use poem_openapi::{
    OpenApi,
    param::{Path, Query},
    payload::{Binary, Json},
};
use upload_manifest::MAXIMUM_MANIFEST_SIZE;

use crate::{
    service::common::types::{limit::Limit, page::Page},
    types::{ManifestId, NotificationChannelId, WorkspaceId},
};

/// Workspace API — endpoints for managing workspaces and lock file manifests.
pub struct Api;

#[OpenApi]
impl Api {
    /// Create a new workspace.
    ///
    /// The server assigns a UUID v7 workspace ID and returns it in the response
    /// body. Clients must use this ID to associate manifests and webhooks with
    /// the workspace.
    #[oai(path = "/v1/workspaces", method = "post")]
    async fn create_workspace(
        &self,
        body: Json<create_workspace::CreateWorkspaceRequest>,
    ) -> create_workspace::AllResponses {
        create_workspace::endpoint(body.0).await
    }

    /// List workspaces.
    #[oai(path = "/v1/workspaces", method = "get")]
    async fn list_workspaces(
        &self,
        /// Page number (1-based, default: 1).
        page: Query<Option<Page>>,
        /// Maximum items per page (default: 20).
        limit: Query<Option<Limit>>,
    ) -> list_workspaces::AllResponses {
        list_workspaces::endpoint(page.0, limit.0).await
    }

    /// Delete a workspace.
    ///
    /// Permanently removes the workspace and all associated data.
    #[oai(path = "/v1/workspaces/:workspace_id", method = "delete")]
    async fn delete_workspace(
        &self,
        /// Workspace to delete.
        workspace_id: Path<WorkspaceId>,
    ) -> delete_workspace::AllResponses {
        delete_workspace::endpoint(workspace_id.0).await
    }

    /// Reserve a manifest slot for a workspace.
    ///
    /// Validates workspace access, generates a server-assigned manifest ID, and
    /// records the declared ecosystem type. The caller must then upload the raw
    /// lock file via `PUT /v1/workspaces/{workspace_id}/manifests/{manifest_id}`.
    #[oai(path = "/v1/workspaces/:workspace_id/manifests", method = "post")]
    async fn create_manifest(
        &self,
        /// Workspace to associate this manifest with.
        workspace_id: Path<WorkspaceId>,
        body: Json<create_manifest::CreateManifestRequest>,
    ) -> create_manifest::AllResponses {
        create_manifest::endpoint(workspace_id.0, body.0).await
    }

    /// Upload a lock file manifest to be scanned for vulnerabilities.
    ///
    /// Accepts raw binary lock file bytes for a manifest ID previously reserved
    /// via `POST /v1/workspaces/{workspace_id}/manifests`. Stores the file, runs
    /// an OSV scan, and persists detected vulnerabilities.
    #[oai(
        path = "/v1/workspaces/:workspace_id/manifests/:manifest_id",
        method = "put"
    )]
    async fn upload_manifest(
        &self,
        /// Workspace the manifest belongs to.
        workspace_id: Path<WorkspaceId>,
        /// Manifest ID returned by the reservation step.
        manifest_id: Path<ManifestId>,
        /// Raw binary content of the lock file.
        manifest: Binary<Body>,
    ) -> upload_manifest::AllResponses {
        match manifest.0.into_bytes_limit(MAXIMUM_MANIFEST_SIZE).await {
            Ok(manifest_bytes) => {
                upload_manifest::endpoint(workspace_id.0, manifest_id.0, manifest_bytes).await
            },
            Err(ReadBodyError::PayloadTooLarge) => {
                upload_manifest::Responses::PayloadTooLarge.into()
            },
            Err(_) => {
                upload_manifest::Responses::UnprocessableContent(Json(
                    "Failed to read manifest from the request".into(),
                ))
                .into()
            },
        }
    }

    /// List manifests belonging to a workspace.
    ///
    /// Returns metadata and detected vulnerabilities for every uploaded manifest
    /// in the workspace. Results are ordered from newest to oldest.
    #[oai(path = "/v1/workspaces/:workspace_id/manifests", method = "get")]
    async fn list_manifests(
        &self,
        /// Workspace to list manifests for.
        workspace_id: Path<WorkspaceId>,
        /// Page number (1-based, default: 1).
        page: Query<Option<Page>>,
        /// Maximum items per page (default: 20).
        limit: Query<Option<Limit>>,
    ) -> list_manifests::AllResponses {
        list_manifests::endpoint(workspace_id.0, page.0, limit.0).await
    }

    /// Download the raw bytes of a previously uploaded lock file manifest.
    ///
    /// Returns the raw binary content of the manifest. The manifest must belong
    /// to the given workspace.
    #[oai(
        path = "/v1/workspaces/:workspace_id/manifests/:manifest_id/raw",
        method = "get"
    )]
    async fn get_manifest_raw(
        &self,
        /// Workspace the manifest belongs to.
        workspace_id: Path<WorkspaceId>,
        /// Manifest ID to download.
        manifest_id: Path<ManifestId>,
    ) -> get_manifest_raw::AllResponses {
        get_manifest_raw::endpoint(workspace_id.0, manifest_id.0).await
    }

    /// Delete a manifest from a workspace.
    ///
    /// Permanently removes the manifest and its associated data from the workspace.
    ///
    /// **Warning:** this endpoint does not remove the raw lock file from blob
    /// storage. The underlying file will remain until it is cleaned up separately.
    #[oai(
        path = "/v1/workspaces/:workspace_id/manifests/:manifest_id",
        method = "delete"
    )]
    async fn delete_manifest(
        &self,
        /// Workspace the manifest belongs to.
        workspace_id: Path<WorkspaceId>,
        /// Manifest to delete.
        manifest_id: Path<ManifestId>,
    ) -> delete_manifest::AllResponses {
        delete_manifest::endpoint(workspace_id.0, manifest_id.0).await
    }

    /// Link a notification channel to a workspace.
    ///
    /// Associates an existing notification channel with the workspace so that
    /// workspace scan results are delivered through it.
    #[oai(path = "/v1/workspaces/:workspace_id/channels", method = "post")]
    async fn add_workspace_channel(
        &self,
        /// Workspace to link the channel to.
        workspace_id: Path<WorkspaceId>,
        body: Json<add_workspace_channel::AddWorkspaceChannelRequest>,
    ) -> add_workspace_channel::AllResponses {
        add_workspace_channel::endpoint(workspace_id.0, body.0).await
    }

    /// List notification channels linked to a workspace.
    ///
    /// Returns all notification channels that have been associated with the
    /// workspace.
    #[oai(path = "/v1/workspaces/:workspace_id/channels", method = "get")]
    async fn list_workspace_channels(
        &self,
        /// Workspace to list channels for.
        workspace_id: Path<WorkspaceId>,
        /// Page number (1-based, default: 1).
        page: Query<Option<Page>>,
        /// Maximum items per page (default: 20).
        limit: Query<Option<Limit>>,
    ) -> list_workspace_channels::AllResponses {
        list_workspace_channels::endpoint(workspace_id.0, page.0, limit.0).await
    }

    /// Remove a notification channel from a workspace.
    ///
    /// Unlinks the notification channel from the workspace and permanently
    /// deletes the channel record.
    #[oai(
        path = "/v1/workspaces/:workspace_id/channels/:channel_id",
        method = "delete"
    )]
    async fn delete_workspace_channel(
        &self,
        /// Workspace the channel belongs to.
        workspace_id: Path<WorkspaceId>,
        /// Notification channel to remove.
        channel_id: Path<NotificationChannelId>,
    ) -> delete_workspace_channel::AllResponses {
        delete_workspace_channel::endpoint(workspace_id.0, channel_id.0).await
    }
}
