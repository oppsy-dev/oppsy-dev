import { useState } from 'react';
import styles from './ChannelSettingsSection.module.css';
import { BackIcon, TrashIcon } from '../../../components/Icons';
import type { NotificationChannel } from '../../../api/notification_channels';
import { useUpdateChannel } from '../../../hooks/notification_channels';

type Props = {
  channel: NotificationChannel;
  onBack: () => void;
  onDelete: () => Promise<void>;
  onDeleted: () => void;
};

export function ChannelSettingsSection({ channel, onBack, onDelete, onDeleted }: Props) {
  const [state, setState] = useState({
    active: channel.active,
    showConfirm: false,
    confirmText: '',
    deleting: false,
    deleteError: null as string | null,
  });

  const set = (patch: Partial<typeof state>) => setState((s) => ({ ...s, ...patch }));

  const { mutate: saveChannel, isPending: saving } = useUpdateChannel();

  const isDirty = state.active !== channel.active;

  const handleSave = () => {
    saveChannel({
      channelId: channel.id,
      req: { name: channel.name, conf: channel.conf, active: state.active },
    });
  };

  const handleDelete = async () => {
    set({ deleting: true, deleteError: null });
    try {
      await onDelete();
      onDeleted();
    } catch {
      set({ deleteError: 'Failed to delete channel. Please try again.', deleting: false });
    }
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
                {state.active
                  ? 'This channel is active and will receive notifications.'
                  : 'This channel is paused. No notifications will be delivered.'}
              </p>
            </div>
            <button
              type="button"
              aria-pressed={state.active}
              className={
                state.active
                  ? `${styles.toggle} ${styles.toggleOn}`
                  : `${styles.toggle} ${styles.toggleOff}`
              }
              onClick={() => set({ active: !state.active })}
            >
              <span className={styles.toggleThumb} />
            </button>
          </div>
        </div>
      </div>

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
      <div className={styles.dangerSection}>
        <div className={styles.dangerHeader}>
          <h3 className={styles.dangerTitle}>Danger zone</h3>
          <p className={styles.dangerDesc}>These actions are permanent and cannot be undone.</p>
        </div>

        {!state.showConfirm ? (
          <div className={styles.dangerPanel}>
            <div>
              <p className={styles.actionTitle}>Delete this notification channel</p>
              <p className={styles.actionDesc}>
                All notification events and settings will be permanently removed.
              </p>
            </div>
            <button
              type="button"
              className={styles.deleteBtn}
              onClick={() => set({ showConfirm: true })}
            >
              <TrashIcon width={13} height={13} />
              Delete
            </button>
          </div>
        ) : (
          <div className={styles.dangerPanelConfirm}>
            <p className={styles.confirmPrompt}>
              Type <code className={styles.confirmCode}>{channel.name}</code> to confirm deletion
            </p>
            <input
              className={styles.confirmInput}
              value={state.confirmText}
              onChange={(e) => set({ confirmText: e.target.value })}
              placeholder={channel.name}
              autoFocus
            />
            <div className={styles.confirmActions}>
              <button
                type="button"
                className={styles.deleteForeverBtn}
                onClick={handleDelete}
                disabled={state.confirmText !== channel.name || state.deleting}
              >
                <TrashIcon width={13} height={13} />
                {state.deleting ? 'Deleting…' : 'Permanently delete'}
              </button>
              <button
                type="button"
                className={styles.cancelBtn}
                onClick={() => set({ showConfirm: false, confirmText: '' })}
                disabled={state.deleting}
              >
                Cancel
              </button>
            </div>
            {state.deleteError && <p className={styles.error}>{state.deleteError}</p>}
          </div>
        )}
      </div>
    </div>
  );
}
