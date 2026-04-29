import { Link } from 'react-router-dom';
import type { NotificationChannel } from '../../../../api/notification_channels';
import { AppRoute } from '../../../../routes/Routes';
import {
  ChannelIcon,
  CHANNEL_ICON_BG,
  CHANNEL_ICON_COLOR,
  TrashIcon,
} from '../../../../components/Icons';
import styles from './ChannelCard.module.css';

type ChannelCardProps = {
  channel: NotificationChannel;
  onRemove: () => void;
};

export function ChannelCard({ channel, onRemove }: ChannelCardProps) {
  return (
    <Link
      to={AppRoute.Channel.replace(':channelId', channel.id)}
      style={{ textDecoration: 'none' }}
    >
      <div className={styles.card}>
        <div
          className={styles.iconWrap}
          style={{
            color: CHANNEL_ICON_COLOR[channel.conf.type],
            background: CHANNEL_ICON_BG[channel.conf.type],
          }}
        >
          <ChannelIcon type={channel.conf.type} width={18} height={18} />
        </div>

        <div className={styles.info}>{channel.name}</div>

        <div
          className={
            channel.active
              ? `${styles.statusBadge} ${styles.statusActive}`
              : `${styles.statusBadge} ${styles.statusInactive}`
          }
        >
          <span className={styles.statusDot} />
          {channel.active ? 'Active' : 'Inactive'}
        </div>

        <button
          type="button"
          className={styles.trashBtn}
          aria-label="Remove channel"
          onClick={(e) => {
            e.preventDefault();
            e.stopPropagation();
            onRemove();
          }}
        >
          <TrashIcon width={14} height={14} />
        </button>
      </div>
    </Link>
  );
}
