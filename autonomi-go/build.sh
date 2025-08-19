#!/bin/bash

# Build script for Autonomi Go bindings

set -e

echo "Building Autonomi Go bindings..."

# Build the Rust FFI library
echo "Building Rust FFI library..."
cargo build --release -p autonomi-ffi

# The library should be in ../target/release/
if [ ! -f "../target/release/libautonomi_ffi.so" ] && [ ! -f "../target/release/libautonomi_ffi.dylib" ] && [ ! -f "../target/release/libautonomi_ffi.a" ]; then
    echo "Error: FFI library not found in ../target/release/"
    echo "Please ensure the Rust library built successfully"
    exit 1
fi

echo "Rust FFI library built successfully"

# Run Go tests
echo "Running Go tests..."
cd autonomi
go test -v -short
cd ..

echo "Build complete!"
echo ""
echo "To run the example:"
echo "  1. Start a local network: cargo run --bin antctl -- local run --build --clean"
echo "  2. Set your private key: export SECRET_KEY=<your_private_key>"
echo "  3. Run the example: cd examples && go run basic.go"