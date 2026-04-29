import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { useOsvRecordsStore } from '../../stores/useOsvRecordsStore';
import { NotFoundPage } from '../NotFoundPage/NotFoundPage';
import type { OsvRecord } from './cveUtils';
import { OverviewTab } from './OverviewTab/OverviewTab';
import { AffectedTab } from './AffectedTab/AffectedTab';
import { SeverityTab } from './SeverityTab/SeverityTab';
import { ReferencesTab } from './ReferencesTab/ReferencesTab';
import { CreditsTab } from './CreditsTab/CreditsTab';
import { OppsyLogo } from '../../components/OppsyLogo/OppsyLogo';
import { ViewToggle, ViewMode } from './ViewToggle/ViewToggle';
import { Hero } from './Hero/Hero';
import { JsonView } from '../../components/JsonView/JsonView';
import styles from './CveRecordPage.module.css';
import { PageBackground } from '../../components/PageBackground/PageBackground';
import { AppRoute } from '../../routes/Routes';
import { BackIcon } from '../../components/Icons';

enum Tab {
  Overview = 'overview',
  Affected = 'affected',
  Severity = 'severity',
  References = 'references',
  Credits = 'credits',
}

const TABS: { id: Tab; label: string }[] = [
  { id: Tab.Overview, label: 'Overview' },
  { id: Tab.Affected, label: 'Affected' },
  { id: Tab.Severity, label: 'Severity' },
  { id: Tab.References, label: 'References' },
  { id: Tab.Credits, label: 'Credits' },
];

export function CveRecordPage() {
  const { name } = useParams<{ name: string }>();
  const navigate = useNavigate();
  const canGoBack = window.history.length > 1;
  const { records, fetchRecord } = useOsvRecordsStore();
  const record = name !== undefined ? (records[name] as OsvRecord | undefined) : undefined;
  const [notFound, setNotFound] = useState(false);
  const [tab, setTab] = useState<Tab>(Tab.Overview);
  const [viewMode, setViewMode] = useState<ViewMode>(ViewMode.View);

  useEffect(() => {
    if (!name) return;
    fetchRecord(name)
      .then((r) => {
        if (r === undefined) setNotFound(true);
      })
      .catch((err: unknown) => {
        console.error(err);
      });
  }, [name, fetchRecord]);

  const background = <PageBackground is_top_glow={false} />;

  if (notFound) return <NotFoundPage />;
  if (record === undefined) return <main className={styles.page}>{background}</main>;

  return (
    <main className={styles.page}>
      {background}

      <div className={styles.inner}>
        <div className={styles.topRow}>
          {canGoBack ? (
            <button type="button" className={styles.backButton} onClick={() => navigate(-1)}>
              <BackIcon width={13} height={13} />
              Back
            </button>
          ) : (
            <span />
          )}
          <OppsyLogo link_to={AppRoute.WorkspacesDashboard} />
        </div>

        <Hero record={record} />

        <div className={styles.tabBar}>
          <div className={styles.tabs}>
            {TABS.map(({ id, label }) => (
              <button
                key={id}
                type="button"
                className={
                  tab === id
                    ? viewMode === ViewMode.Raw
                      ? styles.tabActiveMuted
                      : styles.tabActive
                    : styles.tab
                }
                onClick={() => {
                  if (viewMode !== ViewMode.Raw) setTab(id);
                }}
              >
                {label}
              </button>
            ))}
          </div>
          <ViewToggle viewMode={viewMode} onChange={setViewMode} />
        </div>

        {viewMode === ViewMode.Raw ? (
          <div className={styles.rawJson}>
            <JsonView value={record} filename={`${record.id}.json`} />
          </div>
        ) : (
          <div className={styles.tabContent}>
            {tab === Tab.Overview && <OverviewTab record={record} />}
            {tab === Tab.Affected && <AffectedTab record={record} />}
            {tab === Tab.Severity && <SeverityTab record={record} />}
            {tab === Tab.References && <ReferencesTab record={record} />}
            {tab === Tab.Credits && <CreditsTab record={record} />}
          </div>
        )}
      </div>
    </main>
  );
}
