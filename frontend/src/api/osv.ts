import { get } from './client';
import { components, paths } from './schema';

// --- Types ---

type V1OsvStatusGetResp =
  paths['/v1/osv/status']['get']['responses']['200']['content']['application/json; charset=utf-8'];

export type OsvSyncStatus = components['schemas']['OsvSyncStatus'];

// --- OSV Sync Status ---

export async function fetchOsvSyncStatus(): Promise<V1OsvStatusGetResp> {
  try {
    const res = await get('/v1/osv/status');
    return (await res.json()) as V1OsvStatusGetResp;
  } catch (err) {
    throw err;
  }
}
