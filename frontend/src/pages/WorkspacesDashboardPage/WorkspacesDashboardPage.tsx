import { Workspaces } from './Workspaces/Workspaces';
import styles from './WorkspacesDashboardPage.module.css';

export function WorkspacesDashboardPage() {
  return (
    <div className={styles.page}>
      <header className={styles.pageHeader}>
        <h1 className={styles.pageTitle}>Workspaces</h1>
        <p className={styles.pageSubtitle}>
          Monitor dependency vulnerabilities across your projects.
        </p>
      </header>
      <Workspaces />
    </div>
  );
}
