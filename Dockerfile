# FlameLang v2.0.0 Compiler Docker Image
# Multi-stage build for optimal size

# Stage 1: Build the compiler
FROM rust:1.75 as builder

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY compiler/ ./compiler/
COPY runtime/ ./runtime/
COPY stdlib/ ./stdlib/

# Build release version
RUN cargo build --release --bin flamelang-cli

# Stage 2: Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libicu72 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /build/target/release/flamelang-cli /usr/local/bin/flamelang

# Set working directory
WORKDIR /workspace

# Set entry point
ENTRYPOINT ["flamelang"]
CMD ["info"]

# Labels
LABEL org.opencontainers.image.title="FlameLang Compiler"
LABEL org.opencontainers.image.description="FlameLang v2.0.0 sovereign compiler toolchain"
LABEL org.opencontainers.image.version="2.0.0"
LABEL org.opencontainers.image.vendor="StrategicKhaos DAO LLC"
