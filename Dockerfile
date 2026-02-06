# =============================================================================
# Stage 1: Build Rust Backend
# =============================================================================
FROM rust:1.77-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust workspace
COPY Cargo.toml Cargo.lock ./
COPY execution/ ./execution/

# Build release binary
WORKDIR /app/execution
RUN cargo build --release

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=builder /app/execution/target/release/execution /app/bacon-algo

# Copy .env.example as fallback
COPY .env.example /app/.env.example

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["/app/bacon-algo"]
