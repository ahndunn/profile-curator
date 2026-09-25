# syntax=docker/dockerfile:1
FROM rust:1.98-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
# Pre-fetch and cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs && cargo build --release || true
COPY src ./src
RUN touch src/main.rs src/lib.rs && cargo build --release

FROM debian:bookworm-slim AS runtime

RUN useradd -m -u 1000 appuser
USER appuser
WORKDIR /home/appuser

COPY --from=builder /app/target/release/profile-curator-mcp /usr/local/bin/profile-curator-mcp

ENTRYPOINT ["profile-curator-mcp"]
