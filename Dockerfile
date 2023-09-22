FROM rust:1.70

WORKDIR /usr/src/rinha

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/rinha"]
