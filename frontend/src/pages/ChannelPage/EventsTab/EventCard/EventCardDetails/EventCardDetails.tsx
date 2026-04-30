import type { NotificationEvent } from '../../../../../api/notification_channels';
import { Link } from 'react-router-dom';
import { AppRoute } from '../../../../../routes/Routes';
import styles from './EventCardDetails.module.css';

type Props = { event: NotificationEvent };

export function EventCardDetails({ event }: Props) {
  const { meta, error } = event;

  return (
    <div className={styles.details}>
      {error && (
        <div className={styles.section}>
          <span className={styles.label}>Delivery error</span>
          <p className={styles.errorText}>{error}</p>
        </div>
      )}

      <div className={styles.section}>
        <span className={styles.label}>Workspace</span>
        <span className={styles.value}>{meta.workspace_name}</span>
      </div>

      <div className={styles.section}>
        <span className={styles.label}>Manifest</span>
        <div className={styles.manifestRow}>
          <span className={styles.manifestName}>{meta.manifest_name}</span>
          <span className={styles.typeBadge}>{meta.manifest_type}</span>
          {meta.manifest_tag && <span className={styles.tagBadge}>{meta.manifest_tag}</span>}
        </div>
      </div>

      <div className={styles.section}>
        <span className={styles.label}>Vulnerabilities</span>
        <div className={styles.osvList}>
          {meta.osv_records.map((id) => (
            <Link key={id} to={AppRoute.OsvRecord.replace(':name', id)} className={styles.osvBadge}>
              {id}
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}
