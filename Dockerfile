# Dockerfile

# --- STAGE 1: Build Image ---
FROM rust:1.75.0 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/webhook_listener .

EXPOSE 8080


CMD ["/usr/local/bin/webhook_listener"]