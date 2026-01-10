# Build stage
FROM rustlang/rust:nightly-slim AS builder

# Ensure nightly is active
RUN rustup default nightly

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Build the service in release mode using nightly
WORKDIR /app/service
RUN cargo +nightly build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/livedraft-arena-service /app/livedraft-arena

# Make binary executable
RUN chmod +x /app/livedraft-arena

# Expose port 8080
EXPOSE 8080

# Set environment variables for production
ENV RUST_LOG=info
ENV BIND_ADDRESS=0.0.0.0
ENV PORT=8080
ENV CORS_ORIGINS=*

# Run the service
CMD ["./livedraft-arena"]

