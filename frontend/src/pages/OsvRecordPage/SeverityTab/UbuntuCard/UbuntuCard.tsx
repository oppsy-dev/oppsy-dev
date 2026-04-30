import styles from './UbuntuCard.module.css';

const UBUNTU_CVE_PRIORITIES_URL = 'https://ubuntu.com/security/cves';

const BADGE_CLASS: Record<string, string> = {
  critical: styles.badgeCritical,
  high: styles.badgeHigh,
  medium: styles.badgeMedium,
  low: styles.badgeLow,
};

type UbuntuCardProps = {
  score: string;
};

export function UbuntuCard({ score }: UbuntuCardProps) {
  const badgeClass = BADGE_CLASS[score] ?? styles.badgeNone;

  return (
    <div className={styles.card}>
      <a
        href={UBUNTU_CVE_PRIORITIES_URL}
        target="_blank"
        rel="noopener noreferrer"
        className={styles.title}
      >
        Ubuntu Advisory
      </a>
      <span className={[styles.badge, badgeClass].join(' ')}>{score.toUpperCase()}</span>
    </div>
  );
}
