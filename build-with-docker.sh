#!/bin/bash
set -e

echo "Building Florin token program with Docker..."
echo "This will use the projectserum/build:v0.31.1 Docker image to build the program"
echo "with proper support for Token-2022 extensions."

# Build the Docker image
docker build -t florin-token-builder .

# Run the Docker container to build the program
docker run -v $(pwd):/app -w /app florin-token-builder

echo "Build completed! Check the target directory for the build output."
