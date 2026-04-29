FROM debian:trixie-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && update-ca-certificates \
    && curl -sSf https://atlasgo.sh | sh \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /core-db
COPY atlas.hcl .
COPY sqlite-migrations/ ./sqlite-migrations/

# Applies all pending migrations to the SQLite file at the mounted /data volume.
CMD ["atlas", "migrate", "apply", "--url", "sqlite:///data/oppsy.db", "--env", "sqlite"]
