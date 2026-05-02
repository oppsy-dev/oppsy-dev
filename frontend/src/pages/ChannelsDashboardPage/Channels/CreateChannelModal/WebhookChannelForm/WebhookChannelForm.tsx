import type {
  CreateChannelRequest,
  WebhookChannelConf,
} from '../../../../../api/notification_channels';
import { TemplateField } from '../TemplateField/TemplateField';
import styles from '../CreateChannelModal.module.css';

export const WEBHOOK_DEFAULT_TEMPLATE = '';

export type WebhookFormState = {
  name: string;
  url: string;
  secret: string;
  template: string;
};

export function buildWebhookChannel(state: WebhookFormState): CreateChannelRequest | null {
  if (!state.name.trim() || !state.url.trim()) return null;
  return {
    name: state.name.trim(),
    conf: {
      type: 'Webhook',
      webhook_url: state.url.trim(),
      template: state.template,
      ...(state.secret.trim() ? { secret: state.secret.trim() } : {}),
    } as WebhookChannelConf,
  };
}

type WebhookChannelFormProps = {
  value: WebhookFormState;
  onChange: (v: WebhookFormState) => void;
};

export function WebhookChannelForm({ value, onChange }: WebhookChannelFormProps) {
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
          placeholder="e.g. PagerDuty"
        />
      </div>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="ch-url">
          Endpoint URL
        </label>
        <input
          id="ch-url"
          className={styles.input}
          type="url"
          value={value.url}
          onChange={(e) => onChange({ ...value, url: e.target.value })}
          placeholder="https://example.com/webhook"
        />
      </div>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="ch-secret">
          Signing secret <span className={styles.fieldLabelOptional}>(optional)</span>
        </label>
        <input
          id="ch-secret"
          className={styles.input}
          value={value.secret}
          onChange={(e) => onChange({ ...value, secret: e.target.value })}
          placeholder="Used to sign HMAC-SHA256 payloads"
        />
      </div>
      <TemplateField value={value.template} onChange={(v) => onChange({ ...value, template: v })} />
    </>
  );
}
