-- notification_channels: one row per configured channel instance.
-- type discriminates the channel kind: 'email', 'webhook', 'discord', 'slack', 'telegram', etc.
-- configuration holds a JSON object whose schema is determined by type.
CREATE TABLE notification_channels (
  id            blob    NOT NULL PRIMARY KEY, -- TODO: use uuid type when migrating to PostgreSQL
  name          text    NOT NULL,
  conf          text    NOT NULL, -- JSON-encoded; schema varies by type
  active        integer NOT NULL DEFAULT 1 -- boolean: 1 = active, 0 = inactive
);

-- workspace_notification_channels: a workspace can have many channels,
-- including multiple channels of the same type (e.g. two webhooks).
CREATE TABLE workspace_notification_channels (
  workspace_id             blob NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
  channel_id  blob NOT NULL REFERENCES notification_channels (id) ON DELETE CASCADE,
  PRIMARY KEY (workspace_id, channel_id)
);

CREATE TABLE notification_events (
  id              blob    NOT NULL PRIMARY KEY, -- TODO: use uuid type when migrating to PostgreSQL
  channel_id      blob    NOT NULL REFERENCES notification_channels (id) ON DELETE CASCADE,
  error           text,  -- NULL means successfully delivered

  meta            text    NOT NULL -- JSON-encoded;
);
