FROM rust:1.81 as builder
WORKDIR /usr/src/app

# Copy only the dependency files first
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
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
    apt-get install -y --no-install-recommends \
    libssl-dev \
    ca-certificates \
    curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/penumbra-indexer /usr/local/bin/

# Create a non-root user for better security
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy the env file for Railway
COPY .env.production /app/.env

# Expose the port
EXPOSE 3000

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/penumbra-indexer

# Switch to non-root user
USER appuser

# Healthcheck to ensure the service is running
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:3000/api/health || exit 1

CMD ["/usr/local/bin/penumbra-indexer"]