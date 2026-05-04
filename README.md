<p align="center">
  <img src="logo.png" alt="OPPSY" width="200" />
</p>

# OPPSY

OSV (Open Source Vulnerability) management platform. Scans dependency manifests across workspaces, matches packages against the [OSV database](https://osv.dev), and delivers notifications via webhooks.

## Build with Dagger

[Dagger](https://dagger.io) runs the build pipelines inside containers — no local Rust or Node toolchain required beyond having the [Dagger CLI](https://docs.dagger.io/install) installed.

**First-time setup** (generates SDK bindings and project files):

```bash
dagger develop
```

### Build `OPPSY` image

Builds the full image (Rust binary + frontend assets + Atlas migrations) and loads it directly into Docker:

```bash
dagger call oppsy-build --src=. export-image --name oppsy:latest
```

For Podman, point Dagger at the Podman socket first:

```bash
export DOCKER_HOST=unix://$XDG_RUNTIME_DIR/podman/podman.sock
dagger call oppsy-build --src=. export-image --name oppsy:latest
```

Run the image (mount a volume at `/data` so the SQLite database persists):

```bash
# Docker
docker run -p 3030:3030 -v oppsy-data:/data oppsy:latest

# Podman
podman run -p 3030:3030 -v oppsy-data:/data oppsy:latest
```

The service will be available at [http://localhost:3030](http://localhost:3030).

