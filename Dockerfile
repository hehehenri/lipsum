FROM rust:1.70

WORKDIR /var/rinha/

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/lipsum"]
