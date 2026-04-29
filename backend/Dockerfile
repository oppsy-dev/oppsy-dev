FROM rust:1.91-slim AS builder
WORKDIR /app
COPY . .
ARG IS_LOCAL_DEV=false
RUN if [ "$IS_LOCAL_DEV" = "true" ]; then \
      cargo build --release -p service --features local-dev; \
    else \
      cargo build --release -p service; \
    fi

FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates curl && update-ca-certificates
COPY --from=builder /app/target/release/service /usr/local/bin/service
CMD ["service"]
