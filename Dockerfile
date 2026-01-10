FROM rustlang/rust:nightly-slim AS builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN rustup default nightly

COPY . .

WORKDIR /app/service

RUN cargo +nightly build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/livedraft-arena-service /app/livedraft-arena

EXPOSE 8080

CMD ["./livedraft-arena"]