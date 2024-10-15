# Stage 1: Builder
FROM rust:1.70-buster AS builder

# Install necessary build dependencies
RUN apt-get update && apt-get install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  git \
  && rm -rf /var/lib/apt/lists/*

# Copy your project files
COPY . /usr/src/can-i-connect
WORKDIR /usr/src/can-i-connect

# Build the project for a static binary
RUN cargo build --release

# Stage 2: Final minimal image
FROM debian:buster

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
  libssl1.1 \
  && rm -rf /var/lib/apt/lists/*

# Copy the statically built binary from the builder stage
COPY --from=builder /usr/src/can-i-connect/target/release/can-i-connect /usr/local/bin/can-i-connect

# Expose port 8000
EXPOSE 8000

# Set the entrypoint for the container
ENTRYPOINT ["/usr/local/bin/can-i-connect"]
