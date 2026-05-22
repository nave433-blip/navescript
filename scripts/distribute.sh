#!/bin/bash
# NaveScript Distribution Script

echo "🚀 Packaging NaveScript for distribution..."

# 1. Build
make build

# 2. Test
make test

# 3. Package
make release

echo "✨ Packaging complete. Artifacts are in navescript/release/packages/"
