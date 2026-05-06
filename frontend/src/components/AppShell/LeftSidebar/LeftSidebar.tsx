import { NavLink } from 'react-router-dom';
import { OppsyLogo } from '../../OppsyLogo/OppsyLogo';
import { AppRoute } from '../../../routes/Routes';
import styles from './LeftSidebar.module.css';
import { NotificationIcon } from '../../Icons';

function BookIcon() {
  return (
    <svg
      width="15"
      height="15"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
      <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
    </svg>
  );
}

function GridIcon() {
  return (
    <svg
      width="15"
      height="15"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <rect x="3" y="3" width="7" height="7" rx="1.5" />
      <rect x="14" y="3" width="7" height="7" rx="1.5" />
      <rect x="3" y="14" width="7" height="7" rx="1.5" />
      <rect x="14" y="14" width="7" height="7" rx="1.5" />
    </svg>
  );
}

export function LeftSidebar() {
  return (
    <nav className={styles.sidebar}>
      <div className={styles.logoArea}>
        <OppsyLogo link_to={AppRoute.WorkspacesDashboard} />
      </div>

      <div className={styles.navSection}>
        <NavLink
          end
          to={AppRoute.WorkspacesDashboard}
          className={({ isActive }) =>
            [styles.navItem, isActive ? styles.navItemActive : ''].filter(Boolean).join(' ')
          }
        >
          <GridIcon />
          <span>Workspaces</span>
        </NavLink>
        <NavLink
          end
          to={AppRoute.ChannelsDashboard}
          className={({ isActive }) =>
            [styles.navItem, isActive ? styles.navItemActive : ''].filter(Boolean).join(' ')
          }
        >
          <NotificationIcon width={15} height={15} />
          <span>Notifications</span>
        </NavLink>
      </div>

      <div className={styles.bottomSection}>
        <a
          href="https://oppsy-dev.github.io/oppsy-dev/"
          target="_blank"
          rel="noreferrer"
          className={styles.docsLink}
        >
          <BookIcon />
          <span>Docs</span>
        </a>
      </div>

      {/* <div className={styles.teamsSection}>
        <span className={styles.sectionLabel}>Teams</span>
        <div className={styles.teamsList}>
          {teams.map((t) => (
            <TeamChip key={t.id} team={t} />
          ))}
          <AddTeamButton />
        </div>
      </div>

      <div className={styles.bottomSection}>
        <button type="button" className={styles.accountButton}>
          <div className={styles.avatar}>U</div>
          <div className={styles.accountInfo}>
            <span className={styles.accountName}>Account</span>
            <span className={styles.accountMeta}>Settings &amp; billing</span>
          </div>
        </button>
      </div> */}
    </nav>
  );
}
