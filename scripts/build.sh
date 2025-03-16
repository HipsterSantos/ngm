#!/bin/bash
set -e

# Build Rust components
cd rust
cargo build --release
cd ..

# Install Python dependencies
cd python
pip install -r requirements.txt
cd ..

# Create release directory
mkdir -p release