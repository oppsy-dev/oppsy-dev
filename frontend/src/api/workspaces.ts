import { del, get, post, putBinary } from './client';
import type { PaginationParams } from './client';
import { components, paths } from './schema';
import { NotificationChannelId } from './notification_channels';

// --- Types ---

type V1WorkspacesGetResp =
  paths['/v1/workspaces']['get']['responses']['200']['content']['application/json; charset=utf-8'];

type V1WorkspacesPostReq =
  paths['/v1/workspaces']['post']['requestBody']['content']['application/json; charset=utf-8'];

type V1ManifestsGetResp =
  paths['/v1/workspaces/{workspace_id}/manifests']['get']['responses']['200']['content']['application/json; charset=utf-8'];

type V1WorkspaceChannelsGetResp =
  paths['/v1/workspaces/{workspace_id}/channels']['get']['responses']['200']['content']['application/json; charset=utf-8'];

export type WorkspaceInfo = components['schemas']['WorkspaceInfo'];
export type WorkspaceId = WorkspaceInfo['id'];
export type CreateWorkspaceRequest = V1WorkspacesPostReq;
export type Manifest = V1ManifestsGetResp['manifests'][number];
export type ManifestId = Manifest['id'];
export type ManifestType = components['schemas']['ManifestType'];
export type CreateManifestRequest = components['schemas']['CreateManifestRequest'];
export type { PaginationParams } from './client';
export type { NotificationChannel, NotificationChannelId } from './notification_channels';

// --- Workspaces ---

export async function fetchWorkspaces(params: PaginationParams = {}): Promise<V1WorkspacesGetResp> {
  try {
    const res = await get('/v1/workspaces', {
      page: params.page?.toString(),
      limit: params.limit?.toString(),
    });
    return (await res.json()) as V1WorkspacesGetResp;
  } catch (err) {
    throw err;
  }
}

export async function createWorkspace(req: CreateWorkspaceRequest): Promise<WorkspaceInfo> {
  try {
    const res = await post('/v1/workspaces', req);
    return (await res.json()) as WorkspaceInfo;
  } catch (err) {
    throw err;
  }
}

export async function deleteWorkspace(workspaceId: WorkspaceId): Promise<void> {
  try {
    await del(`/v1/workspaces/${workspaceId}`);
  } catch (err) {
    throw err;
  }
}

// --- Manifests ---

export async function fetchManifests(
  workspaceId: WorkspaceId,
  params: PaginationParams = {},
): Promise<V1ManifestsGetResp> {
  try {
    const res = await get(`/v1/workspaces/${workspaceId}/manifests`, {
      page: params.page?.toString(),
      limit: params.limit?.toString(),
    });
    return (await res.json()) as V1ManifestsGetResp;
  } catch (err) {
    throw err;
  }
}

export type UploadManifestInput = CreateManifestRequest & { file: File };

export async function uploadManifest(
  workspaceId: WorkspaceId,
  { file, ...req }: UploadManifestInput,
): Promise<string> {
  try {
    const createRes = await post(`/v1/workspaces/${workspaceId}/manifests`, req);
    const manifestId = (await createRes.json()) as string;
    const bytes = await file.arrayBuffer();
    await putBinary(`/v1/workspaces/${workspaceId}/manifests/${manifestId}`, bytes);
    return manifestId;
  } catch (err) {
    throw err;
  }
}

export async function removeWorkspaceManifest(
  workspaceId: WorkspaceId,
  manifestId: ManifestId,
): Promise<void> {
  try {
    await del(`/v1/workspaces/${workspaceId}/manifests/${manifestId}`);
  } catch (err) {
    throw err;
  }
}

// --- Workspace channels ---

export async function fetchWorkspaceChannels(
  workspaceId: WorkspaceId,
  params: PaginationParams = {},
): Promise<V1WorkspaceChannelsGetResp> {
  try {
    const res = await get(`/v1/workspaces/${workspaceId}/channels`, {
      page: params.page?.toString(),
      limit: params.limit?.toString(),
    });
    return (await res.json()) as V1WorkspaceChannelsGetResp;
  } catch (err) {
    throw err;
  }
}

export async function addWorkspaceChannel(
  workspaceId: WorkspaceId,
  channelId: NotificationChannelId,
): Promise<void> {
  try {
    await post(`/v1/workspaces/${workspaceId}/channels`, { channel_id: channelId });
  } catch (err) {
    throw err;
  }
}

export async function removeWorkspaceChannel(
  workspaceId: WorkspaceId,
  channelId: NotificationChannelId,
): Promise<void> {
  try {
    await del(`/v1/workspaces/${workspaceId}/channels/${channelId}`);
  } catch (err) {
    throw err;
  }
}
