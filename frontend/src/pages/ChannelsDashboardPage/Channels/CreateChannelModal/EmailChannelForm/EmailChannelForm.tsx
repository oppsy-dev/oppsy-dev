import type {
  CreateChannelRequest,
  EmailChannelConf,
} from '../../../../../api/notification_channels';
import { TemplateField } from '../TemplateField/TemplateField';
import styles from '../CreateChannelModal.module.css';

export const EMAIL_DEFAULT_TEMPLATE = `
subject: """
[OPPSY] New vulnerabilities detected in \\(_workspace_name)/\\(_manifest_name)
"""

body: "OPPSY detected
Workspace:      \\(_workspace_name)
Manifest:       \\(_manifest_name) (\\(_manifest_type))
Review each finding at https://osv.dev and assess whether your project is affected.
--
To stop receiving these emails, disable or delete this notification channel in OPPSY.
""""`;

export type EmailFormState = {
  name: string;
  from: string;
  addresses: string[];
  template: string;
};

export function buildEmailChannel(state: EmailFormState): CreateChannelRequest | null {
  const validTo = state.addresses.map((a) => a.trim()).filter(Boolean);
  if (!state.name.trim() || !state.from.trim() || validTo.length === 0) return null;
  return {
    name: state.name.trim(),
    conf: {
      type: 'Email',
      from: state.from.trim(),
      to: validTo,
      template: state.template,
    } as EmailChannelConf,
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
        <label className={styles.fieldLabel} htmlFor="ch-email-from">
          Sender address
        </label>
        <input
          id="ch-email-from"
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
          {value.addresses.map((addr, i) => (
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
              {value.addresses.length > 1 && (
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

      <TemplateField value={value.template} onChange={(v) => onChange({ ...value, template: v })} />
    </>
  );
}
