import { useState } from 'react';
import { CodeType, CodeView } from '../../../../components/CodeView/CodeView';
import styles from './CreateChannelModal.module.css';

type TemplateFieldProps = {
  value: string;
  onChange: (value: string) => void;
};

export function TemplateField({ value, onChange }: TemplateFieldProps) {
  const [open, setOpen] = useState(false);

  return (
    <div className={styles.field}>
      <div className={styles.fieldToggleRow}>
        <span className={styles.fieldLabel}>Payload template</span>
        <button
          type="button"
          className={styles.fieldToggleBtn}
          onClick={() => setOpen((o) => !o)}
        >
          {open ? 'Hide' : 'Customize'}
        </button>
      </div>
      {open && (
        <CodeView
          code={value}
          type={CodeType.CUE}
          height="220px"
          onChange={onChange}
        />
      )}
    </div>
  );
}
