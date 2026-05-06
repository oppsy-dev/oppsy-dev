import { useEffect, useRef, useState } from 'react';
import { useAllChannelEvents } from '../../hooks/notification_channels';
import type { NotificationChannelType } from '../../api/notification_channels';
import { formatUuidV7TimeAgo } from '../../utils/uuidV7';
import { EventFeedCard } from './EventFeedCard/EventFeedCard';
import type { EventFeedEvent } from './EventFeedCard/EventFeedCard';
import styles from './EventsFeed.module.css';

const PAGE_SIZE = 8;

export function EventsFeed() {
  const { events: rawEvents, isLoading } = useAllChannelEvents();
  const [visibleCount, setVisibleCount] = useState(PAGE_SIZE);
  const sentinelRef = useRef<HTMLDivElement>(null);

  const events: EventFeedEvent[] = rawEvents.map((e) => ({
    id: e.id,
    channelId: e.channel_id,
    channelName: e.channel.name,
    channelType: e.channel.conf.type as NotificationChannelType,
    workspaceName: e.meta.workspace_name,
    manifestName: e.meta.manifest_name,
    manifestTag: e.meta.manifest_tag,
    timeAgo: formatUuidV7TimeAgo(e.id),
    delivered: !e.error,
  }));

  useEffect(() => {
    const el = sentinelRef.current;
    if (!el) return;

    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setVisibleCount((n) => Math.min(n + PAGE_SIZE, events.length));
        }
      },
      { threshold: 0.1 },
    );

    observer.observe(el);
    return () => observer.disconnect();
  }, [events.length]);

  const visible = events.slice(0, visibleCount);

  return (
    <div className={styles.feed}>
{isLoading ? (
        <p className={styles.empty}>Loading…</p>
      ) : events.length === 0 ? (
        <p className={styles.empty}>No events yet.</p>
      ) : (
        <div className={styles.list}>
          {visible.map((event) => (
            <EventFeedCard key={event.id} event={event} />
          ))}
          <div ref={sentinelRef} className={styles.sentinel} />
        </div>
      )}
    </div>
  );
}
