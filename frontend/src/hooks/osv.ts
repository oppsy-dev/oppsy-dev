import { useQuery } from '@tanstack/react-query';
import { fetchOsvSyncStatus } from '../api/osv';

export const osvSyncStatusQueryKey = () => ['osv', 'status'] as const;

export function useOsvSyncStatus() {
  return useQuery({
    queryKey: osvSyncStatusQueryKey(),
    queryFn: fetchOsvSyncStatus,
  });
}
