import { useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { useWorkspaces, useDeleteWorkspace } from '../../hooks/workspaces';
import { AppRoute } from '../../routes/Routes';
import { ManifestsSection } from './ManifestsSection/ManifestsSection';
import { WorkspaceSettingsSection } from './WorkspaceSettingsSection/WorkspaceSettingsSection';
import { NotificationsSection } from './NotificationsSection/NotificationsSection';
import { PageHeader } from '../../components/PageHeader/PageHeader';
import styles from './WorkspacePage.module.css';
import { BackIcon, CubeIcon } from '../../components/Icons';

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

      <PageHeader
        name={displayName ?? ''}
        id={workspaceId ?? ''}
        icon={<CubeIcon width={28} height={28} />}
        onSettingsClick={() => setShowSettings((s) => !s)}
        settingsActive={showSettings}
      />

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
                  onClick={() => setTab(label)}
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
