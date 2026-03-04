# Use the official Rust image as the base builder
FROM rust:1.80 as builder

WORKDIR /usr/src/syntropy

# Install system dependencies (e.g., for Headless Chrome and cpal)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libasound2-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the Syntropy CLI (Headless daemon) for release
RUN cargo build --release -p syntropy-cli

# Create the minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies for Chrome, Audio, and Wasmtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libasound2 \
    chromium \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder environment
COPY --from=builder /usr/src/syntropy/target/release/syntropy-cli /app/syntropy-cli

# Set the default configuration environment
ENV SYNTROPY_ENV=production

# The default process is the daemon
ENTRYPOINT ["/app/syntropy-cli"]
