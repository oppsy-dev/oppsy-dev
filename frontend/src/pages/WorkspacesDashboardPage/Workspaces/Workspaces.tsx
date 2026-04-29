import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useWorkspaces, useCreateWorkspace } from '../../../hooks/workspaces';
import { WorkspaceCard } from './WorkspaceCard/WorkspaceCard';
import { CreateWorkspaceModal } from './CreateWorkspaceModal/CreateWorkspaceModal';
import styles from './Workspaces.module.css';
import { AppRoute } from '../../../routes/Routes';

const PAGE_SIZE = 9;

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
  const [page, setPage] = useState(0);

  const handleConfirm = async (name: string) => {
    const info = await createWorkspace.mutateAsync({ name });
    navigate(AppRoute.Workspace.replace(':workspaceId', info.id));
  };

  const totalPages = Math.ceil(workspaces.length / PAGE_SIZE);
  const pageWorkspaces = workspaces.slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE);
  const pagesNavigation = totalPages > 1 && (
    <div className={styles.pagination}>
      <button
        type="button"
        className={styles.pageBtn}
        onClick={() => setPage((p) => p - 1)}
        disabled={page === 0}
      >
        ← Previous
      </button>

      <div className={styles.pageDots}>
        {Array.from({ length: totalPages }, (_, i) => (
          <button
            key={i}
            type="button"
            className={i === page ? `${styles.pageDot} ${styles.pageDotActive}` : styles.pageDot}
            onClick={() => setPage(i)}
            aria-label={`Page ${i + 1}`}
          />
        ))}
      </div>

      <button
        type="button"
        className={styles.pageBtn}
        onClick={() => setPage((p) => p + 1)}
        disabled={page === totalPages - 1}
      >
        Next →
      </button>
    </div>
  );

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

      {pageWorkspaces.length > 0 && (
        <>
          <div className={styles.grid}>
            {pageWorkspaces.map((ws) => (
              <WorkspaceCard key={ws.id} workspace={ws} />
            ))}
          </div>

          {pagesNavigation}
        </>
      )}

      {modalOpen && (
        <CreateWorkspaceModal onClose={() => setModalOpen(false)} onConfirm={handleConfirm} />
      )}
    </section>
  );
}
