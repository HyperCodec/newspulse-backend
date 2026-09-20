FROM rust:1-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm AS runner

# Install CA certificates for TLS/wss connection support to SurrealDB
RUN apt-get update && apt-get install -y ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/steelhacks-xiii /usr/local/bin/server

ENV ADDR="0.0.0.0:3000"
# SURREAL_URI, SURREAL_USER, SURREAL_PASS should be provided at runtime

EXPOSE 3000

CMD ["server"]