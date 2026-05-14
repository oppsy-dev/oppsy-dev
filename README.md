<p align="center">
  <img src="logo.png" alt="OPPSY" width="200" />
</p>

# OPPSY

OPPSY is an open-source vulnerability management platform. It watches your project's dependency lock files, continuously checks them against the [OSV database](https://osv.dev), and notifies you whenever a vulnerability is found or updated — without any manual effort.

## Documentation

[https://oppsy-dev.github.io/oppsy-dev/](https://oppsy-dev.github.io/oppsy-dev/)

## Quick start

```bash
docker run --name oppsy -p 3030:3030 -v oppsy-data:/data ghcr.io/oppsy-dev/oppsy:latest
```

The service is available at [http://localhost:3030](http://localhost:3030).

## Configuration

OPPSY is configured entirely through environment variables prefixed with `OPPSY_SERVICE_`. Every option has a sensible default — the snippets below show two of the most commonly tuned settings. See [docs/service-configuration.md](docs/service-configuration.md) for the full reference.

**Restrict OSV ecosystems** — by default OPPSY downloads vulnerability data for every ecosystem published by OSV. Narrow it to just the ones you care about to reduce disk usage and sync time:

```bash
docker run --name oppsy -p 3030:3030 -v oppsy-data:/data \
  -e OPPSY_SERVICE_OSV_ECOSYSTEMS=crates.io,npm,Go,PyPI,Maven \
  ghcr.io/oppsy-dev/oppsy:latest
```

**Change the OSV sync interval** — how often (in minutes) the background task re-downloads OSV data and re-evaluates tracked manifests. Defaults to `15`; values below 15 provide no benefit since OSV publishes updates with up to 15 minutes of latency.

```bash
docker run --name oppsy -p 3030:3030 -v oppsy-data:/data \
  -e OPPSY_SERVICE_OSV_SYNC_INTERVAL=30 \
  ghcr.io/oppsy-dev/oppsy:latest
```

## Community

**Our Discord server** [https://discord.gg/9UFcrp38](https://discord.gg/9UFcrp38)
