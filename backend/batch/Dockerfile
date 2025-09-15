# Multi-stage build for production deployment
FROM rust:1.87-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY atcoder_client ./atcoder_client/
COPY ddb_client ./ddb_client/
COPY api ./api/
COPY batch ./batch/

WORKDIR /app/batch
RUN cargo build --release

# Minimal runtime image
FROM debian:bookworm-slim AS production

# Install runtime dependencies for HTTPS
RUN apt update && \
    apt install -y ca-certificates libssl3 && \
    apt clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/crawl_new_contests ./
COPY --from=builder /app/target/release/crawl_new_submissions ./

RUN groupadd -r appuser && useradd -r -g appuser appuser && \
    chown -R appuser:appuser /app
USER appuser

CMD ["sh", "-c", "./crawl_new_contests && ./crawl_new_submissions"]
