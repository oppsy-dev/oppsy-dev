import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { useManifestPackages } from '../../../../../hooks/workspaces';
import { AppRoute } from '../../../../../routes/Routes';
import type { ManifestId, WorkspaceId } from '../../../../../api/workspaces';
import styles from './PackagesTab.module.css';

const PAGE_SIZE = 50;

type PackagesTabProps = {
  workspaceId: WorkspaceId;
  manifestId: ManifestId;
  id?: string;
  labelledBy?: string;
};

export function PackagesTab({ workspaceId, manifestId, id, labelledBy }: PackagesTabProps) {
  const [page, setPage] = useState(0);
  const [vulnerableOnly, setVulnerableOnly] = useState(false);
  const toggleVulnerableOnly = () => {
    setVulnerableOnly((v) => !v);
    setPage(0);
  };
  const { data, isLoading, isError } = useManifestPackages(workspaceId, manifestId, {
    page,
    limit: PAGE_SIZE,
    vulnerableOnly,
  });

  const total = data?.total ?? 0;
  const pageCount = Math.max(1, Math.ceil(total / PAGE_SIZE));

  // If `total` shrinks (filter toggled, manifest replaced, etc.) snap the page
  // back into range so the pager controls stay sane.
  useEffect(() => {
    if (page > pageCount - 1) setPage(pageCount - 1);
  }, [page, pageCount]);

  if (isLoading)
    return (
      <div className={styles.status} role="tabpanel" id={id} aria-labelledby={labelledBy}>
        Loading packages…
      </div>
    );
  if (isError || !data)
    return (
      <div className={styles.statusError} role="tabpanel" id={id} aria-labelledby={labelledBy}>
        Failed to load packages.
      </div>
    );

  const { packages } = data;
  const pageStart = total === 0 ? 0 : page * PAGE_SIZE + 1;
  const pageEnd = Math.min(total, page * PAGE_SIZE + packages.length);

  const filterBar = (
    <div className={styles.filterBar}>
      <label className={styles.filterLabel}>
        <input
          type="checkbox"
          className={styles.filterCheckbox}
          checked={vulnerableOnly}
          onChange={toggleVulnerableOnly}
        />
        Vulnerable only
      </label>
      <span className={styles.filterCount}>
        {total} package{total === 1 ? '' : 's'}
      </span>
    </div>
  );

  if (packages.length === 0) {
    return (
      <div className={styles.wrapper} role="tabpanel" id={id} aria-labelledby={labelledBy}>
        {filterBar}
        <div className={styles.status}>
          {vulnerableOnly
            ? 'No vulnerable packages in this manifest.'
            : 'No packages parsed from this manifest.'}
        </div>
      </div>
    );
  }

  return (
    <div className={styles.wrapper} role="tabpanel" id={id} aria-labelledby={labelledBy}>
      {filterBar}
      <div className={styles.list}>
        {packages.map((pkg, idx) => (
          <div key={`${idx}:${pkg.ecosystem}:${pkg.name}@${pkg.version}`} className={styles.row}>
            <span className={styles.name}>{pkg.name}</span>
            <span className={styles.version}>{pkg.version}</span>
            <span className={styles.ecosystem}>{pkg.ecosystem}</span>
            <div className={styles.osvIds}>
              {pkg.osv_ids.length === 0 ? (
                <span className={styles.osvIdsEmpty}>—</span>
              ) : (
                pkg.osv_ids.map((osvId) => (
                  <Link
                    key={osvId}
                    to={AppRoute.OsvRecord.replace(':name', osvId)}
                    className={styles.osvIdBadge}
                    onClick={(e) => e.stopPropagation()}
                  >
                    {osvId}
                  </Link>
                ))
              )}
            </div>
          </div>
        ))}
      </div>

      <div className={styles.pagination}>
        <span className={styles.pageRange}>
          {pageStart}–{pageEnd} of {total}
        </span>
        <div className={styles.pageControls}>
          <button
            type="button"
            className={styles.pageBtn}
            onClick={() => setPage((p) => Math.max(0, p - 1))}
            disabled={page === 0}
          >
            Previous
          </button>
          <span className={styles.pageIndicator}>
            Page {page + 1} of {pageCount}
          </span>
          <button
            type="button"
            className={styles.pageBtn}
            onClick={() => setPage((p) => Math.min(pageCount - 1, p + 1))}
            disabled={page >= pageCount - 1}
          >
            Next
          </button>
        </div>
      </div>
    </div>
  );
}
