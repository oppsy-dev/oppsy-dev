# Service configuration

All configuration is read from environment variables at startup. There are no config files — every option has a sensible default, so the service runs without any environment variables set.

## Naming convention

Every variable is prefixed with `OPPSY_SERVICE_`. The rest of the name is the field name in uppercase. For example, the `bind_address` field is configured via `OPPSY_SERVICE_BIND_ADDRESS`.

---

## Reference

### `OPPSY_SERVICE_BIND_ADDRESS`

**Type:** socket address (`<ip>:<port>`)\
**Default:** `0.0.0.0:3030`

The network address the HTTP server binds to. Accepts any valid socket address.

```
OPPSY_SERVICE_BIND_ADDRESS=127.0.0.1:8080
```

---

### `OPPSY_SERVICE_LOG_FORMAT`

**Type:** enum\
**Default:** `human_readable`\
**Values:** `human_readable`, `json`

Controls how log output is formatted. Use `json` when shipping logs to a structured log aggregator (Datadog, Loki, etc.).

```
OPPSY_SERVICE_LOG_FORMAT=json
```

---

### `OPPSY_SERVICE_LOG_LEVEL`

**Type:** enum\
**Default:** `INFO`\
**Values:** `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`

Minimum severity level for emitted log lines. Values are case-sensitive and must be uppercase.

```
OPPSY_SERVICE_LOG_LEVEL=DEBUG
```

---

### `OPPSY_SERVICE_API_URL_PREFIX`

**Type:** string\
**Default:** `/api`

The path prefix under which all REST API routes are mounted. Change this if you need to namespace the API behind a reverse proxy path.

```
OPPSY_SERVICE_API_URL_PREFIX=/v1
```

---

### `OPPSY_SERVICE_CORE_DB_URL`

**Type:** SQLite URL\
**Default:** `sqlite://oppsy.db`

Connection URL for the SQLite database that stores workspaces, manifests, and notification channel records. The path in the URL is resolved relative to the working directory.

```
OPPSY_SERVICE_CORE_DB_URL=sqlite:///data/oppsy.db
```

> When running in Docker, mount a volume at `/data` and point this (and the path settings below) at it so data persists across container restarts.

---

### `OPPSY_SERVICE_MANIFEST_DB_PATH`

**Type:** filesystem path\
**Default:** `./manifest_db`

Directory where the raw content of uploaded manifest files (lock files) is stored. The directory is created if it does not exist.

```
OPPSY_SERVICE_MANIFEST_DB_PATH=/data/manifest_db
```

---

### `OPPSY_SERVICE_OSV_DB_PATH`

**Type:** filesystem path\
**Default:** `./osv_db`

Directory where downloaded OSV vulnerability archives are cached on disk. At startup and on each sync cycle, OPPSY fetches updated archives from the [OSV GCS bucket](https://google.github.io/osv.dev/data/) into this directory.

```
OPPSY_SERVICE_OSV_DB_PATH=/data/osv_db
```

---

### `OPPSY_SERVICE_OSV_SYNC_INTERVAL`

**Type:** integer (minutes)\
**Default:** `15`

How often the background task re-downloads OSV vulnerability data and re-evaluates all tracked manifests. The OSV team publishes updates with at most 15 minutes of latency, so values below 15 provide no benefit.

```
OPPSY_SERVICE_OSV_SYNC_INTERVAL=30
```

---

### `OPPSY_SERVICE_OSV_ECOSYSTEMS`

**Type:** comma-separated list of ecosystem names\
**Default:** *(empty — all ecosystems)*

Restricts which OSV ecosystem archives are downloaded and indexed. When left empty (the default), OPPSY downloads data for every ecosystem published in the OSV GCS bucket. Setting an explicit list reduces disk usage and sync time at the cost of narrower vulnerability coverage.

Values must exactly match the canonical OSV ecosystem names (case-sensitive):

| Name | Description |
|---|---|
| `AlmaLinux` | AlmaLinux OS |
| `Alpine` | Alpine Linux |
| `Android` | Android |
| `Bitnami` | Bitnami application catalog |
| `CRAN` | R packages |
| `Chainguard` | Chainguard container images |
| `Debian` | Debian Linux |
| `GHC` | Haskell (GHC) |
| `GIT` | Generic Git repository vulnerabilities ([including C/C++](https://osv.dev/blog/posts/introducing-broad-c-c++-support/)) |
| `Go` | Go modules |
| `Hackage` | Haskell packages (Hackage) |
| `Hex` | Elixir/Erlang packages |
| `Linux` | Linux kernel |
| `Maven` | Java (Maven Central) |
| `NuGet` | .NET packages |
| `OSS-Fuzz` | OSS-Fuzz findings |
| `Packagist` | PHP (Packagist/Composer) |
| `PyPI` | Python packages |
| `Red Hat` | Red Hat Linux |
| `Rocky Linux` | Rocky Linux |
| `RubyGems` | Ruby gems |
| `SUSE` | SUSE Linux |
| `SwiftURL` | Swift packages |
| `Ubuntu` | Ubuntu Linux |
| `VSCode` | VS Code extensions |
| `Wolfi` | Wolfi (Chainguard) |
| `crates.io` | Rust packages |
| `npm` | JavaScript/Node packages |
| `opam` | OCaml packages |
| `openEuler` | openEuler Linux |
| `openSUSE` | openSUSE Linux |

```
OPPSY_SERVICE_OSV_ECOSYSTEMS=crates.io,npm,Go,PyPI,Maven
```

---

### `OPPSY_SERVICE_FRONTEND_PATH`

**Type:** filesystem path\
**Default:** `./frontend`

Directory containing the pre-built React SPA assets (`index.html`, JS bundles, etc.). The backend serves these files at `/` and falls back to `index.html` for any path not matched by an API route, enabling client-side routing.

In the official Docker image this is baked in at build time and does not need to be changed.

```
OPPSY_SERVICE_FRONTEND_PATH=/app/frontend
```

---

### `OPPSY_SERVICE_SMTP_URL`

**Type:** URL (`smtp://...`)\
**Default:** *(unset — email notifications disabled)*

SMTP connection URL used to send email vulnerability notifications. When this variable is not set, the email notification backend is not initialised and email channels cannot be used.

Format: `smtp://username:password@host:port`

```
OPPSY_SERVICE_SMTP_URL=smtp://alerts:secret@mail.example.com:587
```

---
