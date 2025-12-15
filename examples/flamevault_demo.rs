//! FlameVault Demonstration
//! 
//! Example usage of the FlameVault quantum-resistant encryption engine

use flamelang::{FlameVault, FlameVaultBlock};

fn main() {
    println!("=== FlameVault Quantum-Resistant Encryption Demo ===\n");

    // Create a new FlameVault instance with auto-generated keys
    let vault = FlameVault::new();
    println!("✓ Generated quantum-resistant keypair");
    println!("  - Kyber-1024 public key: {} bytes", vault.public_key().len());

    // Example plaintext
    let plaintext = b"Secret message: FlameLang quantum-resistant encryption!";
    println!("\n📝 Plaintext: {:?}", String::from_utf8_lossy(plaintext));
    println!("   Length: {} bytes", plaintext.len());

    // Create genesis block
    let genesis = FlameVaultBlock::genesis();
    let genesis_hash = genesis.hash();
    println!("\n🔗 Genesis block hash: {:02x?}...", &genesis_hash[..8]);

    // Encrypt the plaintext
    println!("\n🔐 Encrypting with polymorphic layer cascade...");
    let result = vault.encrypt(plaintext, genesis_hash).unwrap();
    
    println!("   ✓ Encryption complete!");
    println!("   - Ciphertext length: {} bytes", result.ciphertext.len());
    println!("   - Layer ordering: {:?}", result.layer_ordering);
    println!("   - Block timestamp: {}", result.block.timestamp);
    println!("   - DNA commitment: {:02x?}...", &result.block.dna_commitment[..8]);
    println!("   - Dilithium signature: {} bytes", result.block.dilithium_sig.len());

    // Calculate block hash
    let block_hash = result.block.hash();
    println!("\n🔗 Block hash: {:02x?}...", &block_hash[..8]);

    println!("\n✅ FlameVault demonstration complete!");
    println!("\n💡 Key Innovation: Polymorphic Layer Cascade Encryption");
    println!("   - 5 transformation layers applied in permuted order");
    println!("   - 5! = 120 possible layer orderings");
    println!("   - Attacker must guess both keys AND layer sequence");
    println!("   - Quantum-resistant (NIST PQC: Kyber + Dilithium)");
    println!("   - Biological entropy via DNA encoding (4^n keyspace)");
}
