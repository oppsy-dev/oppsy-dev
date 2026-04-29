import styles from '../ConfigurationTab.module.css';
import { Row } from '../Row/Row';
import { SecretRow } from './SecretRow/SecretRow';

export type WebhookConf = { type: 'Webhook'; webhook_url: string; secret?: string };

type WebhookConfigurationProps = { conf: WebhookConf };

export function WebhookConfiguration({ conf }: WebhookConfigurationProps) {
  return (
    <>
      <Row label="Endpoint URL">
        <code className={styles.urlValue}>{conf.webhook_url}</code>
      </Row>
      <SecretRow secret={conf.secret} />
    </>
  );
}
