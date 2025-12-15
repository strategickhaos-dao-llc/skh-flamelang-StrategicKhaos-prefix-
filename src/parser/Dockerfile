# src/parser/Dockerfile
FROM rust:1.75-slim AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release --bin flame-parser-test  # Add bin target in Cargo.toml later

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/flame-parser-test /usr/local/bin/
CMD ["flame-parser-test"]
