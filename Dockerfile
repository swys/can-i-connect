# Stage 1: Builder
FROM rust:1.70-buster AS builder

# Install necessary build dependencies, including musl-tools for static linking
RUN apt-get update && apt-get install -y \
  musl-tools \
  && rm -rf /var/lib/apt/lists/*

# Set environment variables to use musl for static linking
ENV CC=musl-gcc \
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc

# Copy your project files
COPY . /usr/src/can-i-connect
WORKDIR /usr/src/can-i-connect

# Build the project for a static binary using musl
RUN rustup target add aarch64-unknown-linux-musl \
  && cargo build --release --target aarch64-unknown-linux-musl

# Stage 2: Final minimal image with Debian
FROM debian:buster

# Install any necessary debugging tools
RUN apt-get update && apt-get install -y \
  gdb \
  strace \
  curl \
  telnet \
  netcat \
  dnsutils \
  net-tools \
  && rm -rf /var/lib/apt/lists/*

# Copy the statically built binary from the builder stage
COPY --from=builder /usr/src/can-i-connect/target/aarch64-unknown-linux-musl/release/can-i-connect /usr/local/bin/can-i-connect

# Expose port 8000
EXPOSE 8000 9100

# Set the entrypoint for the container
ENTRYPOINT ["/usr/local/bin/can-i-connect"]
