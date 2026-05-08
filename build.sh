#!/bin/bash
set -e
echo "Building WASM package..."
wasm-pack build --target bundler --release 2>&1 || { echo "wasm-pack not found or failed. Checking compilation..."; cargo check --target wasm32-unknown-unknown 2>&1; }
echo "Copying to npm-package/"
if [ -d "pkg" ]; then
    cp -r pkg/* npm-package/ 2>/dev/null || true
fi
echo "Done"
