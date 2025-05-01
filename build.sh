#!/bin/bash
set -e

# Florin Token Build Script
# This script automates the build process for the Florin token project
# with the correct Solana, Anchor, and Rust versions.

echo "🌟 Starting Florin token build process..."

# Check if Solana CLI is installed and has the correct version
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Installing Solana CLI v2.1.5..."
    sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.5/install)"
    export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"
else
    SOLANA_VERSION=$(solana --version | awk '{print $2}')
    if [[ "$SOLANA_VERSION" != "2.1.5" ]]; then
        echo "⚠️ Warning: Solana CLI version is $SOLANA_VERSION, but 2.1.5 is recommended."
        echo "Consider running: sh -c \"$(curl -sSfL https://release.anza.xyz/v2.1.5/install)\""
    else
        echo "✅ Solana CLI v2.1.5 detected."
    fi
fi

# Check if Rust is installed with the correct version
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    rustup install 1.79.0
else
    echo "✅ Rust detected."
fi

# Set Rust version for this project
echo "🔧 Setting Rust 1.79.0 for this project..."
rustup install 1.79.0
rustup override set 1.79.0

# Check for required system packages
echo "🔍 Checking for required system packages..."
if ! dpkg -l | grep -q libssl-dev || ! dpkg -l | grep -q pkg-config; then
    echo "📦 Installing required packages: pkg-config libssl-dev..."
    apt update && apt install -y pkg-config libssl-dev
else
    echo "✅ Required packages already installed."
fi

# Clean build artifacts if requested
if [[ "$1" == "clean" ]]; then
    echo "🧹 Cleaning previous build artifacts..."
    rm -f Cargo.lock
    rm -rf target
fi

# Build the project
echo "🔨 Building Florin token project..."
anchor build

echo "✅ Build completed successfully!"
echo "🚀 You can now deploy the program using: anchor deploy"
