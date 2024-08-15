FROM rust:1.80.1

WORKDIR /api
COPY . .

RUN cargo build --release

CMD ["./target/release/Example-API"]