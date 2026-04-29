import { Link } from 'react-router';
import styles from './OppsyLogo.module.css';

type OppsyLogoProps = {
  link_to?: string;
};

function OppsyIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 32 36"
      fill="none"
      className={styles.icon}
      aria-hidden="true"
    >
      <circle cx="16" cy="18" r="15" stroke="currentColor" strokeWidth="2" />
      <line
        x1="9.5"
        y1="12"
        x2="13.5"
        y2="16"
        stroke="currentColor"
        strokeWidth="2.2"
        strokeLinecap="round"
      />
      <line
        x1="13.5"
        y1="12"
        x2="9.5"
        y2="16"
        stroke="currentColor"
        strokeWidth="2.2"
        strokeLinecap="round"
      />
      <line
        x1="18.5"
        y1="12"
        x2="22.5"
        y2="16"
        stroke="currentColor"
        strokeWidth="2.2"
        strokeLinecap="round"
      />
      <line
        x1="22.5"
        y1="12"
        x2="18.5"
        y2="16"
        stroke="currentColor"
        strokeWidth="2.2"
        strokeLinecap="round"
      />
      <ellipse cx="16" cy="25" rx="4" ry="5" fill="currentColor" />
    </svg>
  );
}

export function OppsyLogo({ link_to }: OppsyLogoProps) {
  const logo = (
    <span className={styles.logo}>
      <OppsyIcon />
      <span className={styles.name}>ppsy</span>
    </span>
  );

  if (link_to) {
    return (
      <Link to={link_to} className={styles.link}>
        {logo}
      </Link>
    );
  }

  return logo;
}
