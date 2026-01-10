FROM rustlang/rust:nightly-slim AS builder

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