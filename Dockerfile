# Use Rust slim image as the base image
FROM rust:1.76.0-slim-buster

# Set the working directory
WORKDIR /usr/src/can-i-connect

# Install OpenSSL and necessary libraries
RUN apt-get update && apt-get install -y \
  libssl-dev \
  pkg-config \
  git \
  dnsutils \
  netcat-openbsd \
  && rm -rf /var/lib/apt/lists/*

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Copy the rest of the source code
COPY . .

# Build the project in release mode
RUN cargo build --release

# Copy the Rust binary to the clean directory
RUN cp /usr/src/can-i-connect/target/release/can-i-connect /usr/local/bin/

# Expose a port if needed
EXPOSE 8000

# Set the entry point for the container
ENTRYPOINT ["/usr/local/bin/can-i-connect"]