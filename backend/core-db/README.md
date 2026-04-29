# core-db

Library crate for OPPSY's core domain entities: users, teams, and workspaces, backed by a SQL database.

## Responsibilities

- Exposes `CoreDb` — a connection-pool wrapper around a SQL database.
- Implements all queries.
- Owns the database schema as versioned Atlas migrations under `sqlite-migrations/`  (currently targeting SQLite).

## Schema migrations

Schema is managed with [Atlas](https://atlasgo.io). All commands are run from `backend/core-db/`.

```bash
cd backend/core-db
```

**Rehash after editing a migration file:**

```bash
atlas migrate hash --env sqlite
```

**Apply all pending migrations:**

```bash
atlas migrate apply --url sqlite://oppsy.db --env sqlite
```

**Check migration status:**

```bash
atlas migrate status --url sqlite://oppsy.db --env sqlite
```
