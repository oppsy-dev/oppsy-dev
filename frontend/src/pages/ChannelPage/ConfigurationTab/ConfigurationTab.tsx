import type {
  ChannelConf,
  DiscordChannelConf,
  EmailChannelConf,
  NotificationChannel,
  NotificationChannelType,
  WebhookChannelConf,
} from '../../../api/notification_channels';
import styles from './ConfigurationTab.module.css';
import { DiscordConfiguration } from './DiscordConfiguration/DiscordConfiguration';
import { EmailConfiguration } from './EmailConfiguration/EmailConfiguration';
import { WebhookConfiguration } from './WebhookConfiguration/WebhookConfiguration';

type Props = { channel: NotificationChannel };

const CONF_RENDERERS: Record<
  NotificationChannelType,
  (conf: ChannelConf) => React.ReactElement
> = {
  Webhook: (conf) => <WebhookConfiguration conf={conf as WebhookChannelConf} />,
  Discord: (conf) => <DiscordConfiguration conf={conf as DiscordChannelConf} />,
  Email: (conf) => <EmailConfiguration conf={conf as EmailChannelConf} />,
};

const CHANNEL_DESCRIPTIONS: Record<NotificationChannelType, string> = {
  Webhook: 'Vulnerability alerts are delivered as signed HTTP POST requests to the endpoint below.',
  Discord: 'Vulnerability alerts are posted to a Discord channel via the incoming webhook below.',
  Email: 'Vulnerability alert digests are sent to the email addresses listed below.',
};

export function ConfigurationTab({ channel }: Props) {
  return (
    <div className={styles.container}>
      <section className={styles.section}>
        <p className={styles.sectionDesc}>
          {CHANNEL_DESCRIPTIONS[channel.conf.type]}
        </p>
        <div className={styles.card}>
          {CONF_RENDERERS[channel.conf.type](channel.conf)}
        </div>
      </section>
    </div>
  );
}
