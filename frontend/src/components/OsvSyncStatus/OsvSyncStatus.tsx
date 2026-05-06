import { useEffect, useState } from 'react';
import { useOsvSyncStatus } from '../../hooks/osv';
import styles from './OsvSyncStatus.module.css';

const RING_R = 14;
const RING_CIRCUMFERENCE = 2 * Math.PI * RING_R;

function formatElapsed(secs: number): string {
  if (secs < 60) return `${secs}s ago`;
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return s > 0 ? `${m}m ${s}s ago` : `${m}m ago`;
}

function formatNext(remainingSecs: number): string {
  if (remainingSecs <= 0) return 'overdue';
  return `~${Math.ceil(remainingSecs / 60)}m`;
}

export function OsvSyncStatus() {
  const { data } = useOsvSyncStatus();

  const lastSyncUnix = data ? Math.floor(new Date(data.last_sync_at).getTime() / 1000) : null;
  const intervalSecs = data?.sync_interval ?? 0;

  const [elapsedSecs, setElapsedSecs] = useState(() =>
    lastSyncUnix != null ? Math.floor(Date.now() / 1000) - lastSyncUnix : 0,
  );

  useEffect(() => {
    if (lastSyncUnix == null) return;
    setElapsedSecs(Math.floor(Date.now() / 1000) - lastSyncUnix);
    const id = setInterval(() => {
      setElapsedSecs(Math.floor(Date.now() / 1000) - lastSyncUnix);
    }, 1000);
    return () => clearInterval(id);
  }, [lastSyncUnix]);

  if (!data) return null;

  const progress = intervalSecs > 0 ? Math.min(elapsedSecs / intervalSecs, 1) : 0;
  const dashOffset = RING_CIRCUMFERENCE * (1 - progress);
  const isStale = progress >= 1;

  return (
    <div className={[styles.widget, isStale ? styles.stale : ''].filter(Boolean).join(' ')}>
      <div className={styles.ringWrap}>
        <svg width="40" height="40" viewBox="0 0 40 40">
          <circle className={styles.ringTrack} cx="20" cy="20" r={RING_R} />
          <circle
            className={[styles.ringFill, isStale ? styles.stale : ''].filter(Boolean).join(' ')}
            cx="20"
            cy="20"
            r={RING_R}
            strokeDasharray={RING_CIRCUMFERENCE}
            strokeDashoffset={dashOffset}
          />
        </svg>
        <div className={[styles.ringCenter, isStale ? styles.stale : ''].filter(Boolean).join(' ')}>
          {isStale ? '!' : `${Math.round(progress * 100)}%`}
        </div>
      </div>

      <div className={styles.info}>
        <div className={[styles.ago, isStale ? styles.stale : ''].filter(Boolean).join(' ')}>
          {formatElapsed(elapsedSecs)}
        </div>
        <div className={styles.label}>OSV sync</div>
        <div className={[styles.next, isStale ? styles.stale : ''].filter(Boolean).join(' ')}>
          {isStale ? (
            'overdue'
          ) : (
            <>
              next in <b>{formatNext(intervalSecs - elapsedSecs)}</b>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
