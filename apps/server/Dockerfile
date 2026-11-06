FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY apps/server ./apps/server
COPY apps/cli ./apps/cli
COPY apps/agent ./apps/agent
COPY crates ./crates

RUN cargo build --release -p itonda-server


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ffmpeg \
    curl \
    ca-certificates \
    python3 && \
    curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp && \
    chmod a+rx /usr/local/bin/yt-dlp && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/itonda-server /usr/local/bin/itonda-server

CMD ["itonda-server"]
