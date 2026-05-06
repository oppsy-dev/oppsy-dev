import { OsvSyncStatus } from '../../OsvSyncStatus/OsvSyncStatus';
import { EventsFeed } from '../../EventsFeed/EventsFeed';
import styles from './RightSidebar.module.css';

export function RightSidebar() {
  return (
    <aside className={styles.sidebar}>
      <div className={styles.syncStatusWrap}>
        <OsvSyncStatus />
      </div>
      <div className={styles.feedWrap}>
        <EventsFeed />
      </div>
    </aside>
  );
}
