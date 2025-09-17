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

# Individual problem test targets
test-two-sum:
	cargo test two_sum

test-reverse-string:
	cargo test reverse_string

test-valid-parentheses:
	cargo test valid_parentheses

test-max-subarray:
	cargo test max_subarray

test-binary-search:
	cargo test binary_search

test-merge-intervals:
	cargo test merge_intervals

test-add-two-numbers:
	cargo test add_two_numbers

test-course-schedule:
	cargo test course_schedule

test-lru-cache:
	cargo test lru_cache

test-max-area:
	cargo test max_area

test-longest-substring:
	cargo test longest_substring

test-group-anagrams:
	cargo test group_anagrams

test-level-order:
	cargo test level_order

test-meeting-rooms:
	cargo test meeting_rooms

# Test all new problems
test-new-problems:
	cargo test two_sum reverse_string valid_parentheses max_subarray binary_search merge_intervals

# Test all existing problems
test-existing-problems:
	cargo test add_two_numbers course_schedule lru_cache max_area longest_substring group_anagrams level_order meeting_rooms