//! Utility functions for hashing and compression

use anyhow::Result;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::io::{Read, Write};

/// Compute SHA-1 hash of data
///
/// # Arguments
///
/// * `data` - The data to hash
///
/// # Returns
///
/// A 40-character hexadecimal string
///
/// # Example
///
/// ```
/// let hash = oxid::utils::hash_data(b"Hello World");
/// assert_eq!(hash.len(), 40);
/// ```
pub fn hash_data(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Compress data using zlib
///
/// # Arguments
///
/// * `data` - The data to compress
///
/// # Returns
///
/// Compressed data as bytes
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

/// Decompress zlib-compressed data
///
/// # Arguments
///
/// * `data` - The compressed data
///
/// # Returns
///
/// Decompressed data as bytes
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_data() {
        let data = b"Hello World";
        let hash = hash_data(data);

        // SHA-1 always produces 40 hex characters
        assert_eq!(hash.len(), 40);

        // Same input always produces same hash
        let hash2 = hash_data(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_compress_decompress() {
        let original = b"Hello World, this is a test of compression!";
        let compressed = compress(original).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(original.to_vec(), decompressed);

        // Compression should make it smaller for text
        assert!(compressed.len() < original.len());
    }

    #[test]
    fn test_git_blob_hash() {
        // Test that we compute the same hash as Git for "Hello World"
        let content = b"Hello World";
        let blob_data = format!("blob {}\0", content.len());
        let mut data = blob_data.as_bytes().to_vec();
        data.extend_from_slice(content);

        let hash = hash_data(&data);

        // This is the known Git hash for "Hello World"
        assert_eq!(hash, "557db03de997c86a4a028e1ebd3a1ceb225be238");
    }
}
