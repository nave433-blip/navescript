# Dockerfile for Navescript Runtime
FROM rust:1.80-slim AS builder

WORKDIR /usr/src/navescript
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies if any
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/navescript/target/release/navescript /usr/local/bin/navescript

WORKDIR /app

ENTRYPOINT ["navescript"]
CMD ["run", "main.ns"]
