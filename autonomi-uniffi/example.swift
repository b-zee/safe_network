import Foundation

// Include the generated bindings (they will be compiled together)
// The actual binding code is in autonomi_uniffi.swift

// Example usage of the autonomi_uniffi bindings
// This demonstrates how to use the self_encryption::encrypt function from Swift

// Example 1: Test the simple add function
let sum = add(left: 5, right: 3)
print("5 + 3 = \(sum)")

// Example 2: Encrypt some data
let testData = "Hello, Autonomi Network!".data(using: .utf8)!

do {
    let encrypted = try encrypt(data: testData)
    
    print("\nEncryption successful!")
    print("Datamap chunk size: \(encrypted.datamapChunk.count) bytes")
    print("Number of content chunks: \(encrypted.contentChunks.count)")
    
    for (index, chunk) in encrypted.contentChunks.enumerated() {
        print("  Chunk \(index): \(chunk.count) bytes")
    }
} catch let error as EncryptionError {
    print("Encryption failed: \(error)")
} catch {
    print("Unexpected error: \(error)")
}

// Example 3: Try encrypting empty data (this might fail depending on self_encryption implementation)
let emptyData = Data()

do {
    let encrypted = try encrypt(data: emptyData)
    print("\nEmpty data encryption successful!")
    print("Datamap chunk size: \(encrypted.datamapChunk.count) bytes")
} catch let error as EncryptionError {
    print("\nEmpty data encryption failed (expected): \(error)")
} catch {
    print("\nUnexpected error: \(error)")
}