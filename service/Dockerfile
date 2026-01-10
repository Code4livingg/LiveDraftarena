# LiveDraft Arena Service - Production Dockerfile
# Multi-stage build for optimized production image

# Build stage
FROM rust:1.79-slim as builder

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_HTTP_TIMEOUT=600

# Install system dependencies needed for Linera SDK compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the entire project (contracts + service)
COPY . .

# Build the service in release mode
WORKDIR /app/service
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/livedraft-arena-service /app/livedraft-arena-service

# Make binary executable
RUN chmod +x /app/livedraft-arena-service

# Expose port 8080
EXPOSE 8080

# Set environment variables for production
ENV RUST_LOG=info
ENV BIND_ADDRESS=0.0.0.0
ENV PORT=8080
ENV CORS_ORIGINS=*

# Run the service
CMD ["./livedraft-arena-service"]
