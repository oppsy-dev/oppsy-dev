import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  fetchWorkspaces,
  createWorkspace,
  deleteWorkspace,
  fetchManifests,
  fetchWorkspaceChannels,
  addWorkspaceChannel,
  removeWorkspaceChannel,
  uploadManifest,
  removeWorkspaceManifest,
} from '../api/workspaces';
import type {
  WorkspaceId,
  CreateWorkspaceRequest,
  NotificationChannelId,
  UploadManifestInput,
  ManifestId,
  PaginationParams,
} from '../api/workspaces';

export const workspacesQueryKey = () => ['workspaces'] as const;
const manifestsQueryKey = (workspaceId: WorkspaceId) =>
  ['workspace', workspaceId, 'manifests'] as const;
export const workspaceChannelsQueryKey = (workspaceId: WorkspaceId) =>
  ['workspace', workspaceId, 'channels'] as const;

export function useWorkspaces(params: PaginationParams = {}) {
  return useQuery({
    queryKey: [...workspacesQueryKey(), params] as const,
    queryFn: () => fetchWorkspaces(params),
    select: (data) => data.workspaces,
    // TODO: specify a proper stale time instead of Infinity
    staleTime: Infinity,
  });
}

export function useCreateWorkspace() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: CreateWorkspaceRequest) => createWorkspace(req),
    onSuccess: () =>
      // The workspace list now has a new entry the client hasn't seen yet.
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() }),
  });
}

export function useDeleteWorkspace() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (workspaceId: WorkspaceId) => deleteWorkspace(workspaceId),
    onSuccess: (_data, workspaceId) => {
      // Drop all cached sub-queries for this workspace (manifests, channels) — the workspace no longer exists.
      queryClient.removeQueries({ queryKey: ['workspace', workspaceId] });
      // The workspace list now has one fewer entry.
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
    },
  });
}

export function useUploadManifest(workspaceId: WorkspaceId) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: UploadManifestInput) => uploadManifest(workspaceId, input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
      // The manifest list for this workspace has a new entry.
      queryClient.invalidateQueries({ queryKey: manifestsQueryKey(workspaceId) });
      // Uploading a manifest triggers a vulnerability scan, which can produce new notification events.
      // Invalidate the global channel list and all channel sub-queries (events) so the UI reflects
      // any newly generated alerts and updated channel state.
      queryClient.invalidateQueries({ queryKey: ['channels'] });
      queryClient.invalidateQueries({ queryKey: ['channel'] });
    },
  });
}

export function useManifests(workspaceId: WorkspaceId, params: PaginationParams = {}) {
  return useQuery({
    queryKey: [...manifestsQueryKey(workspaceId), params] as const,
    queryFn: () => fetchManifests(workspaceId, params),
    select: (data) => data.manifests,
    // TODO: specify a proper stale time instead of Infinity
    staleTime: Infinity,
  });
}

export function useRemoveWorkspaceManifest(workspaceId: WorkspaceId) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (manifestId: ManifestId) => removeWorkspaceManifest(workspaceId, manifestId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
      // The manifest list for this workspace is now missing the removed entry.
      queryClient.invalidateQueries({ queryKey: manifestsQueryKey(workspaceId) });
      queryClient.invalidateQueries({ queryKey: ['channels'] });
      queryClient.invalidateQueries({ queryKey: ['channel'] });
    },
  });
}

export function useWorkspaceChannels(workspaceId: WorkspaceId, params: PaginationParams = {}) {
  return useQuery({
    queryKey: [...workspaceChannelsQueryKey(workspaceId), params] as const,
    queryFn: () => fetchWorkspaceChannels(workspaceId, params),
    select: (data) => data.channels,
    // TODO: specify a proper stale time instead of Infinity
    staleTime: Infinity,
  });
}

export function useAddWorkspaceChannel(workspaceId: WorkspaceId) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (channelId: NotificationChannelId) => addWorkspaceChannel(workspaceId, channelId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
      // The channel list for this workspace has a new entry.
      queryClient.invalidateQueries({ queryKey: workspaceChannelsQueryKey(workspaceId) });
      queryClient.invalidateQueries({ queryKey: ['channels'] });
    },
  });
}

export function useRemoveWorkspaceChannel(workspaceId: WorkspaceId) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (channelId: NotificationChannelId) =>
      removeWorkspaceChannel(workspaceId, channelId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
      // The channel list for this workspace is now missing the removed entry.
      queryClient.invalidateQueries({ queryKey: workspaceChannelsQueryKey(workspaceId) });
      queryClient.invalidateQueries({ queryKey: ['channels'] });
    },
  });
}
