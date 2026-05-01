import type { EmailChannelConf } from '../../../../../api/notification_channels';
import styles from '../ChannelConfSection.module.css';

type Props = {
  value: EmailChannelConf;
  onChange: (v: EmailChannelConf) => void;
};

export function EmailChannelConfForm({ value, onChange }: Props) {
  const updateAddress = (index: number, next: string) =>
    onChange({ ...value, to: value.to.map((a, i) => (i === index ? next : a)) });

  const addAddress = () => onChange({ ...value, to: [...value.to, ''] });

  const removeAddress = (index: number) =>
    onChange({ ...value, to: value.to.filter((_, i) => i !== index) });

  return (
    <>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="conf-email-from">
          Sender address
        </label>
        <input
          id="conf-email-from"
          className={styles.input}
          type="email"
          value={value.from}
          onChange={(e) => onChange({ ...value, from: e.target.value })}
          placeholder="notifications@example.com"
        />
      </div>
      <div className={styles.field}>
        <label className={styles.fieldLabel}>Recipient addresses</label>
        <div className={styles.emailList}>
          {value.to.map((addr, i) => (
            // eslint-disable-next-line react/no-array-index-key
            <div key={i} className={styles.emailRow}>
              <input
                className={styles.input}
                type="email"
                value={addr}
                onChange={(e) => updateAddress(i, e.target.value)}
                placeholder="security@example.com"
                aria-label={`Recipient address ${i + 1}`}
              />
              {value.to.length > 1 && (
                <button
                  type="button"
                  className={styles.removeEmailBtn}
                  onClick={() => removeAddress(i)}
                  aria-label={`Remove recipient address ${i + 1}`}
                >
                  ×
                </button>
              )}
            </div>
          ))}
        </div>
        <button type="button" className={styles.addEmailBtn} onClick={addAddress}>
          + Add another recipient
        </button>
      </div>
    </>
  );
}
