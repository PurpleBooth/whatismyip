# This help screen
show-help:
        just --list

# Test it was built ok
test:
        RUST_BACKTRACE=1 cargo test

# Test the markdown in the docs directory
specdown: build
        specdown run --temporary-workspace-dir --add-path "{{justfile_directory()}}/target/release" ./README.md

# Run a smoke test and see if the app runs
smoke-test: build
        cargo run --bin ellipsis -- -h

# Build release version
build:
        cargo build --release

# Lint it
lint:
        cargo +nightly fmt --all -- --check
        cargo +nightly clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo -A clippy::multiple-crate-versions
        cargo +nightly check
        cargo +nightly audit

# Format what can be formatted
fmt:
        cargo +nightly fix --allow-dirty
        cargo +nightly clippy --allow-dirty --fix -Z unstable-options --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::cargo -D clippy::nursery -A clippy::multiple-crate-versions
        cargo +nightly fmt --all
        yamlfmt -w .github/*.yml .github/workflows/*.yml .*.yml

# Clean the build directory
clean:
        cargo clean
