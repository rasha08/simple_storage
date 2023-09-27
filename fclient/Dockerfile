FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

EXPOSE 3001
CMD ["/usr/src/app/target/release/fclient" ]
