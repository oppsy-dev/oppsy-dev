import type {
  CreateChannelRequest,
  DiscordChannelConf,
} from '../../../../../api/notification_channels';
import styles from '../CreateChannelModal.module.css';

const BT = '`';

export const DISCORD_DEFAULT_TEMPLATE = `_count: len(osv_records)
_plural: *"ies" | "y"
if _count == 1 {
\t_plural: "y"
}
manifest_tag: *null | string | null
_tag_line: *"" | string
if manifest_tag != null {
\t_tag_line: "\\n**Tag:** ${BT}\\(manifest_tag)${BT}"
}

content: ":shield: **OPPSY** detected **\\(_count)** new open-source vulnerabilit\\(_plural)\\n\\n**Workspace:** \\(workspace_name)\\n**Manifest:** ${BT}\\(manifest_name)${BT} (\\(manifest_type))\\(_tag_line)\\n\\n*To stop receiving these notifications, disable or delete this channel in OPPSY.*"`;

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
    </>
  );
}
