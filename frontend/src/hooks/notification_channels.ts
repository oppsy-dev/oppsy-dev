import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  fetchChannels,
  createChannel,
  updateChannel,
  deleteChannel,
  fetchChannelEvents,
} from '../api/notification_channels';
import type {
  NotificationChannelId,
  CreateChannelRequest,
  UpdateChannelRequest,
  PaginationParams,
} from '../api/notification_channels';
import { useWorkspaces, workspaceChannelsQueryKey, workspacesQueryKey } from './workspaces';

const channelsQueryKey = () => ['channels'] as const;
const channelEventsQueryKey = (channelId: NotificationChannelId) =>
  ['channel', channelId, 'events'] as const;

export function useChannels(params: PaginationParams = {}) {
  return useQuery({
    queryKey: [...channelsQueryKey(), params] as const,
    queryFn: () => fetchChannels(params),
    select: (data) => data.channels,
    // TODO: specify a proper stale time instead of Infinity
    staleTime: Infinity,
  });
}

export function useCreateChannel() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: CreateChannelRequest) => createChannel(req),
    onSuccess: () => {
      // The global channel list has a new entry the client hasn't seen yet.
      queryClient.invalidateQueries({ queryKey: channelsQueryKey() });
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
    },
  });
}

export function useUpdateChannel() {
  const queryClient = useQueryClient();
  const { data: workspaces = [] } = useWorkspaces();
  return useMutation({
    mutationFn: ({
      channelId,
      req,
    }: {
      channelId: NotificationChannelId;
      req: UpdateChannelRequest;
    }) => updateChannel(channelId, req),
    onSuccess: (_data, { channelId }) => {
      queryClient.invalidateQueries({ queryKey: channelsQueryKey() });
      queryClient.invalidateQueries({ queryKey: ['channel', channelId] });
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
      for (const w of workspaces) {
        queryClient.invalidateQueries({ queryKey: workspaceChannelsQueryKey(w.id) });
      }
    },
  });
}

export function useDeleteChannel() {
  const queryClient = useQueryClient();
  const { data: workspaces = [] } = useWorkspaces();
  return useMutation({
    mutationFn: (channelId: NotificationChannelId) => deleteChannel(channelId),
    onSuccess: (_data, channelId) => {
      // Drop all cached sub-queries for this channel (events) — the channel no longer exists.
      queryClient.removeQueries({ queryKey: ['channel', channelId] });
      // The global channel list now has one fewer entry.
      queryClient.invalidateQueries({ queryKey: channelsQueryKey() });
      // The deleted channel may have been attached to any workspace. Since we don't track which
      // workspaces used it, invalidate every workspace's channel list and the workspace list itself
      // to stay consistent with any workspace-level channel counts or associations.
      queryClient.invalidateQueries({ queryKey: workspacesQueryKey() });
      for (const w of workspaces) {
        queryClient.invalidateQueries({ queryKey: workspaceChannelsQueryKey(w.id) });
      }
    },
  });
}

export function useChannelEvents(channelId: NotificationChannelId, params: PaginationParams = {}) {
  return useQuery({
    queryKey: [...channelEventsQueryKey(channelId), params] as const,
    queryFn: () => fetchChannelEvents(channelId, params),
    select: (data) => data.events,
    // TODO: specify a proper stale time instead of Infinity
    staleTime: Infinity,
  });
}
