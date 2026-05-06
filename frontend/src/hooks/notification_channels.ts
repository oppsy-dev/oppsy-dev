import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  fetchChannels,
  createChannel,
  updateChannel,
  deleteChannel,
  fetchChannelEvents,
  fetchAllChannelEvents,
} from '../api/notification_channels';
import type {
  NotificationChannel,
  NotificationChannelId,
  NotificationEvent,
  CreateChannelRequest,
  UpdateChannelRequest,
  PaginationParams,
} from '../api/notification_channels';
import { useWorkspaces, workspaceChannelsQueryKey, workspacesQueryKey } from './workspaces';

const channelsQueryKey = () => ['channels'] as const;
const channelEventsQueryKey = (channelId: NotificationChannelId) =>
  ['channel', channelId, 'events'] as const;
export const allChannelEventsQueryKey = () => ['channels', 'events'] as const;

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
      queryClient.removeQueries({ queryKey: allChannelEventsQueryKey() });
      for (const w of workspaces) {
        queryClient.invalidateQueries({ queryKey: workspaceChannelsQueryKey(w.id) });
      }
    },
  });
}

export type EnrichedEvent = NotificationEvent & { channel: NotificationChannel };

export function useAllChannelEvents(): { events: EnrichedEvent[]; isLoading: boolean } {
  const { data: channels = [], isLoading: channelsLoading } = useChannels();

  const eventsQuery = useQuery({
    queryKey: allChannelEventsQueryKey(),
    queryFn: () => fetchAllChannelEvents(),
    staleTime: Infinity,
  });

  const channelById = Object.fromEntries(channels.map((c) => [c.id, c]));

  const events = (eventsQuery.data?.events ?? []).flatMap((event) => {
    const channel = channelById[event.channel_id];
    return channel ? [{ ...event, channel }] : [];
  });

  const isLoading = channelsLoading || eventsQuery.isLoading;

  return { events, isLoading };
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
