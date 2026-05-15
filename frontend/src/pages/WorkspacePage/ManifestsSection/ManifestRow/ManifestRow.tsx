import { useRef, useState } from 'react';
import type { KeyboardEvent } from 'react';
import type { paths } from '../../../../api/schema';
import { OsvRecordRow } from './OsvRecordRow/OsvRecordRow';
import { PackagesTab } from './PackagesTab/PackagesTab';
import { formatUuidV7Date } from '../../../../utils/uuidV7';
import styles from './ManifestRow.module.css';
import { TrashIcon } from '../../../../components/Icons';
import type { WorkspaceId } from '../../../../api/workspaces';

type ManifestInfo =
  paths['/v1/workspaces/{workspace_id}/manifests']['get']['responses']['200']['content']['application/json; charset=utf-8']['manifests'][number];

type TabKey = 'vulnerabilities' | 'packages';
const TAB_ORDER: TabKey[] = ['vulnerabilities', 'packages'];

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
  workspaceId: WorkspaceId;
  manifest: ManifestInfo;
  isLast: boolean;
  onRemove: () => void;
};

export function ManifestRow({ workspaceId, manifest, isLast, onRemove }: ManifestRowProps) {
  const vulns = manifest.vulnerabilities;
  const [expanded, setExpanded] = useState(false);
  const [activeTab, setActiveTab] = useState<TabKey>('vulnerabilities');
  const toggle = () => setExpanded((v) => !v);

  const tabBaseId = `manifest-${manifest.id}`;
  const tabId = (tab: TabKey) => `${tabBaseId}-tab-${tab}`;
  const panelId = (tab: TabKey) => `${tabBaseId}-panel-${tab}`;
  const vulnTabRef = useRef<HTMLButtonElement>(null);
  const pkgTabRef = useRef<HTMLButtonElement>(null);
  const tabRef = (tab: TabKey) => (tab === 'vulnerabilities' ? vulnTabRef : pkgTabRef);

  const onTabKeyDown = (e: KeyboardEvent<HTMLButtonElement>) => {
    if (e.key !== 'ArrowLeft' && e.key !== 'ArrowRight') return;
    e.preventDefault();
    const idx = TAB_ORDER.indexOf(activeTab);
    const next =
      e.key === 'ArrowRight'
        ? TAB_ORDER[(idx + 1) % TAB_ORDER.length]
        : TAB_ORDER[(idx - 1 + TAB_ORDER.length) % TAB_ORDER.length];
    setActiveTab(next);
    tabRef(next).current?.focus();
  };

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
          <div className={styles.tabBar} role="tablist" onClick={(e) => e.stopPropagation()}>
            <button
              type="button"
              role="tab"
              id={tabId('vulnerabilities')}
              ref={vulnTabRef}
              aria-selected={activeTab === 'vulnerabilities'}
              aria-controls={panelId('vulnerabilities')}
              tabIndex={activeTab === 'vulnerabilities' ? 0 : -1}
              className={[
                styles.tabBtn,
                activeTab === 'vulnerabilities' ? styles.tabBtnActive : '',
              ].join(' ')}
              onClick={() => setActiveTab('vulnerabilities')}
              onKeyDown={onTabKeyDown}
            >
              Vulnerabilities{vulns.length > 0 ? ` (${vulns.length})` : ''}
            </button>
            <button
              type="button"
              role="tab"
              id={tabId('packages')}
              ref={pkgTabRef}
              aria-selected={activeTab === 'packages'}
              aria-controls={panelId('packages')}
              tabIndex={activeTab === 'packages' ? 0 : -1}
              className={[styles.tabBtn, activeTab === 'packages' ? styles.tabBtnActive : ''].join(
                ' ',
              )}
              onClick={() => setActiveTab('packages')}
              onKeyDown={onTabKeyDown}
            >
              Packages
            </button>
          </div>

          {activeTab === 'vulnerabilities' && (
            <div
              role="tabpanel"
              id={panelId('vulnerabilities')}
              aria-labelledby={tabId('vulnerabilities')}
            >
              {vulns.length === 0 ? (
                <div className={styles.noVulns}>
                  <CheckIcon />
                  No vulnerabilities found
                </div>
              ) : (
                <div className={styles.vulnList}>
                  {vulns.map((osv_id) => (
                    <OsvRecordRow key={osv_id} osvId={osv_id} />
                  ))}
                </div>
              )}
            </div>
          )}

          {activeTab === 'packages' && (
            <PackagesTab
              workspaceId={workspaceId}
              manifestId={manifest.id}
              id={panelId('packages')}
              labelledBy={tabId('packages')}
            />
          )}
        </div>
      )}
    </div>
  );
}
