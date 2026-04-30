import type { OsvRecord } from '../cveUtils';
import styles from './OverviewTab.module.css';

type OverviewTabProps = {
  record: OsvRecord;
};

export function OverviewTab({ record }: OverviewTabProps) {
  if (!record.details) {
    return <p className={styles.empty}>No description available.</p>;
  }

  return (
    <div className={styles.card}>
      <p className={styles.cardLabel}>Description</p>
      {/* details is CommonMark markdown — rendered as plain text until a markdown renderer is added */}
      <p className={styles.descText}>{record.details}</p>
    </div>
  );
}
