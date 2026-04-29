FROM rust:1.91-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p service

FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates curl && update-ca-certificates
COPY --from=builder /app/target/release/service /usr/local/bin/service
CMD ["service"]
