#!/bin/bash
set -e

# Run build script
./scripts/build.sh

# Create platform-specific packages
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Create release directory for this platform
mkdir -p release/$PLATFORM-$ARCH

# Copy binaries
cp rust/target/release/ngm-core release/$PLATFORM-$ARCH/
cp -r python/ngm_cli release/$PLATFORM-$ARCH/
cp python/requirements.txt release/$PLATFORM-$ARCH/

# Create archive
tar -czf release/ngm-$PLATFORM-$ARCH.tar.gz -C release/$PLATFORM-$ARCH .