import { Link } from 'react-router-dom';
import { formatDate } from '../cveUtils';
import type { OsvRecord } from '../cveUtils';
import { AppRoute } from '../../../routes/Routes';
import styles from './Hero.module.css';

type HeroProps = {
  record: OsvRecord;
};

export function Hero({ record }: HeroProps) {
  const hasChips =
    (record.aliases && record.aliases.length > 0) || (record.related && record.related.length > 0);

  return (
    <div className={styles.hero}>
      <div className={styles.heroTopRow}>
        <span className={styles.heroId}>{record.id}</span>
        {record.withdrawn && <span className={styles.withdrawnBadge}>Withdrawn</span>}
      </div>

      {record.summary && <p className={styles.heroSummary}>{record.summary}</p>}

      {hasChips && (
        <div className={styles.heroChipsRow}>
          {record.aliases && record.aliases.length > 0 && (
            <div className={styles.chipsGroup}>
              <span className={styles.chipsGroupLabel}>Aliases</span>
              <div className={styles.chips}>
                {record.aliases.map((alias) => (
                  <Link key={alias} to={AppRoute.CveRecord.replace(':name', alias)} className={styles.chip}>
                    {alias}
                  </Link>
                ))}
              </div>
            </div>
          )}
          {record.related && record.related.length > 0 && (
            <div className={styles.chipsGroup}>
              <span className={styles.chipsGroupLabel}>Related</span>
              <div className={styles.chips}>
                {record.related.map((rel) => (
                  <Link key={rel} to={AppRoute.CveRecord.replace(':name', rel)} className={styles.chipAccent}>
                    {rel}
                  </Link>
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      <div className={styles.heroTimeline}>
        {record.published && (
          <div className={styles.heroTimelineItem}>
            <span className={styles.heroTimelineKey}>Published</span>
            <span className={styles.heroTimelineVal}>{formatDate(record.published)}</span>
          </div>
        )}
        <div className={styles.heroTimelineItem}>
          <span className={styles.heroTimelineKey}>Modified</span>
          <span className={styles.heroTimelineVal}>{formatDate(record.modified)}</span>
        </div>
        {record.withdrawn && (
          <div className={styles.heroTimelineItem}>
            <span className={styles.heroTimelineKey}>Withdrawn</span>
            <span className={`${styles.heroTimelineVal} ${styles.heroTimelineWithdrawn}`}>
              {formatDate(record.withdrawn)}
            </span>
          </div>
        )}
      </div>
    </div>
  );
}
