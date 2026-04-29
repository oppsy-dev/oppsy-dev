import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useChannels } from '../../../hooks/notification_channels';
import type { NotificationChannel } from '../../../api/notification_channels';
import {
  useWorkspaceChannels,
  useAddWorkspaceChannel,
  useRemoveWorkspaceChannel,
} from '../../../hooks/workspaces';
import { AppRoute } from '../../../routes/Routes';
import { ChannelCard } from './ChannelCard/ChannelCard';
import { PickChannelModal } from './PickChannelModal/PickChannelModal';
import styles from './NotificationsSection.module.css';

type NotificationsSectionProps = {
  workspaceId: string;
};

export function NotificationsSection({ workspaceId }: NotificationsSectionProps) {
  const navigate = useNavigate();
  const [showPicker, setShowPicker] = useState(false);

  const { data: linkedChannels = [], isLoading, isError } = useWorkspaceChannels(workspaceId);
  const { data: allChannels = [] } = useChannels();

  const addMutation = useAddWorkspaceChannel(workspaceId);
  const removeMutation = useRemoveWorkspaceChannel(workspaceId);

  const linkedIds = new Set(linkedChannels.map((ch) => ch.id));
  const availableChannels = allChannels.filter((ch) => !linkedIds.has(ch.id));

  const handleAddClick = () => {
    if (availableChannels.length === 0) {
      navigate(AppRoute.ChannelsDashboard);
    } else {
      setShowPicker(true);
    }
  };

  const handlePick = async (channel: NotificationChannel) => {
    await addMutation.mutateAsync(channel.id);
    setShowPicker(false);
  };

  return (
    <div>
      <div className={styles.header}>
        <div>
          <h3 className={styles.title}>Notifications</h3>
          <p className={styles.desc}>Configure how you receive vulnerability alerts.</p>
        </div>
      </div>

      {isLoading && linkedChannels.length === 0 && <p className={styles.desc}>Loading channels…</p>}

      {isError && <p className={styles.desc}>Failed to load notification channels.</p>}

      {linkedChannels.length > 0 && (
        <div className={styles.channelList}>
          {linkedChannels.map((ch) => (
            <ChannelCard key={ch.id} channel={ch} onRemove={() => removeMutation.mutate(ch.id)} />
          ))}
        </div>
      )}

      <button type="button" className={styles.addChannelBtn} onClick={handleAddClick}>
        + Add channel
      </button>

      {showPicker && (
        <PickChannelModal
          channels={availableChannels}
          onPick={handlePick}
          onClose={() => setShowPicker(false)}
        />
      )}
    </div>
  );
}
