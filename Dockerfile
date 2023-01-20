FROM rust:1.62.0-slim-bullseye as builder
WORKDIR /app
COPY Cargo.toml /app/
COPY src /app/src/
COPY templates /app/templates/
RUN cargo build --release

FROM debian:bullseye-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/sample-rust-axum /app/sample-rust-axum
ENTRYPOINT ["/app/sample-rust-axum"]