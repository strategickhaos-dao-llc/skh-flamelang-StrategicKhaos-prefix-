//! Quantum-resistant cryptographic primitives
//! 
//! Implements NIST Post-Quantum Cryptography winners:
//! - CRYSTALS-Kyber for key exchange (lattice-based)
//! - CRYSTALS-Dilithium for digital signatures (lattice-based)
//! - SPHINCS+ for hash-based signatures

use crate::FlameError;
use pqcrypto_kyber::kyber1024;
use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::kem::{PublicKey as KemPublicKey, SharedSecret as KemSharedSecret};
use pqcrypto_traits::sign::{PublicKey as SigPublicKey, SignedMessage as SigSignedMessage};

/// Quantum-resistant key pair for encryption
pub struct QuantumResistantKeyPair {
    pub kem_public: kyber1024::PublicKey,
    pub kem_secret: kyber1024::SecretKey,
    pub sig_public: dilithium5::PublicKey,
    pub sig_secret: dilithium5::SecretKey,
}

impl QuantumResistantKeyPair {
    /// Generate new quantum-resistant key pair
    pub fn generate() -> Self {
        let (kem_public, kem_secret) = kyber1024::keypair();
        let (sig_public, sig_secret) = dilithium5::keypair();

        Self {
            kem_public,
            kem_secret,
            sig_public,
            sig_secret,
        }
    }

    /// Export public key bytes for Kyber
    pub fn kyber_public_bytes(&self) -> Vec<u8> {
        self.kem_public.as_bytes().to_vec()
    }

    /// Export public key bytes for Dilithium
    pub fn dilithium_public_bytes(&self) -> Vec<u8> {
        self.sig_public.as_bytes().to_vec()
    }
}

/// Kyber-based key exchange
pub struct KeyExchange;

impl KeyExchange {
    /// Encapsulate shared secret with recipient's public key
    /// Returns (shared_secret, ciphertext) - note the order from pqcrypto
    pub fn encapsulate(
        public_key: &kyber1024::PublicKey,
    ) -> (kyber1024::SharedSecret, kyber1024::Ciphertext) {
        kyber1024::encapsulate(public_key)
    }

    /// Decapsulate shared secret with secret key
    pub fn decapsulate(
        ciphertext: &kyber1024::Ciphertext,
        secret_key: &kyber1024::SecretKey,
    ) -> kyber1024::SharedSecret {
        kyber1024::decapsulate(ciphertext, secret_key)
    }

    /// Convert shared secret to 32-byte key
    pub fn shared_secret_to_key(shared_secret: &kyber1024::SharedSecret) -> [u8; 32] {
        use sha3::{Digest, Sha3_256};
        
        let mut hasher = Sha3_256::new();
        hasher.update(KemSharedSecret::as_bytes(shared_secret));
        let result = hasher.finalize();
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        key
    }
}

/// Dilithium-based signature scheme
pub struct SignatureScheme;

impl SignatureScheme {
    /// Sign a message with Dilithium
    pub fn sign(message: &[u8], secret_key: &dilithium5::SecretKey) -> dilithium5::SignedMessage {
        dilithium5::sign(message, secret_key)
    }

    /// Verify a signed message
    pub fn verify(
        signed_message: &dilithium5::SignedMessage,
        public_key: &dilithium5::PublicKey,
    ) -> Result<Vec<u8>, FlameError> {
        dilithium5::open(signed_message, public_key).map_err(|_| FlameError::Transform {
            layer: 0,
            message: "Signature verification failed".to_string(),
        })
    }

    /// Create detached signature
    pub fn sign_detached(message: &[u8], secret_key: &dilithium5::SecretKey) -> Vec<u8> {
        let signed = dilithium5::sign(message, secret_key);
        signed.as_bytes().to_vec()
    }
}

/// Hash-based signature support (SPHINCS+)
#[allow(dead_code)]
pub mod sphincs {
    use pqcrypto_sphincsplus::sphincsshake128ssimple as sphincs;

    /// Generate SPHINCS+ key pair
    pub fn keypair() -> (sphincs::PublicKey, sphincs::SecretKey) {
        sphincs::keypair()
    }

    /// Sign with SPHINCS+
    pub fn sign(message: &[u8], secret_key: &sphincs::SecretKey) -> sphincs::SignedMessage {
        sphincs::sign(message, secret_key)
    }

    /// Verify SPHINCS+ signature
    pub fn verify(
        signed_message: &sphincs::SignedMessage,
        public_key: &sphincs::PublicKey,
    ) -> Result<Vec<u8>, crate::FlameError> {
        sphincs::open(signed_message, public_key).map_err(|_| crate::FlameError::Transform {
            layer: 0,
            message: "SPHINCS+ signature verification failed".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = QuantumResistantKeyPair::generate();
        assert!(keypair.kyber_public_bytes().len() > 0);
        assert!(keypair.dilithium_public_bytes().len() > 0);
    }

    #[test]
    fn test_key_exchange() {
        let keypair = QuantumResistantKeyPair::generate();
        let (shared_secret1, ciphertext) = KeyExchange::encapsulate(&keypair.kem_public);
        let shared_secret2 = KeyExchange::decapsulate(&ciphertext, &keypair.kem_secret);
        
        assert_eq!(KemSharedSecret::as_bytes(&shared_secret1), KemSharedSecret::as_bytes(&shared_secret2));
    }

    #[test]
    fn test_signature() {
        let keypair = QuantumResistantKeyPair::generate();
        let message = b"Test message for FlameVault";
        
        let signed = SignatureScheme::sign(message, &keypair.sig_secret);
        let verified = SignatureScheme::verify(&signed, &keypair.sig_public).unwrap();
        
        assert_eq!(message.as_slice(), verified.as_slice());
    }
}
