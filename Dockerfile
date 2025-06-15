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
RUN apt update && apt install python3 wget xz-utils -y && apt clean && \
    wget https://github.com/yt-dlp/yt-dlp-nightly-builds/releases/latest/download/yt-dlp -P /usr/bin/ && \
    chmod +x /usr/bin/yt-dlp && \
    wget https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz && \
    tar -xvf ffmpeg-master-latest-linux64-gpl.tar.xz  && \
    mv ffmpeg-master-latest-linux64-gpl/bin/ffmpeg /usr/bin && \
    chmod +x /usr/bin/ffmpeg && \
    rm -rf ffmpeg-master-latest-linux64-gpl.tar.xz && \
    rm -rf ffmpeg-master-latest-linux64-gpl

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/maxine-rust /usr/local/bin/maxine-rust

# Set the startup command to run your binary
CMD ["maxine-rust"] 