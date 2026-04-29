import { useState } from 'react';
import styles from './WorkspaceSettingsSection.module.css';
import { BackIcon, TrashIcon } from '../../../components/Icons';

type WorkspaceSettingsSectionProps = {
  workspaceName: string;
  onBack: () => void;
  onDelete: () => Promise<void>;
  onDeleted: () => void;
};

export function WorkspaceSettingsSection({
  workspaceName,
  onBack,
  onDelete,
  onDeleted,
}: WorkspaceSettingsSectionProps) {
  const [showConfirm, setShowConfirm] = useState(false);
  const [confirmText, setConfirmText] = useState('');
  const [deleting, setDeleting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleDelete = async () => {
    setDeleting(true);
    setError(null);
    try {
      await onDelete();
      onDeleted();
    } catch {
      setError('Failed to delete workspace. Please try again.');
      setDeleting(false);
    }
  };

  return (
    <div>
      <button className={styles.backBtn} type="button" onClick={onBack}>
        <BackIcon width={13} height={13} />
        Back to workspace
      </button>

      <div className={styles.dangerSection}>
        <div className={styles.dangerHeader}>
          <h3 className={styles.dangerTitle}>Danger zone</h3>
          <p className={styles.dangerDesc}>These actions are permanent and cannot be undone.</p>
        </div>

        {!showConfirm ? (
          <div className={styles.dangerPanel}>
            <div>
              <p className={styles.actionTitle}>Delete this workspace</p>
              <p className={styles.actionDesc}>
                All manifest files, scan data, and settings will be permanently removed.
              </p>
            </div>
            <button type="button" className={styles.deleteBtn} onClick={() => setShowConfirm(true)}>
              <TrashIcon width={13} height={13} />
              Delete
            </button>
          </div>
        ) : (
          <div className={styles.dangerPanelConfirm}>
            <p className={styles.confirmPrompt}>
              Type <code className={styles.confirmCode}>{workspaceName}</code> to confirm deletion
            </p>
            <input
              className={styles.confirmInput}
              value={confirmText}
              onChange={(e) => setConfirmText(e.target.value)}
              placeholder={workspaceName}
              autoFocus
            />
            <div className={styles.confirmActions}>
              <button
                type="button"
                className={styles.deleteForeverBtn}
                onClick={handleDelete}
                disabled={confirmText !== workspaceName || deleting}
              >
                <TrashIcon width={13} height={13} />
                {deleting ? 'Deleting…' : 'Permanently delete'}
              </button>
              <button
                type="button"
                className={styles.cancelBtn}
                onClick={() => {
                  setShowConfirm(false);
                  setConfirmText('');
                }}
                disabled={deleting}
              >
                Cancel
              </button>
            </div>
            {error && <p className={styles.error}>{error}</p>}
          </div>
        )}
      </div>
    </div>
  );
}
