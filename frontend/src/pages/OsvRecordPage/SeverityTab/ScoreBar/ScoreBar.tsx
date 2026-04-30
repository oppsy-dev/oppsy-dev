import { scoreToSeverity, type Severity } from '../../../../api/osvRecord';
import styles from './ScoreBar.module.css';

const BAR_CLASS: Record<Severity, string> = {
  CRITICAL: styles.barCritical,
  HIGH: styles.barHigh,
  MEDIUM: styles.barMedium,
  LOW: styles.barLow,
  NONE: styles.barNone,
};

type ScoreBarProps = {
  overall: number;
};

export function ScoreBar({ overall }: ScoreBarProps) {
  const severity = scoreToSeverity(overall);
  return (
    <div className={styles.wrapper}>
      <div className={styles.track}>
        <div
          className={[styles.fill, BAR_CLASS[severity]].join(' ')}
          style={{ width: `${(overall / 10) * 100}%` }}
        />
      </div>
      <div className={styles.labels}>
        <span>0</span>
        <span>2</span>
        <span>4</span>
        <span>6</span>
        <span>8</span>
        <span>10</span>
      </div>
    </div>
  );
}
