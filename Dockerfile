# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/actix_web_tutorial /app/actix_web_tutorial

# Expose port
EXPOSE 8080

# Run the application
CMD ["./actix_web_tutorial"]

