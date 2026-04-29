import styles from '../ConfigurationTab.module.css';
import { Row } from '../Row/Row';

export type EmailConf = { type: 'Email'; to_addresses: string[] };

type EmailConfigurationProps = { conf: EmailConf };

export function EmailConfiguration({ conf }: EmailConfigurationProps) {
  return (
    <Row label="Recipients">
      <div className={styles.chipList}>
        {conf.to_addresses.map((addr) => (
          <span key={addr} className={styles.chip}>
            {addr}
          </span>
        ))}
      </div>
    </Row>
  );
}
