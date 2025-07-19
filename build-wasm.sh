#!/bin/bash

# WASM Build Script for Local Development
set -e

echo "🚀 Building WASM Task Manager..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Please install it first:"
    echo "   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WASM package
echo "📦 Building WASM package..."
wasm-pack build --target web --out-dir www/pkg

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📂 Generated files in www/pkg/:"
    ls -la www/pkg/
    echo ""
    echo "🌐 To serve locally, run:"
    echo "   cd www && python3 -m http.server 8000"
    echo "   Then open http://localhost:8000"
else
    echo "❌ Build failed!"
    exit 1
fi