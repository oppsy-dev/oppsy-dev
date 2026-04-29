import type {
  NotificationChannel,
  NotificationChannelType,
} from '../../../api/notification_channels';
import styles from './ConfigurationTab.module.css';
import {
  DiscordConfiguration,
  type DiscordConf,
} from './DiscordConfiguration/DiscordConfiguration';
import { EmailConfiguration, type EmailConf } from './EmailConfiguration/EmailConfiguration';
import {
  WebhookConfiguration,
  type WebhookConf,
} from './WebhookConfiguration/WebhookConfiguration';

type NotificationChannelConf = WebhookConf | DiscordConf | EmailConf;

type Props = { channel: NotificationChannel };

const CONF_RENDERERS: Record<
  NotificationChannelType,
  (conf: NotificationChannelConf) => React.ReactElement
> = {
  Webhook: (conf) => <WebhookConfiguration conf={conf as WebhookConf} />,
  Discord: (conf) => <DiscordConfiguration conf={conf as DiscordConf} />,
  Email: (conf) => <EmailConfiguration conf={conf as EmailConf} />,
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
          {CHANNEL_DESCRIPTIONS[channel.conf.type as NotificationChannelType]}
        </p>
        <div className={styles.card}>
          {CONF_RENDERERS[channel.conf.type as NotificationChannelType](
            channel.conf as NotificationChannelConf,
          )}
        </div>
      </section>
    </div>
  );
}
