# Manifest upload

A manifest is a dependency lock file attached to a workspace. Once uploaded, OPPSY parses it, identifies every package it declares, and runs a vulnerability scan against the OSV database.

## Install oppsy-cli

Pre-built binaries for Linux, macOS, and Windows are available on the [GitHub Releases](https://github.com/oppsy-dev/oppsy-dev/releases) page.

**From source (requires Go 1.24+):**

```sh
go install github.com/oppsy-dev/oppsy-dev/oppsy-cli@latest
```

## Publish a manifest

```sh
oppsy-cli publish \
  --host-url     http://localhost:3030/api \
  --workspace-id <workspace-id> \
  --lockfile     ./Cargo.lock \
  --name         my-service \
  --tag          v0.1.0
```
