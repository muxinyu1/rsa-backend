FROM rust AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM ubuntu:22.04

COPY --from=builder /app/target/release/rsa-rust-cpp /usr/local/bin/rsa-rust-cpp

CMD [ "rsa-rust-cpp" ]