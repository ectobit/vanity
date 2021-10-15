# syntax=docker/dockerfile:1.3
FROM rust:1.55.0 AS builder

WORKDIR /app

RUN cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target cargo build --release && cargo strip

FROM debian:11.1-slim

LABEL org.opencontainers.image.vendor="ectobit.com"

COPY --from=builder /app/target/release/vanity /

CMD ["./vanity"]

EXPOSE 3000
