#!/bin/bash
# NaveScript Install Script

echo "🔧 Installing NaveScript..."

# Check OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    cp bin/nvs /usr/local/bin/nvs
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    sudo cp bin/nvs /usr/local/bin/nvs
else
    echo "Unsupported OS"
    exit 1
fi

echo "✅ NaveScript installed successfully."
nvs --help
