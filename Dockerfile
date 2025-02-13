FROM rust:1.81 as builder
WORKDIR /usr/src/app

# Copy only the dependency files first
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the real source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/penumbra-indexer /usr/local/bin/

# Expose the port
EXPOSE 3000

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/penumbra-indexer

CMD ["/usr/local/bin/penumbra-indexer"]
