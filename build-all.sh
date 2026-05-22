#!/bin/bash
# build-all.sh - Build Navescript for all major platforms

set -e

VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f 2)
echo "Building Navescript v$VERSION..."

PLATFORMS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
)

mkdir -p dist

for platform in "${PLATFORMS[@]}"; do
    echo "Building for $platform..."
    rustup target add $platform || true
    cargo build --release --target $platform
    
    BINARY="target/$platform/release/navescript"
    if [[ "$platform" == *"windows"* ]]; then
        BINARY="${BINARY}.exe"
    fi
    
    if [ -f "$BINARY" ]; then
        cp "$BINARY" "dist/navescript-$platform"
        echo "✅ Built $platform"
    else
        echo "❌ Failed to build $platform"
    fi
done

echo "Build complete. Binaries are in dist/"
