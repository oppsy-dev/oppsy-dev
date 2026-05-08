-- Workspaces: one row per workspace.
CREATE TABLE workspaces (
  id   blob NOT NULL PRIMARY KEY, -- TODO: use uuid type when migrating to PostgreSQL
  name text NOT NULL
);

CREATE TABLE manifests (
  id            blob NOT NULL PRIMARY KEY, -- TODO: use uuid type when migrating to PostgreSQL
  name          text NOT NULL,
  tag           text,
  meta          text    NOT NULL -- JSON-encoded;
);

-- Manifests associated with a workspace (Workspace.manifests: Set<ManifestId>).
CREATE TABLE workspace_manifests (
  workspace_id  blob NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
  manifest_id   blob NOT NULL REFERENCES manifests (id),
  PRIMARY KEY (workspace_id, manifest_id)
);
