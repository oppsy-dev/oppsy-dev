import { useState, useRef, useEffect } from 'react';
import type { CSSProperties } from 'react';
import { CopyIcon, GearIcon } from '../Icons';
import { formatUuidV7Date } from '../../utils/uuidV7';
import styles from './PageHeader.module.css';

type Props = {
  name: string;
  id: string;
  icon: React.ReactNode;
  iconStyle?: CSSProperties;
  badges?: React.ReactNode;
  onSettingsClick: () => void;
  settingsActive: boolean;
};

export function PageHeader({
  name,
  id,
  icon,
  iconStyle,
  badges,
  onSettingsClick,
  settingsActive,
}: Props) {
  // Tracks whether the ID was just copied, to show a brief confirmation state.
  const [copied, setCopied] = useState(false);
  const iconRef = useRef<HTMLDivElement>(null);

  // Keeps the icon wrapper square by matching its width to its current height.
  // The height is driven by the sibling text content via flex-stretch, so CSS
  // aspect-ratio alone doesn't work here — we need to read the computed height
  // and set width explicitly.
  // requestAnimationFrame defers the DOM write to after the ResizeObserver
  // batch completes, preventing the observer from re-triggering itself
  // synchronously and causing a "ResizeObserver loop" browser error.
  useEffect(() => {
    const el = iconRef.current;
    if (!el) return;
    const ro = new ResizeObserver(() => {
      requestAnimationFrame(() => {
        el.style.width = `${el.offsetHeight}px`;
      });
    });
    ro.observe(el);
    return () => ro.disconnect();
  }, []);

  return (
    <div className={styles.header}>
      <div className={styles.left}>
        <div className={styles.info}>
          <div ref={iconRef} className={styles.iconWrap} style={iconStyle}>
            {icon}
          </div>
          <div>
            <h1 className={styles.title}>{name}</h1>
            <p className={styles.subline}>Created {formatUuidV7Date(id)}</p>
            <div className={styles.idRow}>
              <span className={styles.idLabel}>id:</span>
              {/* Clicking copies the full ID to the clipboard and shows a checkmark for 1.5 s. */}
              <button
                type="button"
                className={copied ? `${styles.idBadge} ${styles.idBadgeCopied}` : styles.idBadge}
                onClick={() => {
                  void navigator.clipboard.writeText(id);
                  setCopied(true);
                  setTimeout(() => setCopied(false), 1500);
                }}
              >
                {id}
                <span className={styles.idBadgeIcon}>
                  {copied ? '✓' : <CopyIcon width={10} height={10} />}
                </span>
              </button>
            </div>
          </div>
        </div>
        {badges && <div className={styles.badges}>{badges}</div>}
      </div>
      <button
        type="button"
        className={
          settingsActive ? `${styles.settingsBtn} ${styles.settingsBtnActive}` : styles.settingsBtn
        }
        onClick={onSettingsClick}
      >
        <GearIcon width={13} height={13} />
        Settings
      </button>
    </div>
  );
}
