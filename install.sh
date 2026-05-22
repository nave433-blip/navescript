#!/bin/bash
# install.sh - One-line installer for Navescript

set -e

REPO="nave433-blip/navescript"
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
    echo "Unsupported OS: $OS"
    exit 1
fi

VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$VERSION" ]; then
    VERSION="latest"
fi

URL="https://github.com/$REPO/releases/download/$VERSION/navescript-$PLATFORM"
if [ "$VERSION" == "latest" ]; then
    # Fallback or handled differently if no releases yet
    echo "No release found on GitHub. Please build from source or check back later."
    exit 1
fi

echo "Downloading Navescript $VERSION for $PLATFORM..."
curl -L "$URL" -o navescript
chmod +x navescript

sudo mv navescript /usr/local/bin/navescript

echo "✅ Navescript installed to /usr/local/bin/navescript"
navescript --version
