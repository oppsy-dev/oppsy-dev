import { scoreToSeverity, type Severity } from '../../../../api/osvRecord';
import styles from './SubScoreChip.module.css';

const SCORE_CLASS: Record<Severity, string> = {
  CRITICAL: styles.scoreCritical,
  HIGH: styles.scoreHigh,
  MEDIUM: styles.scoreMedium,
  LOW: styles.scoreLow,
  NONE: styles.scoreNone,
};

const FILL_CLASS: Record<Severity, string> = {
  CRITICAL: styles.fillCritical,
  HIGH: styles.fillHigh,
  MEDIUM: styles.fillMedium,
  LOW: styles.fillLow,
  NONE: styles.fillNone,
};

const TRACK_CLASS: Record<Severity, string> = {
  CRITICAL: styles.trackCritical,
  HIGH: styles.trackHigh,
  MEDIUM: styles.trackMedium,
  LOW: styles.trackLow,
  NONE: styles.trackNone,
};

type SubScoreChipProps = { label: string; value: number };

export function SubScoreChip({ label, value }: SubScoreChipProps) {
  const severity = scoreToSeverity(value);
  return (
    <div className={styles.chip}>
      <div className={[styles.track, TRACK_CLASS[severity]].join(' ')}>
        <div
          className={[styles.fill, FILL_CLASS[severity]].join(' ')}
          style={{ height: `${(value / 10) * 100}%` }}
        />
      </div>
      <span className={styles.valueRow}>
        <span className={[styles.value, SCORE_CLASS[severity]].join(' ')}>{value}</span>
        <span className={styles.outOf}>/10</span>
      </span>
      <span className={styles.label}>{label}</span>
    </div>
  );
}
