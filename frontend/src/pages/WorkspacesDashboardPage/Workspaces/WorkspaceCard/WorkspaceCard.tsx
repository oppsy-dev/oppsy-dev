import { Link } from 'react-router-dom';
import type { components } from '../../../../api/schema';
import styles from './WorkspaceCard.module.css';

type WorkspaceCardProps = {
  workspace: components['schemas']['WorkspaceInfo'];
};

function CubeIcon() {
  return (
    <svg
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" />
      <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
      <line x1="12" y1="22.08" x2="12" y2="12" />
    </svg>
  );
}

export function WorkspaceCard({ workspace }: WorkspaceCardProps) {
  return (
    <Link to={`/workspaces/${workspace.id}`} className={styles.card}>
      <div className={styles.cardTop}>
        <div className={styles.iconWrap}>
          <CubeIcon />
        </div>
      </div>
      <div className={styles.cardBody}>
        <p className={styles.cardLabel}>Workspace</p>
        <p className={styles.cardId}>{workspace.name}</p>
      </div>
    </Link>
  );
}
