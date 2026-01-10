FROM rustlang/rust:nightly-slim as builder

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/livedraft-arena"]
