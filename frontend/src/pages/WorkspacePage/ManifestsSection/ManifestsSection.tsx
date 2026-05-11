import { useState } from 'react';
import { useManifests, useRemoveWorkspaceManifest } from '../../../hooks/workspaces';
import type { WorkspaceId } from '../../../api/workspaces';
import { BackIcon } from '../../../components/Icons';
import { ManifestRow } from './ManifestRow/ManifestRow';
import styles from './ManifestsSection.module.css';

const PAGE_SIZE = 10;

function UploadIcon({ size = 13 }: { size?: number }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polyline points="16 16 12 12 8 16" />
      <line x1="12" y1="12" x2="12" y2="21" />
      <path d="M20.39 18.39A5 5 0 0018 9h-1.26A8 8 0 103 16.3" />
    </svg>
  );
}

type ManifestsSectionProps = {
  workspaceId: WorkspaceId;
};

function EmptyState() {
  return (
    <div className={styles.emptyState}>
      <div className={styles.emptyIconWrap}>
        <UploadIcon size={22} />
      </div>
      <p className={styles.emptyTitle}>No manifests yet</p>
      <p className={styles.emptyDesc}>
        Upload a manifest to start scanning your dependencies for vulnerabilities.{' '}
        <a
          href="https://oppsy-dev.github.io/oppsy-dev/manifest-upload.html"
          target="_blank"
          rel="noreferrer"
        >
          Learn how ↗
        </a>
      </p>
    </div>
  );
}

export function ManifestsSection({ workspaceId }: ManifestsSectionProps) {
  const [page, setPage] = useState(0);
  const { data: manifests = [], isLoading, isError } = useManifests(workspaceId);
  const removeMutation = useRemoveWorkspaceManifest(workspaceId);

  const totalPages = Math.ceil(manifests.length / PAGE_SIZE);
  const pageManifests = manifests.slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE);

  const pagesNaviation = totalPages > 1 && (
    <div className={styles.pagination}>
      <button
        type="button"
        className={styles.pageBtn}
        onClick={() => setPage((p) => p - 1)}
        disabled={page === 0}
        aria-label="Previous page"
      >
        <BackIcon width={13} height={13} />
      </button>

      <span className={styles.pageInfo}>
        {page + 1} <span className={styles.pageInfoSep}>/</span> {totalPages}
      </span>

      <button
        type="button"
        className={`${styles.pageBtn} ${styles.pageBtnNext}`}
        onClick={() => setPage((p) => p + 1)}
        disabled={page === totalPages - 1}
        aria-label="Next page"
      >
        <BackIcon width={13} height={13} />
      </button>
    </div>
  );

  return (
    <div>
      <div className={styles.header}>
        <div>
          <h3 className={styles.title}>Manifest Files</h3>
          <p className={styles.desc}>
            {`${manifests.length} manifest${manifests.length === 1 ? '' : 's'}`}
          </p>
        </div>
      </div>

      {isError && <p className={styles.errorMsg}>Failed to load manifests.</p>}

      {!isError && !isLoading && manifests.length === 0 ? (
        <EmptyState />
      ) : (
        <>
          {pagesNaviation}
          <div className={styles.tableWrap}>
            <div className={styles.table}>
              <div className={styles.tableHead}>
                <span />
                <span>Name</span>
                <span>Tag</span>
                <span className={styles.tableHeadCenter}>Vulns</span>
                <span>Uploaded</span>
                <span />
              </div>
              {pageManifests.map((m, i) => (
                <ManifestRow
                  key={m.id}
                  manifest={m}
                  isLast={i === pageManifests.length - 1}
                  onRemove={() => removeMutation.mutate(m.id)}
                />
              ))}
            </div>
          </div>
          {pagesNaviation}
        </>
      )}
    </div>
  );
}
