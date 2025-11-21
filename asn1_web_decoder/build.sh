#!/bin/bash

echo "Building WebAssembly module..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg

# Create symlink for React app if it doesn't exist
if [ ! -e www/src/wasm ]; then
    echo "Creating symlink for React app..."
    ln -s ../../pkg www/src/wasm
fi

echo "Build complete! WebAssembly module is in pkg/"
echo "To serve the React website locally, run:"
echo "  cd www && npm run dev"
echo "Then open http://localhost:8080 in your browser"
