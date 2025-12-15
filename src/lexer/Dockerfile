# src/lexer/Dockerfile
FROM rust:1.75-slim AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release --bin flame-lexer-test  # Assuming we add a bin target later

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/flame-lexer-test /usr/local/bin/
CMD ["flame-lexer-test"]
