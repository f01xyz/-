#!/bin/bash
set -e

# Ensure we're using nightly
rustup override set nightly

# Remove existing lockfile
rm -f Cargo.lock

# Generate a new lockfile with the proper flag
cargo generate-lockfile -Znext-lockfile-bump

# Build the Rust code first
cargo build -Znext-lockfile-bump

# Now try to build the Solana program
cd programs/florin-token
cargo build-bpf --features no-entrypoint -Znext-lockfile-bump

echo "Build completed successfully!"
