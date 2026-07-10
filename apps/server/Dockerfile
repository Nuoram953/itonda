FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY apps/server ./apps/server
COPY apps/cli ./apps/cli
COPY crates ./crates

RUN cargo build --release -p itonda-server


FROM debian:bookworm-slim

COPY --from=builder /app/target/release/itonda-server /usr/local/bin/itonda-server

CMD ["itonda-server"]
