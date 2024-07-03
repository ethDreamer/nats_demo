# Start from the Rust official image
FROM rust:bullseye as builder

# Set the working directory
WORKDIR /usr/src/nats-demo

# Copy the Cargo.lock and Cargo.toml first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies so this is cached
RUN cargo fetch

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Create a new stage with a minimal image
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /usr/src/nats-demo

# Copy the binaries from the builder stage
COPY --from=builder /usr/src/nats-demo/target/release/producer /usr/local/bin/producer
COPY --from=builder /usr/src/nats-demo/target/release/consumer /usr/local/bin/consumer

# Do nothing, this will be overridden in configs
ENTRYPOINT []

