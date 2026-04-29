import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useWorkspaces, useCreateWorkspace } from '../../../hooks/workspaces';
import { WorkspaceCard } from './WorkspaceCard/WorkspaceCard';
import { CreateWorkspaceModal } from './CreateWorkspaceModal/CreateWorkspaceModal';
import styles from './Workspaces.module.css';
import { AppRoute } from '../../../routes/Routes';

function FolderIcon() {
  return (
    <svg
      width="28"
      height="28"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z" />
    </svg>
  );
}

export function Workspaces() {
  const { data: workspaces = [], isLoading } = useWorkspaces();
  const createWorkspace = useCreateWorkspace();
  const navigate = useNavigate();
  const [modalOpen, setModalOpen] = useState(false);

  const handleConfirm = async (name: string) => {
    const info = await createWorkspace.mutateAsync({ name });
    navigate(AppRoute.Workspace.replace(':workspaceId', info.id));
  };

  return (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <div>
          <h2 className={styles.sectionTitle}>Your Workspaces</h2>
          {workspaces.length > 0 && (
            <p className={styles.count}>
              {workspaces.length} workspace{workspaces.length !== 1 ? 's' : ''}
            </p>
          )}
        </div>
        <button className={styles.addButton} type="button" onClick={() => setModalOpen(true)}>
          <span className={styles.addIcon}>+</span>
          New workspace
        </button>
      </div>

      {isLoading && (
        <div className={styles.skeletonGrid}>
          {[1, 2, 3].map((n) => (
            <div key={n} className={styles.skeleton} />
          ))}
        </div>
      )}

      {!isLoading && workspaces.length === 0 && (
        <div className={styles.emptyState}>
          <div className={styles.emptyIconWrap}>
            <FolderIcon />
          </div>
          <p className={styles.emptyTitle}>No workspaces yet</p>
          <p className={styles.emptyDesc}>
            Upload a lock file to start monitoring your dependencies for vulnerabilities.
          </p>
          <button className={styles.emptyAction} type="button" onClick={() => setModalOpen(true)}>
            Create your first workspace
          </button>
        </div>
      )}

      {workspaces.length > 0 && (
        <div className={styles.grid}>
          {workspaces.map((ws) => (
            <WorkspaceCard key={ws.id} workspace={ws} />
          ))}
        </div>
      )}

      {modalOpen && (
        <CreateWorkspaceModal onClose={() => setModalOpen(false)} onConfirm={handleConfirm} />
      )}
    </section>
  );
}
