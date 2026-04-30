import { formatCreditType } from '../cveUtils';
import type { OsvRecord } from '../cveUtils';
import styles from './CreditsTab.module.css';

type CreditsTabProps = {
  record: OsvRecord;
};

export function CreditsTab({ record }: CreditsTabProps) {
  if (!record.credits || record.credits.length === 0) {
    return <p className={styles.empty}>No credits listed.</p>;
  }

  return (
    <div className={styles.card}>
      <table className={styles.table}>
        <thead>
          <tr>
            <th className={styles.th}>Name</th>
            <th className={styles.th}>Role</th>
            <th className={styles.th}>Contact</th>
          </tr>
        </thead>
        <tbody>
          {record.credits.map((credit, i) => (
            // eslint-disable-next-line react/no-array-index-key
            <tr key={i}>
              <td className={styles.tdVal}>{credit.name}</td>
              <td className={styles.tdKey}>{formatCreditType(credit.type)}</td>
              <td className={styles.tdContact}>
                {credit.contact && credit.contact.length > 0 ? (
                  credit.contact.map((c, j) => (
                    // eslint-disable-next-line react/no-array-index-key
                    <a
                      key={j}
                      href={c}
                      target="_blank"
                      rel="noopener noreferrer"
                      className={styles.link}
                    >
                      {c}
                    </a>
                  ))
                ) : (
                  <span className={styles.empty}>—</span>
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
