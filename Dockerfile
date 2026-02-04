# ============================================
# Stage 1: Build Rust Backend
# ============================================
FROM rust:1.75-alpine AS rust-builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev

# Copy Rust workspace
COPY Cargo.toml Cargo.lock ./
COPY execution ./execution

# Build release binary
WORKDIR /app/execution
RUN cargo build --release

# ============================================
# Stage 2: Build Svelte Frontend
# ============================================
FROM node:20-alpine AS frontend-builder

WORKDIR /app

# Install pnpm
RUN npm install -g pnpm

# Copy frontend code
COPY station/package.json station/pnpm-lock.yaml ./station/
WORKDIR /app/station
RUN pnpm install --frozen-lockfile

# Copy rest of frontend
COPY station ./
RUN pnpm build

# ============================================
# Stage 3: Production Image
# ============================================
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates nodejs npm

# Install Node.js production server
RUN npm install -g http-server

# Copy backend binary
COPY --from=rust-builder /app/execution/target/release/execution /app/backend

# Copy frontend build
COPY --from=frontend-builder /app/station/build /app/frontend

# Create startup script
RUN echo '#!/bin/sh' > /app/start.sh && \
    echo '/app/backend &' >> /app/start.sh && \
    echo 'cd /app/frontend && http-server -p 3000' >> /app/start.sh && \
    chmod +x /app/start.sh

# Expose ports
EXPOSE 3000 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["/app/start.sh"]
