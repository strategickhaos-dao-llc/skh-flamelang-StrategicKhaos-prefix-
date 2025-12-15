//! Hash computation for tamper-evident visualizations
//! 
//! NOTE: This is a placeholder implementation using DefaultHasher.
//! For production use, integrate a proper cryptographic hash like SHA-256 from the sha2 crate.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Compute hash of data (placeholder implementation)
/// 
/// WARNING: This is NOT cryptographically secure. Uses DefaultHasher as a placeholder.
/// For production tamper-evident proofs, replace with SHA-256 or similar.
pub fn compute_hash(data: &str) -> String {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    let hash_value = hasher.finish();
    
    // Format as hex string (64 chars for SHA-256 simulation)
    format!("{:016x}{:016x}{:016x}{:016x}", 
            hash_value, 
            hash_value.rotate_left(16),
            hash_value.rotate_right(16),
            !hash_value)
}

/// Compute hash from bytes (placeholder implementation)
/// 
/// WARNING: This is NOT cryptographically secure. For production use, replace with SHA-256.
pub fn compute_hash_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish().to_le_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() {
        let data = "test data";
        let hash1 = compute_hash(data);
        let hash2 = compute_hash(data);
        
        // Same data should produce same hash
        assert_eq!(hash1, hash2);
        
        // Hash should be 64 characters (simulating SHA-256)
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_different_data_different_hash() {
        let hash1 = compute_hash("data1");
        let hash2 = compute_hash("data2");
        
        // Different data should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_compute_hash_bytes() {
        let data = b"test data";
        let hash = compute_hash_bytes(data);
        
        assert!(!hash.is_empty());
    }
}
