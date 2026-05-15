mod add_manifest;
mod add_workspace_channel;
mod create_workspace;
mod delete_manifest;
mod delete_workspace;
mod delete_workspace_channel;
mod list_manifest_packages;
mod list_manifests;
mod list_workspace_channels;
mod list_workspaces;
// mod upload_manifest;

use poem_openapi::{
    OpenApi,
    param::{Path, Query},
    payload::Json,
};

use crate::{
    service::common::types::{limit::Limit, page::Page},
    types::{Manifest, ManifestId, NotificationChannelId, WorkspaceId},
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

    /// Add a new manifest to a workspace.
    ///
    /// Generates a server-assigned manifest ID, and
    /// runs an OSV scan, and persists detected vulnerabilities.
    #[oai(path = "/v1/workspaces/:workspace_id/manifests", method = "post")]
    async fn add_manifest(
        &self,
        /// Workspace to associate this manifest with.
        workspace_id: Path<WorkspaceId>,
        body: Json<Manifest>,
    ) -> add_manifest::AllResponses {
        add_manifest::endpoint(workspace_id.0, body.0).await
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

    /// List every package parsed from a manifest.
    ///
    /// Returns the full list of dependencies recorded for the manifest, paginated.
    /// Each entry includes the package as it was uploaded plus the OSV IDs that
    /// affect it — empty for packages with no known vulnerabilities. Useful for
    /// confirming what OPPSY parsed from the lock file beyond the vulnerable
    /// subset surfaced by `list_manifests`.
    #[oai(
        path = "/v1/workspaces/:workspace_id/manifests/:manifest_id/packages",
        method = "get"
    )]
    async fn list_manifest_packages(
        &self,
        /// Workspace the manifest belongs to.
        workspace_id: Path<WorkspaceId>,
        /// Manifest to list packages for.
        manifest_id: Path<ManifestId>,
        /// Page number (0-based, default: 0).
        page: Query<Option<Page>>,
        /// Maximum items per page (default: 20).
        limit: Query<Option<Limit>>,
        /// If `true`, only packages with at least one matching OSV record are returned.
        /// Defaults to `false`.
        vulnerable_only: Query<Option<bool>>,
    ) -> list_manifest_packages::AllResponses {
        list_manifest_packages::endpoint(
            workspace_id.0,
            manifest_id.0,
            page.0,
            limit.0,
            vulnerable_only.0,
        )
        .await
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
