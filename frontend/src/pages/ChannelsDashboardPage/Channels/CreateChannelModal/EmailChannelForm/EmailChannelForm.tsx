import type { CreateChannelRequest } from '../../../../../api/notification_channels';
import styles from '../CreateChannelModal.module.css';

export type EmailFormState = {
  name: string;
  addresses: string[];
};

export function buildEmailChannel(state: EmailFormState): CreateChannelRequest | null {
  const valid = state.addresses.map((a) => a.trim()).filter(Boolean);
  if (!state.name.trim() || valid.length === 0) return null;
  return {
    name: state.name.trim(),
    conf: {
      type: 'Email',
      to_addresses: valid,
    } as CreateChannelRequest['conf'],
  };
}

type EmailChannelFormProps = {
  value: EmailFormState;
  onChange: (v: EmailFormState) => void;
};

export function EmailChannelForm({ value, onChange }: EmailChannelFormProps) {
  const addAddress = () => onChange({ ...value, addresses: [...value.addresses, ''] });

  const removeAddress = (index: number) =>
    onChange({ ...value, addresses: value.addresses.filter((_, i) => i !== index) });

  const updateAddress = (index: number, next: string) =>
    onChange({ ...value, addresses: value.addresses.map((a, i) => (i === index ? next : a)) });

  return (
    <>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="ch-name">
          Channel name
        </label>
        <input
          id="ch-name"
          className={styles.input}
          value={value.name}
          onChange={(e) => onChange({ ...value, name: e.target.value })}
          placeholder="e.g. Security Digest"
        />
      </div>

      <div className={styles.field}>
        <label className={styles.fieldLabel}>Email addresses</label>
        <div className={styles.emailList}>
          {value.addresses.map((addr, i) => (
            // eslint-disable-next-line react/no-array-index-key
            <div key={i} className={styles.emailRow}>
              <input
                className={styles.input}
                type="email"
                value={addr}
                onChange={(e) => updateAddress(i, e.target.value)}
                placeholder="security@example.com"
                aria-label={`Email address ${i + 1}`}
              />
              {value.addresses.length > 1 && (
                <button
                  type="button"
                  className={styles.removeEmailBtn}
                  onClick={() => removeAddress(i)}
                  aria-label={`Remove email address ${i + 1}`}
                >
                  ×
                </button>
              )}
            </div>
          ))}
        </div>
        <button type="button" className={styles.addEmailBtn} onClick={addAddress}>
          + Add another email
        </button>
      </div>
    </>
  );
}
