#!/bin/bash
# install.sh - One-line installer for Navescript

set -e

REPO="naveos/navescript"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$OS" == "darwin" ]; then
    if [ "$ARCH" == "arm64" ]; then
        PLATFORM="aarch64-apple-darwin"
    else
        PLATFORM="x86_64-apple-darwin"
    fi
elif [ "$OS" == "linux" ]; then
    if [ "$ARCH" == "aarch64" ]; then
        PLATFORM="aarch64-unknown-linux-gnu"
    else
        PLATFORM="x86_64-unknown-linux-gnu"
    fi
else
    # Default to x86_64 linux
    PLATFORM="x86_64-unknown-linux-gnu"
fi

VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$VERSION" ]; then
    VERSION="v3.0.0"
fi

URL="https://github.com/$REPO/releases/download/$VERSION/navescript-$PLATFORM"

echo "🚀 Installing Nλvescript $VERSION ($PLATFORM)..."
curl -L "$URL" -o navescript
chmod +x navescript

sudo mv navescript /usr/local/bin/navescript

echo "✅ Nλvescript installed successfully to /usr/local/bin/navescript"
navescript --version
echo "💡 Run 'nvs camp' to get started!"
