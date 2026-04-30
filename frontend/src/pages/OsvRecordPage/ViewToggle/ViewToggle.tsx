import { useLayoutEffect, useRef, useState } from 'react';
import type { CSSProperties } from 'react';
import styles from './ViewToggle.module.css';

export enum ViewMode {
  View = 'view',
  Raw = 'raw',
}

type ViewToggleProps = {
  viewMode: ViewMode;
  onChange: (mode: ViewMode) => void;
};

export function ViewToggle({ viewMode, onChange }: ViewToggleProps) {
  const viewBtnRef = useRef<HTMLButtonElement>(null);
  const rawBtnRef = useRef<HTMLButtonElement>(null);
  const [pillStyle, setPillStyle] = useState<CSSProperties>({});

  useLayoutEffect(() => {
    const btn = viewMode === ViewMode.View ? viewBtnRef.current : rawBtnRef.current;
    if (btn) {
      setPillStyle({ left: btn.offsetLeft, width: btn.offsetWidth });
    }
  }, [viewMode]);

  return (
    <div className={styles.toggle}>
      <div className={styles.togglePill} style={pillStyle} />
      <button
        ref={viewBtnRef}
        type="button"
        className={`${styles.toggleBtn}${viewMode === ViewMode.View ? ` ${styles.toggleBtnActive}` : ''}`}
        onClick={() => onChange(ViewMode.View)}
      >
        View
      </button>
      <button
        ref={rawBtnRef}
        type="button"
        className={`${styles.toggleBtn}${viewMode === ViewMode.Raw ? ` ${styles.toggleBtnActive}` : ''}`}
        onClick={() => onChange(ViewMode.Raw)}
      >
        Raw JSON
      </button>
    </div>
  );
}
