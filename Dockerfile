# ---- Build stage ----
FROM rust:1.76-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

# Build release binary
RUN cargo build --release --bin soil-query-api

# ---- Runtime stage ----
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from build stage
COPY --from=builder /app/target/release/soil-query-api .

# Railway injects PORT env variable
ENV PORT=3000

# Data directory — Railway Volume will be mounted here
ENV DATABASE_PATH=/data/soil_data.db

EXPOSE 3000

CMD ["./soil-query-api"]