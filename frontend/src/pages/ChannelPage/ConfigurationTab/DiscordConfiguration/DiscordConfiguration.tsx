import type { DiscordChannelConf } from '../../../../api/notification_channels';
import styles from '../ConfigurationTab.module.css';
import { Row } from '../Row/Row';

type DiscordConfigurationProps = { conf: DiscordChannelConf };

export function DiscordConfiguration({ conf }: DiscordConfigurationProps) {
  return (
    <Row label="Webhook URL">
      <code className={styles.urlValue}>{conf.discord_webhook_url}</code>
    </Row>
  );
}
