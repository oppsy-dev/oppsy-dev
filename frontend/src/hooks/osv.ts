import { useQuery } from '@tanstack/react-query';
import { fetchOsvSyncStatus } from '../api/osv';

const OSV_SYNC_STATUS_STALE_TIME_MS = 60_000;

export const osvSyncStatusQueryKey = () => ['osv', 'status'] as const;

export function useOsvSyncStatus() {
  return useQuery({
    queryKey: osvSyncStatusQueryKey(),
    queryFn: fetchOsvSyncStatus,
    staleTime: OSV_SYNC_STATUS_STALE_TIME_MS,
  });
}
