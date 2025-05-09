FROM rust:1.86 as builder
WORKDIR /usr/src/scraper
COPY . .
ARG SQLX_OFFLINE=true
RUN cargo install --path ./akaflieg-ogn-aprs-scraper

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/akaflieg-ogn-aprs-scraper /usr/local/bin/akaflieg-ogn-aprs-scraper
CMD ["akaflieg-ogn-aprs-scraper"]
