#!/bin/bash
set -e

# Run Rust tests
cd rust
cargo test
cd ..

# Run Python tests
cd python
pytest
cd ..