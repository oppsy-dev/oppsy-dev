import type {
  CreateChannelRequest,
  DiscordChannelConf,
} from '../../../../../api/notification_channels';
import { TemplateField } from '../TemplateField/TemplateField';
import styles from '../CreateChannelModal.module.css';

const BT = '`';

export const DISCORD_DEFAULT_TEMPLATE = `
content: """
:shield: **OPPSY** detected new open-source vulnerabilities

**Workspace:** \\(_workspace_name)
**Manifest:** ${BT}\\(_manifest_name)${BT} (\\(_manifest_type))
*To stop receiving these notifications, disable or delete this channel in OPPSY.*
"""`;

export type DiscordFormState = {
  name: string;
  webhookUrl: string;
  template: string;
};

export function buildDiscordChannel(state: DiscordFormState): CreateChannelRequest | null {
  if (!state.name.trim() || !state.webhookUrl.trim()) return null;
  return {
    name: state.name.trim(),
    conf: {
      type: 'Discord',
      discord_webhook_url: state.webhookUrl.trim(),
      template: state.template,
    } as DiscordChannelConf,
  };
}

type DiscordChannelFormProps = {
  value: DiscordFormState;
  onChange: (v: DiscordFormState) => void;
};

export function DiscordChannelForm({ value, onChange }: DiscordChannelFormProps) {
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
          placeholder="e.g. #vulnerabilities"
        />
      </div>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="ch-webhook-url">
          Webhook URL
        </label>
        <input
          id="ch-webhook-url"
          className={styles.input}
          type="url"
          value={value.webhookUrl}
          onChange={(e) => onChange({ ...value, webhookUrl: e.target.value })}
          placeholder="https://discord.com/api/webhooks/…"
        />
      </div>
      <TemplateField value={value.template} onChange={(v) => onChange({ ...value, template: v })} />
    </>
  );
}
