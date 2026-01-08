FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/livedraft-arena"]