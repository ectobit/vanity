# syntax=docker/dockerfile:1.3
FROM rust:1.58.1 AS builder

ARG TARGETPLATFORM

WORKDIR /app

RUN cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/app/target,id=${TARGETPLATFORM} cargo build --release && \
    cargo strip && \
    mv /app/target/release/vanity /app

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/vanity /

CMD ["./vanity"]

EXPOSE 3000
