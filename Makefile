.PHONY: build test clean run-c-client check

# Default target
all: build test

# Build the Rust library
build:
	cargo build

# Build release version
build-release:
	cargo build --release

# Run Rust tests
test:
	cargo test

# Run specific test
test-specific:
	cargo test $(TEST)

# Build and run C client
run-c-client:
	cargo run --example c_client

# Check code formatting and linting
check:
	cargo fmt --check
	cargo clippy -- -D warnings

# Format code
format:
	cargo fmt

# Clean build artifacts
clean:
	cargo clean

# Generate documentation
docs:
	cargo doc --open

# Install required tools
setup:
	rustup component add clippy rustfmt

# Benchmark (if you add criterion later)
bench:
	cargo bench

# Watch for changes and run tests
watch:
	cargo watch -x test

# Profile with valgrind (requires valgrind)
profile-c:
	cargo build --example c_client
	valgrind --tool=memcheck --leak-check=full ./target/debug/examples/c_client