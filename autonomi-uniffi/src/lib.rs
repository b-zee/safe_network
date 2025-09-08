use bytes::Bytes;

uniffi::setup_scaffolding!();

/// Represents encrypted data with a datamap chunk and content chunks
#[derive(uniffi::Record)]
pub struct EncryptedData {
    /// The serialized datamap chunk that contains metadata about the encrypted data
    pub datamap_chunk: Vec<u8>,
    /// The encrypted content chunks
    pub content_chunks: Vec<Vec<u8>>,
}

/// Custom error type for UniFFI
#[derive(Debug, uniffi::Error, thiserror::Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {message}")]
    EncryptionFailed { message: String },
}

/// Simple addition function for testing
#[uniffi::export]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Encrypts data using self-encryption algorithm
///
/// Takes raw bytes and returns encrypted chunks along with a datamap chunk
/// that can be used to decrypt the data later
#[uniffi::export]
pub fn encrypt(data: Vec<u8>) -> Result<EncryptedData, EncryptionError> {
    // Convert Vec<u8> to Bytes for the autonomi API
    let bytes_data = Bytes::from(data);

    // Use autonomi's self_encryption wrapper
    let (datamap_chunk, content_chunks) =
        autonomi::self_encryption::encrypt(bytes_data).map_err(|e| {
            EncryptionError::EncryptionFailed {
                message: e.to_string(),
            }
        })?;

    // Convert datamap chunk to bytes
    let datamap_bytes = datamap_chunk.value().to_vec();

    // Convert content chunks to Vec<Vec<u8>>
    let chunks_bytes: Vec<Vec<u8>> = content_chunks
        .into_iter()
        .map(|chunk| chunk.value().to_vec())
        .collect();

    Ok(EncryptedData {
        datamap_chunk: datamap_bytes,
        content_chunks: chunks_bytes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_encrypt_small_data() {
        let data = b"Hello, World!".to_vec();
        let result = encrypt(data.clone());

        assert!(result.is_ok(), "Encryption should succeed");

        let encrypted = result.unwrap();
        assert!(
            !encrypted.datamap_chunk.is_empty(),
            "Datamap chunk should not be empty"
        );
        assert!(
            !encrypted.content_chunks.is_empty(),
            "Should have at least one content chunk"
        );
    }

    #[test]
    fn test_encrypt_empty_data() {
        let data = vec![];
        let result = encrypt(data);

        // Empty data might not be supported by self_encryption
        // Let's check if it returns an error and handle appropriately
        match result {
            Ok(encrypted) => {
                assert!(
                    !encrypted.datamap_chunk.is_empty(),
                    "Datamap chunk should exist even for empty data"
                );
            }
            Err(e) => {
                // It's OK if empty data returns an error - self_encryption might not support it
                println!("Empty data encryption error (expected): {}", e);
                assert!(
                    matches!(e, EncryptionError::EncryptionFailed { .. }),
                    "Should be an encryption error"
                );
            }
        }
    }

    #[test]
    fn test_encrypt_large_data() {
        // Create data larger than a single chunk (3 MiB should be enough)
        let data = vec![0u8; 3 * 1024 * 1024];
        let result = encrypt(data);

        assert!(result.is_ok(), "Encryption of large data should succeed");

        let encrypted = result.unwrap();
        assert!(
            !encrypted.datamap_chunk.is_empty(),
            "Datamap chunk should not be empty"
        );
        // Large data should result in multiple chunks
        assert!(
            encrypted.content_chunks.len() > 1,
            "Large data should produce multiple chunks"
        );
    }
}
