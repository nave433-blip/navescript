#!/bin/bash
# NaveScript Native Registry Uploader
# This tool allows users to package and distribute native Navescript packages.

echo "📦 Packaging Native Navescript dependency..."

# 1. Validate navescript.json
if [ ! -f navescript.json ]; then
    echo "❌ Error: navescript.json not found."
    exit 1
fi

# 2. Compile native code
nvs build --target nvs_ir

# 3. Package
tar -czf package.nvp . --exclude=.git --exclude=.navescript

# 4. Upload
echo "🚀 Uploading to NaveScript Registry..."
curl -F "file=@package.nvp" https://reg.navescript.org/upload

echo "✅ Package distributed successfully."
