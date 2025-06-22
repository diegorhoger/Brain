# Brain AI System - Production Dockerfile
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src/ ./src/
COPY examples/ ./examples/

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app brain

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/brain /usr/local/bin/brain
COPY --from=builder /app/target/release/examples/* /usr/local/bin/

# Copy configuration and scripts
COPY scripts/ ./scripts/
COPY tasks/ ./tasks/

# Create data directories
RUN mkdir -p /app/data /app/logs /app/backups && \
    chown -R brain:brain /app

# Switch to app user
USER brain

# Set environment variables
ENV RUST_LOG=info
ENV BRAIN_DATA_DIR=/app/data
ENV BRAIN_LOG_DIR=/app/logs
ENV BRAIN_BACKUP_DIR=/app/backups

# Expose port for API (if applicable)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD brain --health-check || exit 1

# Default command
CMD ["brain", "--config", "/app/scripts/config.toml"] 