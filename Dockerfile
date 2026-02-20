# --- Stage 1: build mdBook binary ---
FROM rust:1.83 AS builder
RUN cargo install mdbook

# --- Stage 2: final Python runtime ---
FROM python:3.10-slim

# Copy mdBook binary only (no Rust toolchain)
COPY --from=builder /usr/local/cargo/bin/mdbook /usr/local/bin/mdbook

# Install dependencies
RUN apt-get update && apt-get install -y curl gcc && rm -rf /var/lib/apt/lists/*

# Set up working directory
WORKDIR /code
COPY . /code

# Python environment
RUN python -m venv env
RUN . env/bin/activate && pip install --no-cache-dir -r requirements.txt

# Build and serve docs
RUN mdbook build
CMD mdbook watch & (cd /code/docs/pandocs/ && python3 -m http.server)
