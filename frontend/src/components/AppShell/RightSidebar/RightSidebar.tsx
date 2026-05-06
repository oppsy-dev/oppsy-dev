import { OsvSyncStatus } from '../../OsvSyncStatus/OsvSyncStatus';
import styles from './RightSidebar.module.css';

export function RightSidebar() {
  return (
    <aside className={styles.sidebar}>
      <div className={styles.syncStatusWrap}>
        <OsvSyncStatus />
      </div>
    </aside>
  );
}
