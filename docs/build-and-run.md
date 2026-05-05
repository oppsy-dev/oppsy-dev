# Build and run OPPSY

## Run Docker

```bash
docker run --name oppsy -p 3030:3030 -v oppsy-data:/data ghcr.io/oppsy-dev/oppsy:latest
```

Mount a volume at `/data` so the SQLite database persists across restarts. The service will be available at <http://localhost:3030>.

## Build from source

[Dagger](https://dagger.io) runs the build pipelines inside containers — no local Rust or Node toolchain required beyond having the [Dagger CLI](https://docs.dagger.io/install) installed.

**First-time setup** (generates SDK bindings and project files):

```bash
dagger develop
```

Builds the full image (Rust binary + frontend assets + Atlas migrations) and loads it directly into Docker:

```bash
dagger call oppsy-build --src=. export-image --name oppsy:latest
```

Then run it:

```bash
docker run -p 3030:3030 -v oppsy-data:/data oppsy:latest
```
