FROM rust:1-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runner

# Install CA certificates for TLS/wss connection support to SurrealDB
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/newspulse-backend /usr/local/bin/server

ENV ADDR="0.0.0.0"
ENV PORT="3000"
# SURREAL_URI, SURREAL_USER, SURREAL_PASS should be provided at runtime

EXPOSE 3000

CMD ["server"]