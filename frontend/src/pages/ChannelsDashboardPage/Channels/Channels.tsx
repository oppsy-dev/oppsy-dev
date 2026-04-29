import { useState } from 'react';
import { useNavigate } from 'react-router';
import { useChannels, useCreateChannel } from '../../../hooks/notification_channels';
import type { CreateChannelRequest } from '../../../api/notification_channels';
import { CreateChannelModal } from './CreateChannelModal/CreateChannelModal';
import { ChannelCard } from './ChannelCard/ChannelCard';
import { NotificationIcon } from '../../../components/Icons';
import { AppRoute } from '../../../routes/Routes';
import styles from './Channels.module.css';

const PAGE_SIZE = 9;

export function Channels() {
  const { data: channels = [], isLoading } = useChannels();
  const createChannel = useCreateChannel();
  const [modalOpen, setModalOpen] = useState(false);
  const navigate = useNavigate();
  const [page, setPage] = useState(0);

  const handleConfirm = async (body: CreateChannelRequest) => {
    const id = await createChannel.mutateAsync(body);
    navigate(AppRoute.Channel.replace(':channelId', id));
  };

  const totalPages = Math.ceil(channels.length / PAGE_SIZE);
  const pageChannels = channels.slice(page * PAGE_SIZE, (page + 1) * PAGE_SIZE);
  const pagesNavigation = totalPages > 1 && (
    <div className={styles.pagination}>
      <button
        type="button"
        className={styles.pageBtn}
        onClick={() => setPage((p) => p - 1)}
        disabled={page === 0}
      >
        ← Previous
      </button>

      <div className={styles.pageDots}>
        {Array.from({ length: totalPages }, (_, i) => (
          <button
            key={i}
            type="button"
            className={i === page ? `${styles.pageDot} ${styles.pageDotActive}` : styles.pageDot}
            onClick={() => setPage(i)}
            aria-label={`Page ${i + 1}`}
          />
        ))}
      </div>

      <button
        type="button"
        className={styles.pageBtn}
        onClick={() => setPage((p) => p + 1)}
        disabled={page === totalPages - 1}
      >
        Next →
      </button>
    </div>
  );

  return (
    <section className={styles.section}>
      <div className={styles.sectionHeader}>
        <div>
          <h2 className={styles.sectionTitle}>Your Notification Channels</h2>
          {channels.length > 0 && (
            <p className={styles.count}>
              {channels.length} channel{channels.length !== 1 ? 's' : ''}
            </p>
          )}
        </div>
        <button className={styles.addButton} type="button" onClick={() => setModalOpen(true)}>
          <span className={styles.addIcon}>+</span>
          New channel
        </button>
      </div>

      {isLoading && (
        <div className={styles.skeletonGrid}>
          {[1, 2, 3].map((n) => (
            <div key={n} className={styles.skeleton} />
          ))}
        </div>
      )}

      {!isLoading && channels.length === 0 && (
        <div className={styles.emptyState}>
          <div className={styles.emptyIconWrap}>
            <NotificationIcon width={28} height={28} />
          </div>
          <p className={styles.emptyTitle}>No channels yet</p>
          <p className={styles.emptyDesc}>
            Add a notification channel to receive alerts when new vulnerabilities are found in your
            workspaces.
          </p>
          <button className={styles.emptyAction} type="button" onClick={() => setModalOpen(true)}>
            Add your first channel
          </button>
        </div>
      )}

      {pageChannels.length > 0 && (
        <div className={styles.grid}>
          {pageChannels.map((c) => (
            <ChannelCard key={c.id} channel={c} />
          ))}
        </div>
      )}
      {pagesNavigation}

      {modalOpen && (
        <CreateChannelModal onClose={() => setModalOpen(false)} onSuccess={handleConfirm} />
      )}
    </section>
  );
}
