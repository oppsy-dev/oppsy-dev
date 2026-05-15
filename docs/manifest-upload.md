# Manifest upload

A manifest is a dependency lock file attached to a workspace. Once uploaded, OPPSY parses it, identifies every package it declares, and runs a vulnerability scan against the OSV database.

## Install oppsy-cli

### From prebuilt binaries (recommended)

Pre-built binaries for Linux, macOS, and Windows are available on the [GitHub Releases](https://github.com/oppsy-dev/oppsy-dev/releases) page.

The quickest way to install is with the generated installer script:

```sh
curl -fsSL https://github.com/oppsy-dev/oppsy-dev/releases/latest/download/install.sh | sh
```

This installs `oppsy-cli` to `~/.local/bin` by default. To choose a different directory:

```sh
curl -fsSL https://github.com/oppsy-dev/oppsy-dev/releases/latest/download/install.sh | sh -s -- -b /usr/local/bin
```

### From source (requires Go 1.24+)

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
