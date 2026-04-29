import { ChannelIcon, CHANNEL_ICON_BG, CHANNEL_ICON_COLOR } from '../../../../../components/Icons';
import type { NotificationChannelType } from '../../../../../api/notification_channels';
import styles from './ChannelTypePicker.module.css';

export type { NotificationChannelType };

type ChannelTypeOption = {
  id: NotificationChannelType;
  label: string;
  description: string;
};

export const CHANNEL_TYPES: ChannelTypeOption[] = [
  {
    id: 'Discord',
    label: 'Discord',
    description: 'Post vulnerability alerts to a Discord channel',
  },
  {
    id: 'Webhook',
    label: 'Webhook',
    description: 'Send signed HTTP POST requests to any endpoint',
  },
  {
    id: 'Email',
    label: 'Email',
    description: 'Receive vulnerability digest emails',
  },
];

type ChannelTypePickerProps = {
  onSelect: (type: NotificationChannelType) => void;
};

export function ChannelTypePicker({ onSelect }: ChannelTypePickerProps) {
  return (
    <div className={styles.grid}>
      {CHANNEL_TYPES.map((ct) => {
        return (
          <button key={ct.id} type="button" className={styles.card} onClick={() => onSelect(ct.id)}>
            <div
              className={styles.cardIcon}
              style={{ color: CHANNEL_ICON_COLOR[ct.id], background: CHANNEL_ICON_BG[ct.id] }}
            >
              <ChannelIcon type={ct.id} width={22} height={22} />
            </div>
            <div className={styles.cardLabel}>{ct.label}</div>
            <div className={styles.cardDesc}>{ct.description}</div>
          </button>
        );
      })}
    </div>
  );
}
