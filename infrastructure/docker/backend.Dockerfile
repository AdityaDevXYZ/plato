
FROM rust:1.77-slim-bookworm AS builder
WORKDIR /app
# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev protobuf-compiler

COPY . .
# Build release binary for the API Gateway / Monolith
RUN cargo build --release --bin plato-api

FROM debian:bookworm-slim AS runner
WORKDIR /app
# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/plato-api /usr/local/bin/plato-api

EXPOSE 8080
CMD ["plato-api"]
