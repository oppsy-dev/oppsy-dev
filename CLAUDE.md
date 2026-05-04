# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

OPPSY is a full-stack OSV (Open Source Vulnerability) management platform. It scans dependency manifests across workspaces, matches packages against the OSV database, and delivers notifications via webhooks. The repo is a monorepo with a Rust backend and a React/TypeScript frontend. In production the backend serves the compiled frontend assets directly — there is no separate frontend server.

## Commands

All top-level tasks are driven by `just` (see `Justfile`). Run `just` to list all recipes.

**Backend:**
```bash
just backend-lint-check          # fmt check + clippy (-D warnings) + cargo-deny
just backend-unit-tests          # cargo test --locked
cargo +nightly fmt --manifest-path backend/Cargo.toml --all   # format in place
```

Run a single test:
```bash
cargo test --manifest-path backend/Cargo.toml -p <crate> <test_name>
```

**Frontend:**
```bash
just frontend-gen-api-client     # regenerate src/api/schema.d.ts from backend OpenAPI
just run-frontend-dev            # gen client → format → yarn start (localhost:3000)
just frontend-lint-check         # prettier check + yarn build
```

**Full pre-commit check:**
```bash
just dev                         # lint-check + unit tests (backend + frontend)
```

**Database migrations** (run from `backend/core-db/`):
```bash
atlas migrate apply --url sqlite://oppsy.db --env sqlite   # apply pending
atlas migrate hash  --env sqlite                           # rehash after editing a file
atlas migrate status --url sqlite://oppsy.db --env sqlite  # check status
```

## Architecture

### Backend — Rust workspace (`backend/Cargo.toml`)

Six crates, each with a single responsibility:

| Crate | Role |
|---|---|
| `service` | Binary entry point. Poem HTTP server on `:3030`, poem-openapi REST endpoints, static frontend serving, tracing. |
| `core-db` | `CoreDb` connection-pool wrapper over SQLite (SQLx). Owns all queries and Atlas migrations under `sqlite-migrations/`. |
| `package-analyzer` | Parses lock files (`Cargo.lock`, `package-lock.json`, `uv.lock`, `poetry.lock`, Go JSON) and matches packages against OSV records via `MultiAnalyzer`. |
| `manifest-storage` | Filesystem read/write for raw manifest bytes. |
| `notifier` | Webhook delivery with HMAC signing; async trait-based. |
| `common` | `ConvertTo` trait and shared error helpers. |

**Resource registry pattern:** `Settings`, `CoreDb`, `OsvDb`, and `Notifier` are singletons initialized once at startup via their `Resource::init()` implementations and accessed via `Resource::get()`.

**Configuration** — all via env vars prefixed `OSV_SERVICE_`:

| Variable | Default |
|---|---|
| `OSV_SERVICE_BIND_ADDRESS` | `0.0.0.0:3030` |
| `OSV_SERVICE_CORE_DB_URL` | `sqlite://oppsy.db` |
| `OSV_SERVICE_MANIFEST_DB_PATH` | `./manifest_db` |
| `OSV_SERVICE_OSV_DB_PATH` | `./osv_db` |
| `OSV_SERVICE_OSV_SYNC_INTERVAL` | `15` (minutes) |
| `OSV_SERVICE_FRONTEND_PATH` | `./frontend` |

**Rust toolchain:** nightly (for clippy + rustfmt). Linting enforces `-D warnings`; no `unwrap`/`expect`/`panic` in library code.

### Frontend — React/TypeScript (`frontend/`)

- **React 19** + React Router 7 + Zustand (client state) + TanStack React Query (server state)
- **Tailwind CSS 3** for styling
- **API client** (`src/api/schema.d.ts`) is auto-generated from the backend's OpenAPI schema — always regenerate after backend API changes with `just frontend-gen-api-client`
- Pages live under `src/pages/`, shared components under `src/components/`, Zustand stores under `src/stores/`
- Package manager: **yarn**
- The frontend is a SPA. In production, run `yarn build` and point `OSV_SERVICE_FRONTEND_PATH` at the resulting `build/` directory. The backend serves all static assets and falls back to `index.html` for client-side routes. No separate frontend process is needed.

### Building the service binary (Dagger)

The service binary is built via a [Dagger](https://dagger.io) pipeline defined in `dagger/src/main.py`. Dagger runs the build inside a container and exports just the compiled binary — no Docker knowledge required beyond having the Dagger CLI installed.

**Project structure:**
```
dagger.json          # module root — kept at repo root so dagger commands run from here
dagger/
  src/
    main.py          # pipeline code (Python SDK)
  sdk/               # generated SDK bindings (after dagger develop)
  pyproject.toml     # generated Python project file (after dagger develop)
```

`dagger.json` has `"source": "dagger"` which tells the CLI that the module source lives in the `dagger/` subdirectory.

**`@object_type` and `@function`:** Dagger modules expose functionality as a typed API. `@object_type` marks a class as a Dagger type — it becomes callable via `dagger call <type>`. `@function` marks a method as an exposed function within that type. The class name (`Oppsy`) and method name (`service_build`) map directly to CLI subcommands: `dagger call service-build ...`.

**First-time setup** (generates `pyproject.toml`, `uv.lock`, and SDK bindings):
```bash
dagger develop
```

**Build and export the binary:**
```bash
dagger call service-build --src=. export --path=./service
```

### CI

| Workflow | Trigger | What it does |
|---|---|---|
| `backend_ci.yml` | Push/PR on backend changes | `backend-lint-check` + `backend-unit-tests` |
| `frontend_ci.yml` | Push/PR on frontend changes | `frontend-lint-check` |
| `conventional_commits.yml` | All PRs | Validates commit message format |
| `integration_tests_nightly.yml` | Nightly | Extended integration tests |
