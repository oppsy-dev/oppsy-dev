import { useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { AppRoute } from '../../routes/Routes';
import { ChannelSettingsSection } from './ChannelSettingsSection/ChannelSettingsSection';
import { ConfigurationTab } from './ConfigurationTab/ConfigurationTab';
import { EventsTab } from './EventsTab/EventsTab';
import styles from './ChannelPage.module.css';
import type { NotificationChannel } from '../../api/notification_channels';
import { useChannels, useDeleteChannel } from '../../hooks/notification_channels';
import { BackIcon, ChannelIcon, CHANNEL_ICON_BG, GearIcon } from '../../components/Icons';
import { formatUuidV7Date, formatUuidV7TimeAgo } from '../../utils/uuidV7';

enum Tab {
  Configuration = 'Configuration',
  Events = 'Events',
}

const TAB_RENDERERS: Record<Tab, (channel: NotificationChannel | undefined) => React.ReactElement> =
  {
    [Tab.Configuration]: (channel) => (channel ? <ConfigurationTab channel={channel} /> : <></>),
    [Tab.Events]: (channel) => <EventsTab channelId={channel?.id ?? ''} />,
  };

export function ChannelPage() {
  const { channelId } = useParams<{ channelId: string }>();
  const navigate = useNavigate();
  const { data: channels = [] } = useChannels();
  const channel = channelId ? channels.find((c) => c.id === channelId) : undefined;
  const deleteChannel = useDeleteChannel();
  const displayName = channel?.name;
  const [tab, setTab] = useState<Tab>(Tab.Configuration);
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className={styles.page}>
      <nav className={styles.breadcrumb}>
        <Link to={AppRoute.ChannelsDashboard} className={styles.breadcrumbLink}>
          <BackIcon width={13} height={13} />
          Channels
        </Link>
        <span className={styles.breadcrumbSep}>/</span>
        <code className={styles.breadcrumbCurrent}>{displayName}</code>
      </nav>

      <div className={styles.header}>
        <div className={styles.headerLeft}>
          <div className={styles.headerInfo}>
            <div
              className={styles.workspaceIcon}
              style={
                channel?.conf.type ? { background: CHANNEL_ICON_BG[channel.conf.type] } : undefined
              }
            >
              {channel?.conf.type && (
                <ChannelIcon type={channel.conf.type} width={22} height={22} />
              )}
            </div>
            <div>
              <h1 className={styles.title}>{displayName}</h1>
              {channelId && <p className={styles.subline}>Created {formatUuidV7Date(channelId)}</p>}
            </div>
          </div>
          <div className={styles.badges}>
            {channel?.conf.type && <span className={styles.typeBadge}>{channel.conf.type}</span>}
            {channel && (
              <span className={channel.active ? styles.activeBadge : styles.inactiveBadge}>
                {channel.active ? 'Active' : 'Inactive'}
              </span>
            )}
          </div>
        </div>
        <button
          type="button"
          className={
            showSettings ? `${styles.settingsBtn} ${styles.settingsBtnActive}` : styles.settingsBtn
          }
          onClick={() => setShowSettings((s) => !s)}
        >
          <GearIcon width={13} height={13} />
          Settings
        </button>
      </div>

      {showSettings && channel ? (
        <ChannelSettingsSection
          channel={channel}
          onBack={() => setShowSettings(false)}
          onDelete={() => (channelId ? deleteChannel.mutateAsync(channelId) : Promise.resolve())}
          onDeleted={() => navigate(AppRoute.ChannelsDashboard)}
        />
      ) : (
        <>
          <div className={styles.statsStrip}>
            <div className={styles.statCard}>
              <div className={styles.statValue}>{channel?.events_count ?? '—'}</div>
              <div className={styles.statLabel}>Events</div>
            </div>
            <div className={styles.statCard}>
              <div className={styles.statValue}>{channel?.workspaces_count ?? '—'}</div>
              <div className={styles.statLabel}>Workspaces</div>
            </div>
            <div className={styles.statCard}>
              <div className={channel?.latest_event_id ? styles.statValue : styles.statValueMuted}>
                {channel?.latest_event_id ? formatUuidV7TimeAgo(channel.latest_event_id) : '—'}
              </div>
              <div className={styles.statLabel}>Latest event</div>
            </div>
          </div>

          <div className={styles.tabBar}>
            <div className={styles.tabs}>
              {(Object.keys(TAB_RENDERERS) as Tab[]).map((label) => (
                <button
                  key={label}
                  type="button"
                  className={tab === label ? styles.tabActive : styles.tab}
                  onClick={() => setTab(label)}
                >
                  {label}
                </button>
              ))}
            </div>
          </div>
          {TAB_RENDERERS[tab](channel)}
        </>
      )}
    </div>
  );
}
