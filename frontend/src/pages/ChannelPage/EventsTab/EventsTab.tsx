import { useChannelEvents } from '../../../hooks/notification_channels';
import { EventCard } from './EventCard/EventCard';
import styles from './EventsTab.module.css';

type Props = { channelId: string };

export function EventsTab({ channelId }: Props) {
  const { data: events = [] } = useChannelEvents(channelId);

  if (events.length === 0) {
    return <p className={styles.empty}>No notification events yet.</p>;
  }

  return (
    <div className={styles.list}>
      {events.map((event) => (
        <EventCard key={event.id} event={event} />
      ))}
    </div>
  );
}
