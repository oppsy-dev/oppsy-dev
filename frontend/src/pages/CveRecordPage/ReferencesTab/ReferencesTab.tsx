import { Link } from 'react-router';
import type { OsvRecord } from '../cveUtils';
import styles from './ReferencesTab.module.css';

type ReferencesTabProps = {
  record: OsvRecord;
};

export function ReferencesTab({ record }: ReferencesTabProps) {
  if (!record.references || record.references.length === 0) {
    return <p className={styles.empty}>No references listed.</p>;
  }

  return (
    <div className={styles.card}>
      <table className={styles.table}>
        <thead>
          <tr>
            <th className={styles.th}>URL</th>
          </tr>
        </thead>
        <tbody>
          {record.references.map((ref, i) => (
            // eslint-disable-next-line react/no-array-index-key
            <tr key={i}>
              <td className={styles.tdUrl}>
                <Link
                  to={ref.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className={styles.link}
                >
                  {ref.url}
                </Link>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
