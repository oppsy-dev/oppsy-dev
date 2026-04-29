import { useState } from 'react';
import type { NotificationEvent } from '../../../../api/notification_channels';
import { ChevronDownIcon } from '../../../../components/Icons';
import { formatUuidV7Date } from '../../../../utils/uuidV7';
import styles from './EventCard.module.css';

type Props = { event: NotificationEvent };

export function EventCard({ event }: Props) {
  const [expanded, setExpanded] = useState(false);

  return (
    <div className={styles.card}>
      <div className={styles.main}>
        <div className={styles.topRow}>
          <div className={styles.dateGroup}>
            <span className={styles.triggerLabel}>Triggered at</span>
            <span className={styles.date}>{formatUuidV7Date(event.id, true)}</span>
          </div>
        </div>
        <div className={styles.bottomRow}>
          {event.error ? (
            <span className={styles.errorBadge}>Failed</span>
          ) : (
            <span className={styles.successBadge}>Delivered</span>
          )}
          {event.error && (
            <button
              type="button"
              className={
                expanded ? `${styles.expandBtn} ${styles.expandBtnOpen}` : styles.expandBtn
              }
              onClick={() => setExpanded((e) => !e)}
              aria-label={expanded ? 'Collapse' : 'Expand'}
            >
              <ChevronDownIcon width={13} height={13} />
            </button>
          )}
        </div>
      </div>

      {expanded && event.error && (
        <div className={styles.details}>
          <div className={styles.detailsSection}>
            <span className={styles.detailsLabel}>Delivery error</span>
            <p className={styles.errorText}>{event.error}</p>
          </div>
        </div>
      )}
    </div>
  );
}
