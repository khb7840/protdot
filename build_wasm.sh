#!/bin/bash

# Build script for WASM

echo "Building Protein Viewer for WebAssembly..."

# Check if wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build for WASM
echo "Compiling to WASM..."
cargo build --release --target wasm32-unknown-unknown

# Create web directory if it doesn't exist
mkdir -p web

# Copy the WASM file
echo "Copying files to web directory..."
cp target/wasm32-unknown-unknown/release/protein_viewer.wasm web/

# Copy HTML file
cp index.html web/

# Download macroquad's JS glue code
echo "Downloading macroquad JS glue code..."
curl -s https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js > web/mq_js_bundle.js

echo ""
echo "✅ Build complete!"
echo ""
echo "To run the web app:"
echo "1. cd web"
echo "2. python3 -m http.server 8000"
echo "3. Open http://localhost:8000 in your browser"
echo ""
