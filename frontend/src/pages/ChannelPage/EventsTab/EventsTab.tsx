import { useState } from 'react';
import { useChannelEvents } from '../../../hooks/notification_channels';
import { BackIcon } from '../../../components/Icons';
import { EventCard } from './EventCard/EventCard';
import styles from './EventsTab.module.css';

const PAGE_SIZE = 10;

type Props = { channelId: string };

export function EventsTab({ channelId }: Props) {
  const { data: events = [] } = useChannelEvents(channelId);
  const [page, setPage] = useState(0);

  if (events.length === 0) {
    return <p className={styles.empty}>No notification events yet.</p>;
  }

  const totalPages = Math.ceil(events.length / PAGE_SIZE);
  const pageEvents = events.slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE);

  const navigation =       (totalPages > 1 && (
        <div className={styles.pagination}>
          <button
            type="button"
            className={styles.pageBtn}
            onClick={() => setPage((p) => p - 1)}
            disabled={page === 0}
            aria-label="Previous page"
          >
            <BackIcon width={13} height={13} />
          </button>

          <span className={styles.pageInfo}>
            {page + 1} <span className={styles.pageInfoSep}>/</span> {totalPages}
          </span>

          <button
            type="button"
            className={`${styles.pageBtn} ${styles.pageBtnNext}`}
            onClick={() => setPage((p) => p + 1)}
            disabled={page === totalPages - 1}
            aria-label="Next page"
          >
            <BackIcon width={13} height={13} />
          </button>
        </div>
      ));
  

  return (
    <div className={styles.container}>
      {navigation}

      <div className={styles.list}>
        {pageEvents.map((event) => (
          <EventCard key={event.id} event={event} />
        ))}
      </div>

      {navigation}
    </div>
  );
}
