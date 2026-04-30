import type { EmailChannelConf } from '../../../../api/notification_channels';
import styles from '../ConfigurationTab.module.css';
import { Row } from '../Row/Row';

type EmailConfigurationProps = { conf: EmailChannelConf };

export function EmailConfiguration({ conf }: EmailConfigurationProps) {
  return (
    <>
      <Row label="Sender">
        <span className={styles.chip}>{conf.from}</span>
      </Row>
      <Row label="Recipients">
        <div className={styles.chipList}>
          {conf.to.map((addr) => (
            <span key={addr} className={styles.chip}>
              {addr}
            </span>
          ))}
        </div>
      </Row>
    </>
  );
}
