import { Channels } from './Channels/Channels';
import styles from './ChannelsDashboardPage.module.css';

export function ChannelsDashboardPage() {
  return (
    <div className={styles.page}>
      <header className={styles.pageHeader}>
        <h1 className={styles.pageTitle}>Notifications channels</h1>
        <p className={styles.pageSubtitle}>Configure notification channels for your workspaces.</p>
      </header>
      <Channels />
    </div>
  );
}
