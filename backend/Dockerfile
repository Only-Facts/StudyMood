FROM rust:slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && \
  apt-get install -y \
  pkg-config \
  libssl-dev \
  build-essential \
  mariadb-client && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock /app/

RUN mkdir src && echo "fn main() {}" > src/main.rs && \
  cargo build --release --locked && \
  rm -rf src

COPY src /app/src

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update && \
  apt-get install -y libssl-dev \
  mariadb-client && \
  apt-get clean

COPY --from=builder /app/target/release/backend /app/studymood
RUN chmod +x /app/studymood

EXPOSE 8081

CMD [ "/app/studymood" ]
