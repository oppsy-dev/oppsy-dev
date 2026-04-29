import { useState } from 'react';
import styles from './CreateWorkspaceModal.module.css';
import { XIcon } from '../../../../components/Icons';

type CreateWorkspaceModalProps = {
  onClose: () => void;
  onConfirm: (name: string) => Promise<void>;
};

export function CreateWorkspaceModal({ onClose, onConfirm }: CreateWorkspaceModalProps) {
  const [name, setName] = useState('');
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleConfirm = async () => {
    setCreating(true);
    setError(null);
    try {
      await onConfirm(name.trim() || 'Untitled workspace');
    } catch (e) {
      setError((e as Error).message);
      setCreating(false);
    }
  };

  return (
    <div className={styles.backdrop}>
      <div className={styles.modal} role="dialog" aria-modal="true" aria-labelledby="modal-title">
        {/* Header */}
        <div className={styles.modalHeader}>
          <h2 id="modal-title" className={styles.modalTitle}>
            New Workspace
          </h2>
          <button className={styles.closeBtn} type="button" onClick={onClose} aria-label="Close">
            <XIcon width={16} height={16} />
          </button>
        </div>

        {/* Name field */}
        <div className={styles.field}>
          <label className={styles.fieldLabel} htmlFor="workspace-name">
            Workspace name
          </label>
          <input
            id="workspace-name"
            className={styles.nameInput}
            type="text"
            placeholder="e.g. My Project"
            value={name}
            onChange={(e) => setName(e.target.value)}
            maxLength={80}
            autoFocus
            autoComplete="off"
          />
        </div>

        {/* Error */}
        {error && (
          <p className={styles.error}>
            'Could not reach the server. Check that the backend is running.'
          </p>
        )}

        {/* Footer */}
        <div className={styles.modalFooter}>
          <button className={styles.cancelBtn} type="button" onClick={onClose} disabled={creating}>
            Cancel
          </button>
          <button
            className={styles.confirmBtn}
            type="button"
            onClick={handleConfirm}
            disabled={creating}
          >
            {creating ? 'Creating…' : 'Create workspace'}
          </button>
        </div>
      </div>
    </div>
  );
}
