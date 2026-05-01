<p align="center">
  <img src="logo.png" alt="OPPSY" width="200" />
</p>

# OPPSY

OSV (Open Source Vulnerability) management platform. Scans dependency manifests across workspaces, matches packages against the [OSV database](https://osv.dev), and delivers notifications via webhooks.

## Run with Docker

```bash
docker compose up --build
```

The service will be available at [http://localhost:8080](http://localhost:8080).

To reset all data and rebuild from scratch:

```bash
docker compose down -v
docker compose up --build
```
