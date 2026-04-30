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
  const [copied, setCopied] = useState(false);
  const iconRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const el = iconRef.current;
    if (!el) return;
    const sync = () => {
      el.style.width = `${el.offsetHeight}px`;
    };
    const ro = new ResizeObserver(sync);
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
