# syntax=docker/dockerfile:1.3
FROM rust:1.55.0 AS builder

WORKDIR /app

RUN cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target cargo build --release && \
    cargo strip && \
    mv /app/target/release/vanity /app

FROM gcr.io/distroless/cc-debian11

LABEL org.opencontainers.image.vendor="ectobit.com"

COPY --from=builder /app/vanity /

CMD ["./vanity"]

EXPOSE 3000
