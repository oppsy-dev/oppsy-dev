import { useState } from 'react';
import { CreateTeamModal } from '../CreateTeamModal/CreateTeamModal';
import styles from './AddTeamButton.module.css';

export function AddTeamButton() {
  const [modalOpen, setModalOpen] = useState(false);

  const handleConfirm = async (name: string) => {
    setModalOpen(false);
  };

  return (
    <>
      <button className={styles.button} onClick={() => setModalOpen(true)} type="button">
        <span className={styles.icon}>+</span>
        <span className={styles.label}>Add team</span>
      </button>

      {modalOpen && (
        <CreateTeamModal onClose={() => setModalOpen(false)} onConfirm={handleConfirm} />
      )}
    </>
  );
}
