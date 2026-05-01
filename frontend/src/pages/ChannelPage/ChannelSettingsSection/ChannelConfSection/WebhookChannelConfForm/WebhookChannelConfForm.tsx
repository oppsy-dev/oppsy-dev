import type { WebhookChannelConf } from '../../../../../api/notification_channels';
import { TemplateField } from '../../../../ChannelsDashboardPage/Channels/CreateChannelModal/TemplateField/TemplateField';
import styles from '../ChannelConfSection.module.css';

type Props = {
  value: WebhookChannelConf;
  onChange: (v: WebhookChannelConf) => void;
};

export function WebhookChannelConfForm({ value, onChange }: Props) {
  return (
    <>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="conf-webhook-url">
          Endpoint URL
        </label>
        <input
          id="conf-webhook-url"
          className={styles.input}
          type="url"
          value={value.webhook_url}
          onChange={(e) => onChange({ ...value, webhook_url: e.target.value })}
          placeholder="https://example.com/webhook"
        />
      </div>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="conf-webhook-secret">
          Signing secret <span className={styles.fieldLabelOptional}>(optional)</span>
        </label>
        <input
          id="conf-webhook-secret"
          className={styles.input}
          value={value.secret ?? ''}
          onChange={(e) => onChange({ ...value, secret: e.target.value || null })}
          placeholder="Used to sign HMAC-SHA256 payloads"
        />
      </div>
      <TemplateField value={value.template} onChange={(v) => onChange({ ...value, template: v })} alwaysOpen />
    </>
  );
}
