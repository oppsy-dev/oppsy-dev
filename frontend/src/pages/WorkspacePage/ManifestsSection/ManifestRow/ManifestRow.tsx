import { useState } from 'react';
import type { paths } from '../../../../api/schema';
import { OsvRecordRow } from './OsvRecordRow/OsvRecordRow';
import { formatUuidV7Date } from '../../../../utils/uuidV7';
import styles from './ManifestRow.module.css';
import { TrashIcon } from '../../../../components/Icons';

type ManifestInfo =
  paths['/v1/workspaces/{workspace_id}/manifests']['get']['responses']['200']['content']['application/json; charset=utf-8']['manifests'][number];

const ECO_CLASS: Record<string, string | undefined> = {
  Npm: styles.ecoNpm,
  Cargo: styles.ecoCargo,
  Poetry: styles.ecoPoetry,
  Go: styles.ecoNpm,
  Uv: styles.ecoPoetry,
};

function ChevronIcon() {
  return (
    <svg
      width="12"
      height="12"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2.5"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polyline points="9 18 15 12 9 6" />
    </svg>
  );
}

function CheckIcon() {
  return (
    <svg
      width="13"
      height="13"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2.5"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polyline points="20 6 9 17 4 12" />
    </svg>
  );
}

type ManifestRowProps = {
  manifest: ManifestInfo;
  isLast: boolean;
  onRemove: () => void;
};

export function ManifestRow({ manifest, isLast, onRemove }: ManifestRowProps) {
  const vulns = manifest.vulnerabilities;
  const [expanded, setExpanded] = useState(false);
  const toggle = () => setExpanded((v) => !v);

  return (
    <div className={[styles.wrapper, isLast ? styles.wrapperLast : ''].filter(Boolean).join(' ')}>
      <div
        role="button"
        tabIndex={0}
        className={[styles.row, expanded ? styles.rowExpanded : ''].filter(Boolean).join(' ')}
        onClick={toggle}
      >
        <span
          className={[styles.chevron, expanded ? styles.chevronExpanded : '']
            .filter(Boolean)
            .join(' ')}
        >
          <ChevronIcon />
        </span>

        <div className={styles.filenameCell}>
          <span className={styles.filename}>{manifest.name}</span>
        </div>

        <span
          className={[
            styles.ecosystemBadge,
            ECO_CLASS[manifest.manifest_type] ?? styles.ecoNpm,
          ].join(' ')}
        >
          {manifest.manifest_type}
        </span>

        <code className={styles.tag}>{manifest.tag ?? '—'}</code>

        <span
          className={[
            styles.vulnCount,
            vulns.length > 0 ? styles.vulnCountRed : styles.vulnCountClean,
          ].join(' ')}
        >
          {vulns.length > 0 ? vulns.length : '—'}
        </span>

        <span className={styles.uploadedAt}>{formatUuidV7Date(manifest.id)}</span>

        <button
          type="button"
          className={styles.trashBtn}
          aria-label="Remove manifest"
          onClick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            onRemove();
          }}
        >
          <TrashIcon width={14} height={14} />
        </button>
      </div>

      {expanded && (
        <div className={styles.vulnPanel}>
          {vulns.length === 0 ? (
            <div className={styles.noVulns}>
              <CheckIcon />
              No vulnerabilities found
            </div>
          ) : (
            <div className={styles.vulnList}>
              {vulns.map((v) => (
                <OsvRecordRow key={v.osv_id} osvId={v.osv_id} detectedAt={v.detected_at} />
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
