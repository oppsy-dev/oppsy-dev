import type { WebhookChannelConf } from '../../../../api/notification_channels';
import styles from '../ConfigurationTab.module.css';
import { Row } from '../Row/Row';
import { SecretRow } from './SecretRow/SecretRow';

type WebhookConfigurationProps = { conf: WebhookChannelConf };

export function WebhookConfiguration({ conf }: WebhookConfigurationProps) {
  return (
    <>
      <Row label="Endpoint URL">
        <code className={styles.urlValue}>{conf.webhook_url}</code>
      </Row>
      <SecretRow secret={conf.secret ?? undefined} />
    </>
  );
}
