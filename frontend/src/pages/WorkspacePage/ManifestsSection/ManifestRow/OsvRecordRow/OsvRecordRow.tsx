import { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useOsvRecordsStore } from '../../../../../stores/useOsvRecordsStore';
import { AppRoute } from '../../../../../routes/Routes';
import styles from './OsvRecordRow.module.css';
import {
  type OsvRecordPartial,
  type Severity,
  severityFromEntry,
} from '../../../../../api/osvRecord';

const SEV_CLASS: Record<Severity, string> = {
  CRITICAL: styles.sevCritical,
  HIGH: styles.sevHigh,
  MEDIUM: styles.sevMedium,
  LOW: styles.sevLow,
  NONE: styles.sevNone,
};

type OsvRecordRowProps = {
  osvId: string;
  detectedAt: string;
};

export function OsvRecordRow({ osvId, detectedAt }: OsvRecordRowProps) {
  const { records, fetchRecord } = useOsvRecordsStore();

  useEffect(() => {
    void fetchRecord(osvId);
  }, [osvId, fetchRecord]);

  const record = records[osvId] as OsvRecordPartial | undefined;
  const severity: Severity = record ? severityFromEntry(record) : 'NONE';

  return (
    <div className={styles.row}>
      <Link
        to={AppRoute.OsvRecord.replace(':name', osvId)}
        className={styles.osvId}
        onClick={(e) => e.stopPropagation()}
      >
        {osvId}
      </Link>
      <span className={[styles.severityBadge, SEV_CLASS[severity]].join(' ')}>{severity}</span>
      <span className={styles.summary}>{record?.summary}</span>
    </div>
  );
}
