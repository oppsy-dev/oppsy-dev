<p align="center">
  <img src="logo.png" alt="OPPSY" width="200" />
</p>

# OPPSY

OPPSY is an open-source vulnerability management platform. It watches your project's dependency lock files, continuously checks them against the [OSV database](https://osv.dev), and notifies you whenever a vulnerability is found or updated — without any manual effort.

## Documentation

- [Introduction](docs/introduction.md) — core concepts: workspaces, manifests, and notification channels
- [Supported ecosystems](docs/ecosystems.md) — Cargo, npm, pip/uv, Poetry, Go
- [Manifest upload](docs/manifest-upload.md) — how to upload lock files
- [Build and run](docs/build-and-run.md) — building the image with Dagger and running it
- [API Reference](docs/api-reference.html) — full REST API reference

## Quick start

Build the image with [Dagger](https://dagger.io) (no local Rust or Node toolchain required):

```bash
dagger develop
dagger call oppsy-build --src=. export-image --name oppsy:latest
```

Run it (mount `/data` so the SQLite database persists):

```bash
docker run -p 3030:3030 -v oppsy-data:/data oppsy:latest
```

The service is available at [http://localhost:3030](http://localhost:3030).
