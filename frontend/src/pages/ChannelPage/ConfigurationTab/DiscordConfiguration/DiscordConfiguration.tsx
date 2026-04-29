import styles from '../ConfigurationTab.module.css';
import { Row } from '../Row/Row';

export type DiscordConf = { type: 'Discord'; discord_webhook_url: string };

type DiscordConfigurationProps = { conf: DiscordConf };

export function DiscordConfiguration({ conf }: DiscordConfigurationProps) {
  return (
    <Row label="Webhook URL">
      <code className={styles.urlValue}>{conf.discord_webhook_url}</code>
    </Row>
  );
}
