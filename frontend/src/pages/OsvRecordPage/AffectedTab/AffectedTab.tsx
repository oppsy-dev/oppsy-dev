import type { OsvRecord } from '../cveUtils';
import { VersionEvents } from './VersionEvents/VersionEvents';
import styles from './AffectedTab.module.css';

type AffectedTabProps = {
  record: OsvRecord;
};

export function AffectedTab({ record }: AffectedTabProps) {
  if (!record.affected || record.affected.length === 0) {
    return <p className={styles.empty}>No affected packages listed.</p>;
  }

  return (
    <div className={styles.card}>
      <table className={styles.table}>
        <thead>
          <tr>
            <th className={styles.th}>Package</th>
            <th className={styles.th}>Ecosystem</th>
            <th className={styles.th}>Versions</th>
          </tr>
        </thead>
        <tbody>
          {record.affected.map((a, i) => (
            // eslint-disable-next-line react/no-array-index-key
            <tr key={i}>
              <td className={`${styles.tdVal} ${styles.tdMono}`}>{a.package?.name ?? '—'}</td>
              <td className={styles.tdKey}>{a.package?.ecosystem ?? '—'}</td>
              <td className={styles.tdVersions}>
                <VersionEvents
                  events={a.ranges?.flatMap((r) => r.events) ?? []}
                  versions={a.versions ?? []}
                />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
