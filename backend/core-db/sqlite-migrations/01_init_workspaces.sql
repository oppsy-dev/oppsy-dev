-- Workspaces: one row per workspace.
CREATE TABLE workspaces (
  id   blob NOT NULL PRIMARY KEY, -- TODO: use uuid type when migrating to PostgreSQL
  name text NOT NULL
);

CREATE TABLE manifests (
  id            blob NOT NULL PRIMARY KEY, -- TODO: use uuid type when migrating to PostgreSQL
  manifest_type text NOT NULL,
  name          text NOT NULL,
  tag           text
);

-- Manifests associated with a workspace (Workspace.manifests: Set<ManifestId>).
CREATE TABLE workspace_manifests (
  workspace_id  blob NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
  manifest_id   blob NOT NULL REFERENCES manifests (id),
  PRIMARY KEY (workspace_id, manifest_id)
);

-- OSV vulnerabilities discovered for a manifest during a scan.
-- osv_id is the OSV record identifier (e.g. "GHSA-xxxx-xxxx-xxxx", "CVE-xxxx-xxxx").
-- OSV records are not stored locally — only the ID is persisted here.
CREATE TABLE manifest_osv_vuln (
  manifest_id  blob NOT NULL REFERENCES manifests (id) ON DELETE CASCADE,
  osv_id       text NOT NULL,
  detected_at  integer NOT NULL, -- TODO Unix epoch seconds (UTC); use TIMESTAMPTZ when migrating to PostgreSQL
  PRIMARY KEY (manifest_id, osv_id)
);