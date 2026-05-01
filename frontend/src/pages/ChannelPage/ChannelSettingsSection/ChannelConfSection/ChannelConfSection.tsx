import styles from './ChannelConfSection.module.css';
import type { ChannelConf } from '../../../../api/notification_channels';
import { WebhookChannelConfForm } from './WebhookChannelConfForm/WebhookChannelConfForm';
import { DiscordChannelConfForm } from './DiscordChannelConfForm/DiscordChannelConfForm';
import { EmailChannelConfForm } from './EmailChannelConfForm/EmailChannelConfForm';

const DESCRIPTIONS: Record<string, string> = {
  Webhook: 'Configure the endpoint URL and optional HMAC signing secret.',
  Discord: 'Configure the Discord webhook URL.',
  Email: 'Configure the sender address and recipients.',
};

type Props = {
  value: ChannelConf;
  onChange: (v: ChannelConf) => void;
};

export function ChannelConfSection({ value, onChange }: Props) {
  return (
    <div className={styles.section}>
      <div className={styles.sectionHeader}>
        <h3 className={styles.sectionTitle}>{value.type} configuration</h3>
        <p className={styles.sectionDesc}>{DESCRIPTIONS[value.type]}</p>
      </div>
      <div className={styles.card}>
        {value.type === 'Webhook' && <WebhookChannelConfForm value={value} onChange={onChange} />}
        {value.type === 'Discord' && <DiscordChannelConfForm value={value} onChange={onChange} />}
        {value.type === 'Email' && <EmailChannelConfForm value={value} onChange={onChange} />}
      </div>
    </div>
  );
}
