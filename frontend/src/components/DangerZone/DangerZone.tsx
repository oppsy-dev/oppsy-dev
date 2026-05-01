import { useState } from 'react';
import styles from './DangerZone.module.css';
import { TrashIcon } from '../Icons';

type Props = {
  name: string;
  title: string;
  description: string;
  onDelete: () => Promise<void>;
  onDeleted: () => void;
};

export function DangerZone({ name, title, description, onDelete, onDeleted }: Props) {
  const [showConfirm, setShowConfirm] = useState(false);
  const [confirmText, setConfirmText] = useState('');
  const [deleting, setDeleting] = useState(false);
  const [deleteError, setDeleteError] = useState<string | null>(null);

  const handleDelete = async () => {
    setDeleting(true);
    setDeleteError(null);
    try {
      await onDelete();
      onDeleted();
    } catch {
      setDeleteError('Failed to delete. Please try again.');
      setDeleting(false);
    }
  };

  return (
    <div className={styles.dangerSection}>
      <div className={styles.dangerHeader}>
        <h3 className={styles.dangerTitle}>Danger zone</h3>
        <p className={styles.dangerDesc}>These actions are permanent and cannot be undone.</p>
      </div>

      {!showConfirm ? (
        <div className={styles.dangerPanel}>
          <div>
            <p className={styles.actionTitle}>{title}</p>
            <p className={styles.actionDesc}>{description}</p>
          </div>
          <button
            type="button"
            className={styles.deleteBtn}
            onClick={() => setShowConfirm(true)}
          >
            <TrashIcon width={13} height={13} />
            Delete
          </button>
        </div>
      ) : (
        <div className={styles.dangerPanelConfirm}>
          <p className={styles.confirmPrompt}>
            Type <code className={styles.confirmCode}>{name}</code> to confirm deletion
          </p>
          <input
            className={styles.confirmInput}
            value={confirmText}
            onChange={(e) => setConfirmText(e.target.value)}
            placeholder={name}
            autoFocus
          />
          <div className={styles.confirmActions}>
            <button
              type="button"
              className={styles.deleteForeverBtn}
              onClick={handleDelete}
              disabled={confirmText !== name || deleting}
            >
              <TrashIcon width={13} height={13} />
              {deleting ? 'Deleting…' : 'Permanently delete'}
            </button>
            <button
              type="button"
              className={styles.cancelBtn}
              onClick={() => { setShowConfirm(false); setConfirmText(''); }}
              disabled={deleting}
            >
              Cancel
            </button>
          </div>
          {deleteError && <p className={styles.error}>{deleteError}</p>}
        </div>
      )}
    </div>
  );
}
