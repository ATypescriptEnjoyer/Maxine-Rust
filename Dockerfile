# Build stage
FROM rust:1.87.0-slim-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    make \
    gcc \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy over your manifests
COPY . .

# Build your application
# This is the caching Docker layer
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    ffmpeg \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/* \
    && pip3 install yt-dlp

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/maxine-rust /usr/local/bin/maxine-rust

# Set the startup command to run your binary
CMD ["maxine-rust"] 