import { useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { useWorkspaces, useDeleteWorkspace } from '../../hooks/workspaces';
import { AppRoute } from '../../routes/Routes';
import { ManifestsSection } from './ManifestsSection/ManifestsSection';
import { WorkspaceSettingsSection } from './WorkspaceSettingsSection/WorkspaceSettingsSection';
import { NotificationsSection } from './NotificationsSection/NotificationsSection';
import styles from './WorkspacePage.module.css';
import { BackIcon, GearIcon } from '../../components/Icons';
import { formatUuidV7Date } from '../../utils/uuidV7';

enum Tab {
  Manifests = 'Manifests',
  Notifications = 'Notifications',
}

const TABS: {
  label: Tab;
  section_element: (workspaceId: string) => React.ReactElement;
}[] = [
  {
    label: Tab.Manifests,
    section_element: (workspaceId) => <ManifestsSection workspaceId={workspaceId} />,
  },
  {
    label: Tab.Notifications,
    section_element: (workspaceId) => <NotificationsSection workspaceId={workspaceId} />,
  },
];

function CubeIcon() {
  return (
    <svg
      width="22"
      height="22"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.6"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" />
      <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
      <line x1="12" y1="22.08" x2="12" y2="12" />
    </svg>
  );
}

export function WorkspacePage() {
  const { workspaceId } = useParams<{ workspaceId: string }>();
  const navigate = useNavigate();
  const { data: workspaces = [] } = useWorkspaces();
  const workspace = workspaceId ? workspaces.find((w) => w.id === workspaceId) : undefined;
  const deleteWorkspace = useDeleteWorkspace();
  const displayName = workspace?.name;
  const [tab, setTab] = useState<Tab>(Tab.Manifests);
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className={styles.page}>
      <nav className={styles.breadcrumb}>
        <Link to={AppRoute.WorkspacesDashboard} className={styles.breadcrumbLink}>
          <BackIcon width={13} height={13} />
          Workspaces
        </Link>
        <span className={styles.breadcrumbSep}>/</span>
        <code className={styles.breadcrumbCurrent}>{displayName}</code>
      </nav>

      <div className={styles.header}>
        <div className={styles.headerLeft}>
          <div className={styles.workspaceIcon}>
            <CubeIcon />
          </div>
          <div>
            <h1 className={styles.title}>{displayName}</h1>
            {workspaceId && (
              <p className={styles.subline}>Created {formatUuidV7Date(workspaceId)}</p>
            )}
          </div>
        </div>
        <button
          type="button"
          className={
            showSettings ? `${styles.settingsBtn} ${styles.settingsBtnActive}` : styles.settingsBtn
          }
          onClick={() => setShowSettings((s) => !s)}
        >
          <GearIcon width={13} height={13} />
          Settings
        </button>
      </div>

      {showSettings ? (
        <WorkspaceSettingsSection
          workspaceName={displayName ?? ''}
          onBack={() => setShowSettings(false)}
          onDelete={() =>
            workspaceId ? deleteWorkspace.mutateAsync(workspaceId) : Promise.resolve()
          }
          onDeleted={() => navigate(AppRoute.WorkspacesDashboard)}
        />
      ) : (
        <>
          <div className={styles.statsStrip}>
            <div className={styles.statCard}>
              <div className={styles.statValue}>{workspace?.manifest_count}</div>
              <div className={styles.statLabel}>Manifests</div>
            </div>
            <div className={styles.statCard}>
              <div className={styles.statValue}>{workspace?.channel_count}</div>
              <div className={styles.statLabel}>Notification channels</div>
            </div>
            <div className={styles.statCard}>
              <div className={styles.statValueMuted}>2h ago</div>
              <div className={styles.statLabel}>Last scan</div>
            </div>
          </div>

          <div className={styles.tabBar}>
            <div className={styles.tabs}>
              {TABS.map(({ label }) => (
                <button
                  key={label}
                  type="button"
                  className={tab === label ? styles.tabActive : styles.tab}
                  onClick={() => {
                    setTab(label);
                  }}
                >
                  {label}
                </button>
              ))}
            </div>
          </div>
          {TABS.filter(({ label }) => label === tab).map(({ section_element }) =>
            section_element(workspaceId ?? ''),
          )}
        </>
      )}
    </div>
  );
}
