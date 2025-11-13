# === Stage 1: Build the Rust binary ===
FROM rust:1.89 as builder

# Install required tools (optional, e.g., git for dependencies)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release || true

# Build the release binary
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /final

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/git-contrib-aggregator ./

# Expose the default Actix Web port
EXPOSE 8080

# Run the application
CMD ["./git-contrib-aggregator"]
