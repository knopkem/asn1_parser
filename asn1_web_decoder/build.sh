#!/bin/bash

echo "Building WebAssembly module..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly module
wasm-pack build --target web --out-dir www/pkg

echo "Build complete! WebAssembly module is in www/pkg/"
echo "To serve the website locally, run:"
echo "  cd www && python3 -m http.server 8080"
echo "Then open http://localhost:8080 in your browser"
