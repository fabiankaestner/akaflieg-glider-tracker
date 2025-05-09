FROM rust:1.86 as builder
WORKDIR /usr/src/scraper
COPY . .
RUN cargo install --path ./web-api

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/web-api /usr/local/bin/web-api
CMD ["web-api"]
