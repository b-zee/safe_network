#!/bin/bash

# Compile and run the Swift example with UniFFI bindings

echo "Compiling Swift example..."

# Set library path for runtime
export DYLD_LIBRARY_PATH="../target/release:$DYLD_LIBRARY_PATH"

# Compile with simpler flags
swiftc \
  -L ../target/release \
  -lautonomi_uniffi \
  -I out \
  -import-objc-header out/autonomi_uniffiFFI.h \
  out/autonomi_uniffi.swift \
  example.swift \
  -o example_test

if [ $? -eq 0 ]; then
    echo "Running example..."
    ./example_test
else
    echo "Compilation failed"
    exit 1
fi