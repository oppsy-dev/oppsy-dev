import { Link } from 'react-router-dom';
import { AppRoute } from '../../../routes/Routes';
import { ChannelIcon, CHANNEL_ICON_COLOR } from '../../Icons';
import type { NotificationChannelType } from '../../../api/notification_channels';
import styles from './EventFeedCard.module.css';

export type EventFeedEvent = {
  id: string;
  channelId: string;
  channelName: string;
  channelType: 'Webhook' | 'Discord' | 'Email';
  workspaceName: string;
  manifestName: string;
  manifestTag?: string;
  timeAgo: string;
  delivered: boolean;
};

type Props = { event: EventFeedEvent };

export function EventFeedCard({ event }: Props) {
  const to = AppRoute.Channel.replace(':channelId', event.channelId);
  const iconColor = CHANNEL_ICON_COLOR[event.channelType as NotificationChannelType];

  return (
    <Link to={to} className={styles.card}>
      <div className={styles.topRow}>
        <div className={styles.channelInfo}>
          <span className={styles.iconWrap} style={{ color: iconColor }}>
            <ChannelIcon type={event.channelType as NotificationChannelType} width={14} height={14} />
          </span>
          <span className={styles.channelName}>{event.channelName}</span>
        </div>
      </div>

      <div className={styles.workspace}>{event.workspaceName}</div>

      <div className={styles.manifestSection}>
        <span className={styles.manifestName}>{event.manifestName}</span>
        {event.manifestTag && (
          <>
            <span className={styles.manifestDivider}>|</span>
            <span className={styles.manifestTag}>{event.manifestTag}</span>
          </>
        )}
      </div>

      <div className={styles.bottomRow}>
        <span className={styles.time}>{event.timeAgo}</span>
        {event.delivered ? (
          <span className={styles.deliveredBadge}>Delivered</span>
        ) : (
          <span className={styles.failedBadge}>Failed</span>
        )}
      </div>
    </Link>
  );
}
