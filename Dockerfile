FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies, clang, and mold linker
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    mold \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Instruct Rust/GCC to use mold as the linker
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

RUN cargo build --release

FROM debian:bookworm-slim AS runner

# Install CA certificates for TLS/wss connection support to SurrealDB
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/newspulse-backend /usr/local/bin/server

ENV ADDR="0.0.0.0"
ENV PORT="3000"

EXPOSE 3000

CMD ["server"]