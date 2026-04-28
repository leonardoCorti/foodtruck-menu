# -------- Build stage --------
FROM rust:1.89 AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
# Copy real source
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN rm -fr ./src
COPY src ./src
COPY templates/ ./templates/
RUN cargo build --release

# -------- Runtime stage --------
FROM debian:bookworm-slim

# Install minimal runtime deps
RUN apt-get update && apt-get install -y \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*


WORKDIR /app

RUN mkdir /app/settings

COPY static/ /app/static/
COPY templates/ /app/templates/

# Copy compiled binary
COPY --from=builder /app/target/release/foodtruck-menu /app/app

# Use non-root user (important)
RUN useradd -m appuser
USER appuser

# Expose your port (change if needed)
EXPOSE 31151

# Run the app
CMD ["./app"]
