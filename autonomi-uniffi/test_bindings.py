#!/usr/bin/env python3
"""
Test script to verify the UniFFI bindings are working correctly
by calling the Rust functions through Python bindings (if we generate them)
"""

import sys
import subprocess

# First, let's generate Python bindings to test our implementation
print("Generating Python bindings...")
result = subprocess.run([
    "cargo", "run", "--bin", "uniffi-bindgen", 
    "generate", "--library", "../target/release/libautonomi_uniffi.dylib",
    "--language", "python", "--out-dir", "out_python"
], capture_output=True, text=True)

if result.returncode != 0:
    print(f"Failed to generate Python bindings: {result.stderr}")
    sys.exit(1)

print("Python bindings generated successfully!")

# Now let's test the bindings
sys.path.insert(0, 'out_python')

try:
    from autonomi_uniffi import add, encrypt, EncryptedData, EncryptionError
    
    # Test 1: Simple add function
    result = add(5, 3)
    print(f"\nTest 1 - Add function: 5 + 3 = {result}")
    assert result == 8, f"Expected 8, got {result}"
    
    # Test 2: Encrypt small data
    test_data = b"Hello, Autonomi Network!"
    try:
        encrypted = encrypt(test_data)
        print(f"\nTest 2 - Encrypt small data:")
        print(f"  Input size: {len(test_data)} bytes")
        print(f"  Datamap chunk size: {len(encrypted.datamap_chunk)} bytes")
        print(f"  Number of content chunks: {len(encrypted.content_chunks)}")
        for i, chunk in enumerate(encrypted.content_chunks):
            print(f"    Chunk {i}: {len(chunk)} bytes")
    except EncryptionError as e:
        print(f"  Encryption failed: {e}")
    
    # Test 3: Encrypt empty data (might fail)
    empty_data = b""
    try:
        encrypted = encrypt(empty_data)
        print(f"\nTest 3 - Empty data encryption succeeded")
        print(f"  Datamap chunk size: {len(encrypted.datamap_chunk)} bytes")
    except EncryptionError as e:
        print(f"\nTest 3 - Empty data encryption failed (expected): {e}")
    
    # Test 4: Encrypt large data
    large_data = b"x" * (3 * 1024 * 1024)  # 3MB
    try:
        encrypted = encrypt(large_data)
        print(f"\nTest 4 - Encrypt large data:")
        print(f"  Input size: {len(large_data)} bytes")
        print(f"  Datamap chunk size: {len(encrypted.datamap_chunk)} bytes")
        print(f"  Number of content chunks: {len(encrypted.content_chunks)}")
    except EncryptionError as e:
        print(f"  Large data encryption failed: {e}")
    
    print("\n✅ All tests completed!")
    
except ImportError as e:
    print(f"Failed to import bindings: {e}")
    print("\nMake sure the library path is correct and the bindings were generated properly")
    sys.exit(1)