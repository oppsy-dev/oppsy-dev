import type { DiscordChannelConf } from '../../../../../api/notification_channels';
import { TemplateField } from '../../../../ChannelsDashboardPage/Channels/CreateChannelModal/TemplateField/TemplateField';
import { DISCORD_TEMPLATE_SCHEMA } from '../../../../ChannelsDashboardPage/Channels/CreateChannelModal/DiscordChannelForm/DiscordChannelForm';
import styles from '../ChannelConfSection.module.css';

type Props = {
  value: DiscordChannelConf;
  onChange: (v: DiscordChannelConf) => void;
};

export function DiscordChannelConfForm({ value, onChange }: Props) {
  return (
    <>
      <div className={styles.field}>
        <label className={styles.fieldLabel} htmlFor="conf-discord-url">
          Webhook URL
        </label>
        <input
          id="conf-discord-url"
          className={styles.input}
          type="url"
          value={value.discord_webhook_url}
          onChange={(e) => onChange({ ...value, discord_webhook_url: e.target.value })}
          placeholder="https://discord.com/api/webhooks/…"
        />
      </div>
      <TemplateField
        value={value.template}
        onChange={(v) => onChange({ ...value, template: v })}
        templateSchema={DISCORD_TEMPLATE_SCHEMA}
        alwaysOpen
      />
    </>
  );
}
