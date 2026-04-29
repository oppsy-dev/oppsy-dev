import { Link } from 'react-router-dom';
import type { components } from '../../../../api/schema';
import styles from './ChannelCard.module.css';
import { ChannelIcon, CHANNEL_ICON_BG } from '../../../../components/Icons';

type ChannelCardProps = {
  channel: components['schemas']['NotificationChannel'];
};

export function ChannelCard({ channel }: ChannelCardProps) {
  return (
    <Link to={`/channels/${channel.id}`} className={styles.card}>
      <div className={styles.cardTop}>
        <div className={styles.iconWrap} style={{ background: CHANNEL_ICON_BG[channel.conf.type] }}>
          <ChannelIcon type={channel.conf.type} width={17} height={17} />
        </div>
      </div>
      <div className={styles.cardBody}>
        <p className={styles.cardLabel}>Channel</p>
        <p className={styles.cardId}>{channel.name}</p>
      </div>
    </Link>
  );
}
