# Build stage - Pinned to 'bookworm' to match the runtime
FROM rust:bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage - Pinned to 'bookworm-slim'
FROM debian:bookworm-slim
WORKDIR /app

# Install OpenSSL and certs (required for network requests)
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/job_executor /app/job_executor

CMD ["./job_executor"]