#!/bin/bash
# Build script for Navescript ecosystem
echo "Building Navescript ecosystem..."
# Build Go entry points but use native logic where possible
go build -o bin/nvs ./cmd/nvs/main.go
go build -o bin/nvm ./cmd/nvm/main.go
go build -o bin/nvs-debug ./cmd/nvs-debug/main.go
echo "Build complete."
