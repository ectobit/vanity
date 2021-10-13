FROM rust:1.55.0 AS builder

WORKDIR /app

RUN cargo install cargo-strip

COPY . /app

RUN cargo build --release --all-features && cargo strip

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/target/release/vanity /

CMD ["./vanity"]

EXPOSE 3000
