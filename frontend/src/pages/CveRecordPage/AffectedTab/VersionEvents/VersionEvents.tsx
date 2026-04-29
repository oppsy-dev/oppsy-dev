import type { OsvEvent } from '../../cveUtils';
import styles from './VersionEvents.module.css';

type VersionEventsProps = {
  events: OsvEvent[];
  versions: string[];
};

type Item = { label: string; value: string; className: string };

const EVENT_CONFIG: Record<string, { label: string; className: string }> = {
  introduced: { label: 'introduced', className: styles.badgeIntroduced },
  fixed: { label: 'fixed', className: styles.badgeFixed },
  last_affected: { label: 'last affected', className: styles.badgeLastAffected },
  limit: { label: 'limit', className: styles.badgeLimit },
  version: { label: 'version', className: styles.badgeVersion },
};

function toItems(events: OsvEvent[], versions: string[]): Item[] {
  const versionItems = versions.map((v) => ({
    value: v,
    ...EVENT_CONFIG['version'],
  }));

  const eventItems = events.flatMap((e) => {
    let res = [];
    if (e.introduced) {
      res.push({
        value: e.introduced,
        ...EVENT_CONFIG['introduced'],
      });
    }
    if (e.fixed) {
      res.push({
        value: e.fixed,
        ...EVENT_CONFIG['fixed'],
      });
    }
    if (e.last_affected) {
      res.push({
        value: e.last_affected,
        ...EVENT_CONFIG['last affected'],
      });
    }
    if (e.limit) {
      res.push({
        value: e.limit,
        ...EVENT_CONFIG['limit'],
      });
    }
    return res;
  });

  return [...eventItems, ...versionItems];
}

export function VersionEvents({ events, versions }: VersionEventsProps) {
  const items = toItems(events, versions);

  if (items.length === 0) {
    return <span>—</span>;
  }

  return (
    <ul className={styles.list}>
      {items.map((item, i) => (
        // eslint-disable-next-line react/no-array-index-key
        <li key={i} className={styles.item}>
          <span className={`${styles.badge} ${item.className}`}>{item.label}</span>
          <span className={styles.value}>{item.value}</span>
        </li>
      ))}
    </ul>
  );
}
