# Use an official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

# Install system dependencies for smbclient (for building with libsmbclient)
RUN apt-get update && apt-get install -y \
    samba-client \
    libsmbclient-dev

# This command creates a dummy main.rs to satisfy dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies first to cache them
RUN cargo build --release

# Copy the actual source files
COPY . .

# Build the final binary
RUN cargo build --release

# Set the entry point to run the compiled binary
CMD ["./target/release/smbiote"]