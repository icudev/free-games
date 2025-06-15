FROM ubuntu:latest

RUN apt-get update && apt-get install -y curl build-essential pkg-config libssl-dev ca-certificates
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rm -rf /var/lib/apt/lists/*;

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /
COPY utils /utils
COPY scraper /app
WORKDIR /app

RUN cargo build --release
CMD ["./target/release/game_scraper"]
