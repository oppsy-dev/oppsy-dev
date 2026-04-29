import { create } from 'zustand';
import { paths } from '../api/schema';
import { get as httpGet } from '../api/client';
import { NotFoundError } from '../api/errors';

type OsvRecord =
  paths['/v1/osv/{record_id}']['get']['responses']['200']['content']['application/json; charset=utf-8'];

interface OsvRecordsState {
  records: Record<string, OsvRecord>;
  fetchRecord: (id: string) => Promise<OsvRecord | undefined>;
}

export const useOsvRecordsStore = create<OsvRecordsState>()((set, get) => ({
  records: {},
  fetchRecord: async (id) => {
    const { records } = get();
    if (id in records) return records[id];

    try {
      const res = await httpGet(`/v1/osv/${id}`);
      const record = (await res.json()) as OsvRecord;
      set((s) => ({ records: { ...s.records, [id]: record } }));
      return record;
    } catch (err) {
      if (err instanceof NotFoundError) return undefined;
      throw err;
    }
  },
}));
