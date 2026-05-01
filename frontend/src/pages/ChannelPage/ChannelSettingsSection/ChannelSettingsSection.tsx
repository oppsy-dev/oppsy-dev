import { useState } from 'react';
import styles from './ChannelSettingsSection.module.css';
import { BackIcon } from '../../../components/Icons';
import type { ChannelConf, NotificationChannel } from '../../../api/notification_channels';
import { useUpdateChannel } from '../../../hooks/notification_channels';
import { DangerZone } from '../../../components/DangerZone/DangerZone';
import { ChannelConfSection } from './ChannelConfSection/ChannelConfSection';

type Props = {
  channel: NotificationChannel;
  onBack: () => void;
  onDelete: () => Promise<void>;
  onDeleted: () => void;
};

export function ChannelSettingsSection({ channel, onBack, onDelete, onDeleted }: Props) {
  const [active, setActive] = useState(channel.active);
  const [conf, setConf] = useState<ChannelConf>(channel.conf as unknown as ChannelConf);

  const { mutate: saveChannel, isPending: saving } = useUpdateChannel();

  const isDirty =
    active !== channel.active ||
    JSON.stringify(conf) !== JSON.stringify(channel.conf);

  const handleSave = () => {
    saveChannel({
      channelId: channel.id,
      req: { name: channel.name, active, conf: conf as unknown as NotificationChannel['conf'] },
    });
  };

  return (
    <div className={styles.settingsPage}>
      <button className={styles.backBtn} type="button" onClick={onBack}>
        <BackIcon width={13} height={13} />
        Back to channel
      </button>

      {/* ── Activity ── */}
      <div className={styles.section}>
        <div className={styles.sectionHeader}>
          <h3 className={styles.sectionTitle}>Activity</h3>
          <p className={styles.sectionDesc}>
            Pausing a channel stops all notifications without removing its configuration.
          </p>
        </div>
        <div className={styles.card}>
          <div className={styles.toggleRow}>
            <div className={styles.toggleInfo}>
              <p className={styles.toggleLabel}>Send notifications</p>
              <p className={styles.toggleDesc}>
                {active
                  ? 'This channel is active and will receive notifications.'
                  : 'This channel is paused. No notifications will be delivered.'}
              </p>
            </div>
            <button
              type="button"
              aria-pressed={active}
              className={active ? `${styles.toggle} ${styles.toggleOn}` : `${styles.toggle} ${styles.toggleOff}`}
              onClick={() => setActive((v) => !v)}
            >
              <span className={styles.toggleThumb} />
            </button>
          </div>
        </div>
      </div>

      {/* ── Configuration ── */}
      <ChannelConfSection value={conf} onChange={setConf} />

      {/* ── Save ── */}
      <div className={styles.formFooter}>
        <button
          type="button"
          className={styles.saveBtn}
          onClick={handleSave}
          disabled={!isDirty || saving}
        >
          {saving ? 'Saving…' : 'Save changes'}
        </button>
      </div>

      {/* ── Danger zone ── */}
      <DangerZone
        name={channel.name}
        title="Delete this notification channel"
        description="All notification events and settings will be permanently removed."
        onDelete={onDelete}
        onDeleted={onDeleted}
      />
    </div>
  );
}
