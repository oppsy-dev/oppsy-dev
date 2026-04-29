import { useState } from 'react';
import styles from '../../ConfigurationTab.module.css';
import { Row } from '../../Row/Row';

type SecretRowProps = { secret: string | undefined };

export function SecretRow({ secret }: SecretRowProps) {
  const [revealed, setRevealed] = useState(false);

  if (!secret) {
    return (
      <Row label="Signing secret">
        <span className={styles.notSet}>Not configured</span>
      </Row>
    );
  }

  return (
    <Row label="Signing secret">
      <span className={revealed ? styles.secretRevealed : styles.secretMasked}>
        {revealed ? secret : '•'.repeat(Math.min(secret.length, 24))}
      </span>
      <button type="button" className={styles.revealBtn} onClick={() => setRevealed((r) => !r)}>
        {revealed ? 'Hide' : 'Reveal'}
      </button>
    </Row>
  );
}
