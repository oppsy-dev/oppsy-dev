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

## Community

**Our Discord server** [https://discord.gg/9UFcrp38](https://discord.gg/9UFcrp38)
