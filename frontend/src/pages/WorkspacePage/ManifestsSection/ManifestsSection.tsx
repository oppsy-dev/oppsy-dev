import { useState } from 'react';
import { useManifests, useRemoveWorkspaceManifest } from '../../../hooks/workspaces';
import type { WorkspaceId } from '../../../api/workspaces';
import { ManifestRow } from './ManifestRow/ManifestRow';
import { UploadManifestModal } from './UploadManifestModal/UploadManifestModal';
import styles from './ManifestsSection.module.css';

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

function EmptyState({ onUpload }: { onUpload: () => void }) {
  return (
    <div className={styles.emptyState}>
      <div className={styles.emptyIconWrap}>
        <UploadIcon size={22} />
      </div>
      <p className={styles.emptyTitle}>No lock files yet</p>
      <p className={styles.emptyDesc}>
        Upload a lock file to start scanning your dependencies for vulnerabilities.
      </p>
      <button type="button" className={styles.emptyAction} onClick={onUpload}>
        Upload your first file
      </button>
    </div>
  );
}

export function ManifestsSection({ workspaceId }: ManifestsSectionProps) {
  const [uploadModalOpen, setUploadModalOpen] = useState(false);
  const { data: manifests = [], isLoading, isError } = useManifests(workspaceId);
  const removeMutation = useRemoveWorkspaceManifest(workspaceId);

  return (
    <div>
      <div className={styles.header}>
        <div>
          <h3 className={styles.title}>Manifest Files</h3>
          <p className={styles.desc}>
            {`${manifests.length} manifest${manifests.length === 1 ? '' : 's'}`}
          </p>
        </div>
        <button type="button" className={styles.uploadBtn} onClick={() => setUploadModalOpen(true)}>
          <UploadIcon />
          Upload file
        </button>
      </div>

      {isError && <p className={styles.errorMsg}>Failed to load manifests.</p>}

      {!isError && !isLoading && manifests.length === 0 ? (
        <EmptyState onUpload={() => setUploadModalOpen(true)} />
      ) : (
        <div className={styles.tableWrap}>
          <div className={styles.table}>
            <div className={styles.tableHead}>
              <span />
              <span>Name</span>
              <span>Ecosystem</span>
              <span>Tag</span>
              <span className={styles.tableHeadCenter}>Vulns</span>
              <span>Uploaded</span>
              <span />
            </div>
            {manifests.map((m, i) => (
              <ManifestRow
                key={m.id}
                manifest={m}
                isLast={i === manifests.length - 1}
                onRemove={() => removeMutation.mutate(m.id)}
              />
            ))}
          </div>
        </div>
      )}

      {uploadModalOpen && (
        <UploadManifestModal workspaceId={workspaceId} onClose={() => setUploadModalOpen(false)} />
      )}
    </div>
  );
}
