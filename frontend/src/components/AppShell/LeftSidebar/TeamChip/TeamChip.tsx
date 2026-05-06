import { NavLink } from 'react-router-dom';
import { AppRoute } from '../../../../routes/Routes';
import styles from './TeamChip.module.css';

type TeamChipProps = {
  id: string;
  name: string;
};

export function TeamChip({ id, name }: TeamChipProps) {
  const SHORT_MAX_LEN = 20;
  const initials = name.slice(0, 2).toUpperCase();
  const short = name.slice(0, SHORT_MAX_LEN);
  const to = AppRoute.TeamDashboard.replace(':teamId', id);

  return (
    <NavLink
      to={to}
      title={name}
      className={({ isActive }) =>
        [styles.chip, isActive ? styles.chipActive : ''].filter(Boolean).join(' ')
      }
    >
      <div className={styles.avatar}>{initials}</div>
      <span className={styles.label}>
        {short}
        {name.length > SHORT_MAX_LEN && <>&hellip;</>}
      </span>
    </NavLink>
  );
}
