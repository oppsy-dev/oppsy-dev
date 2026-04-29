import type { NotificationChannel } from '../../../../api/notification_channels';
import {
  ChannelIcon,
  CHANNEL_ICON_BG,
  CHANNEL_ICON_COLOR,
  XIcon,
} from '../../../../components/Icons';
import styles from './PickChannelModal.module.css';

type Props = {
  channels: NotificationChannel[];
  onPick: (channel: NotificationChannel) => void;
  onClose: () => void;
};

export function PickChannelModal({ channels, onPick, onClose }: Props) {
  return (
    <div className={styles.backdrop}>
      <div className={styles.modal} role="dialog" aria-modal="true">
        <div className={styles.header}>
          <h2 className={styles.title}>Link notification channel</h2>
          <button type="button" className={styles.closeBtn} onClick={onClose} aria-label="Close">
            <XIcon width={16} height={16} />
          </button>
        </div>

        <p className={styles.desc}>
          Select an existing channel to receive vulnerability alerts from this workspace.
        </p>

        <ul className={styles.list}>
          {channels.map((ch) => {
            return (
              <li key={ch.id}>
                <button type="button" className={styles.row} onClick={() => onPick(ch)}>
                  <span
                    className={styles.iconWrap}
                    style={{
                      background: CHANNEL_ICON_BG[ch.conf.type],
                      color: CHANNEL_ICON_COLOR[ch.conf.type],
                    }}
                  >
                    <ChannelIcon type={ch.conf.type} width={18} height={18} />
                  </span>
                  <span className={styles.rowInfo}>
                    <span className={styles.rowName}>{ch.name}</span>
                    <span
                      className={styles.rowType}
                      style={{ color: CHANNEL_ICON_COLOR[ch.conf.type] }}
                    >
                      {ch.conf.type}
                    </span>
                  </span>
                  <span className={ch.active ? styles.activeBadge : styles.inactiveBadge}>
                    {ch.active ? 'Active' : 'Inactive'}
                  </span>
                </button>
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
}
