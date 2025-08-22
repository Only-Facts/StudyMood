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

FROM rust:slim-bookworm

RUN apt-get update && \
  apt-get install -y libssl-dev \
  mariadb-client && \
  apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/backend ./studymood
RUN chmod +x ./studymood

EXPOSE 8081

CMD [ "./studymood" ]
