#!/bin/bash
set -e

# Ensure we're using nightly Rust
rustup override set nightly

# Remove existing lockfile and regenerate with proper flag
rm -f Cargo.lock
cargo generate-lockfile -Znext-lockfile-bump

# Build the regular Rust code
cargo build -Znext-lockfile-bump

# Now build the Solana program directly using cargo-build-sbf
# We need to use the program directory directly
cd programs/florin-token

# Use solana-program-library's build tools directly
echo "Building Solana program..."
cargo build-sbf -- --features no-entrypoint

echo "Build completed!"
