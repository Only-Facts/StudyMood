FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache \
  pkgconfig \
  openssl-dev \
  musl-dev \
  gcc

COPY Cargo.toml /app/

ENV OPENSSL_DIR=/usr

COPY src /app/src

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache \
  libssl3 \
  mariadb-connector-c

COPY --from=builder /app/target/release/backend /usr/local/bin/studymood

EXPOSE 8081

CMD [ "/usr/local/bin/studymood" ]
