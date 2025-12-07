# Stage 1: Build
FROM rust:1.91 AS builder

WORKDIR /usr/src/app

# Copy manifest files and build dependencies first (for layer caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy actual source code and rebuild
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install CA certificates and other runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/rust-web-starter /app/rust-web-starter

# Copy static assets and templates
COPY assets /app/assets
COPY templates /app/templates

# Set environment variables for asset paths
ENV ASSETS_DIR=/app/assets
ENV TEMPLATES_DIR=/app/templates

# Expose the port the app runs on
EXPOSE 3000

# Run the application
CMD ["/app/rust-web-starter"]