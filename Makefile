# Variables
TARGET := target/release/smbiote

.PHONY: all build install run clean test

# Default target
all: build

# Build the project
build:
	cargo build --release

# Install the binary into /usr/local/bin
install: build
	cp $(TARGET) /usr/local/bin/smbiote

# Run the project
run:
	$(TARGET)

# Clean the build artifacts
clean:
	cargo clean

# Test the project
test:
	cargo test
