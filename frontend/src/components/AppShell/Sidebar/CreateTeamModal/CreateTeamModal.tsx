import { useState } from 'react';
import { createPortal } from 'react-dom';
import styles from './CreateTeamModal.module.css';
import { XIcon } from '../../../Icons';

type CreateTeamModalProps = {
  onClose: () => void;
  onConfirm: (name: string) => Promise<void>;
};

export function CreateTeamModal({ onClose, onConfirm }: CreateTeamModalProps) {
  const [name, setName] = useState('');
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleConfirm = async () => {
    setCreating(true);
    setError(null);
    try {
      await onConfirm(name.trim() || 'Untitled team');
    } catch {
      setError('Could not reach the server. Check that the backend is running.');
      setCreating(false);
    }
  };

  return createPortal(
    <div className={styles.backdrop}>
      <div className={styles.modal} role="dialog" aria-modal="true" aria-labelledby="modal-title">
        <div className={styles.modalHeader}>
          <h2 id="modal-title" className={styles.modalTitle}>
            New Team
          </h2>
          <button className={styles.closeBtn} type="button" onClick={onClose} aria-label="Close">
            <XIcon width={16} height={16} />
          </button>
        </div>

        <div className={styles.field}>
          <label className={styles.fieldLabel} htmlFor="team-name">
            Team name
          </label>
          <input
            id="team-name"
            className={styles.nameInput}
            type="text"
            placeholder="e.g. Engineering"
            value={name}
            onChange={(e) => setName(e.target.value)}
            maxLength={80}
            autoFocus
            autoComplete="off"
          />
        </div>

        {error && <p className={styles.error}>{error}</p>}

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
            {creating ? 'Creating…' : 'Create team'}
          </button>
        </div>
      </div>
    </div>,
    document.body,
  );
}
