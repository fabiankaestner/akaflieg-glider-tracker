FROM rust:1.67 as builder
WORKDIR /usr/src/scraper
COPY . .
RUN cargo install --path ./akaflieg-ogn-aprs-scraper

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/akaflieg-ogn-aprs-scraper /usr/local/bin/akaflieg-ogn-aprs-scraper
CMD ["akaflieg-ogn-aprs-scraper"]
