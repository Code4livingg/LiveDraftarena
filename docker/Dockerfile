# Use NIGHTLY Rust with Cargo that supports edition2024
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app

# Ensure nightly is actually used
RUN rustup default nightly

COPY . .

WORKDIR /app/service

RUN cargo +nightly build --release

# ---------------- Runtime stage ----------------
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/service/target/release/livedraft-arena /app/livedraft-arena

EXPOSE 8080

CMD ["/app/livedraft-arena"]

