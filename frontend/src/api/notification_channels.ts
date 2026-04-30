import { del, get, patch, post } from './client';
import type { PaginationParams } from './client';
import { components, paths } from './schema';

// --- Types ---

type V1ChannelsGetResp =
  paths['/v1/channels']['get']['responses']['200']['content']['application/json; charset=utf-8'];

type V1ChannelEventsGetResp =
  paths['/v1/channels/{channel_id}/events']['get']['responses']['200']['content']['application/json; charset=utf-8'];

export type EmailChannelConf = { type: 'Email'; from: string; to: string[] };
export type DiscordChannelConf = { type: 'Discord'; discord_webhook_url: string };
export type WebhookChannelConf = { type: 'Webhook'; webhook_url: string; secret?: string | null };
export type ChannelConf = EmailChannelConf | DiscordChannelConf | WebhookChannelConf;

export type NotificationChannelType = components['schemas']['NotificationChannelType'];

export type NotificationChannel = Omit<components['schemas']['NotificationChannel'], 'conf'> & {
  conf: ChannelConf;
};
export type NotificationChannelId = NotificationChannel['id'];

export type CreateChannelRequest = Omit<
  components['schemas']['CreateNotificationChannelRequest'],
  'conf'
> & { conf: ChannelConf };

export type UpdateChannelRequest = Omit<
  components['schemas']['UpdateNotificationChannelRequest'],
  'conf'
> & { conf: ChannelConf };

export type NotificationEvent = components['schemas']['NotificationEvent'];
export type NotificationEventMeta = components['schemas']['NotificationEventMeta'];
export type { PaginationParams } from './client';

// --- Channels ---

export async function fetchChannels(params: PaginationParams = {}): Promise<V1ChannelsGetResp> {
  try {
    const res = await get('/v1/channels', {
      page: params.page?.toString(),
      limit: params.limit?.toString(),
    });
    return (await res.json()) as V1ChannelsGetResp;
  } catch (err) {
    throw err;
  }
}

export async function createChannel(req: CreateChannelRequest): Promise<string> {
  try {
    const res = await post('/v1/channels', req);
    return (await res.json()) as string;
  } catch (err) {
    throw err;
  }
}

export async function updateChannel(
  channelId: NotificationChannelId,
  req: UpdateChannelRequest,
): Promise<void> {
  try {
    await patch(`/v1/channels/${channelId}`, req);
  } catch (err) {
    throw err;
  }
}

export async function deleteChannel(channelId: NotificationChannelId): Promise<void> {
  try {
    await del(`/v1/channels/${channelId}`);
  } catch (err) {
    throw err;
  }
}

// --- Channel events ---

export async function fetchChannelEvents(
  channelId: NotificationChannelId,
  params: PaginationParams = {},
): Promise<V1ChannelEventsGetResp> {
  try {
    const res = await get(`/v1/channels/${channelId}/events`, {
      page: params.page?.toString(),
      limit: params.limit?.toString(),
    });
    return (await res.json()) as V1ChannelEventsGetResp;
  } catch (err) {
    throw err;
  }
}
