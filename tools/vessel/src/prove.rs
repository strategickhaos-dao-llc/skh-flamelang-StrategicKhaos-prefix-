//! Stage 6: PROVE - SHA-256 hash + on-chain attestation

use crate::types::{ChainAttestation, IntegrityProof};
use anyhow::{Context, Result};
use colored::*;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Prove integrity with SHA-256 hash and optional chain attestation
pub fn prove_integrity(
    file: &PathBuf,
    chain: &str,
    verbose: bool,
) -> Result<()> {
    println!("{}", "  Stage 6: PROVE".bright_yellow().bold());
    
    // Read file
    let content = std::fs::read(file)
        .context("Failed to read file")?;
    
    // Calculate SHA-256 hash
    let hash = calculate_sha256(&content);
    
    println!("  {} SHA-256 hash calculated", "✓".bright_green());
    
    if verbose {
        println!("     Hash: {}", hash.bright_yellow());
    }
    
    // Create proof
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let chain_attestation = if chain != "none" {
        Some(attest_on_chain(&hash, chain, verbose)?)
    } else {
        None
    };
    
    let proof = IntegrityProof {
        hash: hash.clone(),
        algorithm: "SHA-256".to_string(),
        file_path: file.display().to_string(),
        timestamp,
        chain_attestation,
    };
    
    // Save proof to JSON
    let proof_path = file.with_extension("proof.json");
    let proof_json = serde_json::to_string_pretty(&proof)?;
    std::fs::write(&proof_path, proof_json)?;
    
    println!("  {} Proof saved to: {}", "✓".bright_green(), proof_path.display());
    
    // Display proof
    print_proof(&proof, verbose);
    
    Ok(())
}

fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn attest_on_chain(hash: &str, chain: &str, _verbose: bool) -> Result<ChainAttestation> {
    println!("  {} Attesting to {} blockchain...", "→".bright_blue(), chain);
    
    // Note: Blockchain attestation is not yet implemented
    // This is a placeholder that returns mock data for demonstration
    println!("     {} NOTICE: Blockchain attestation not implemented", "⚠".bright_yellow().bold());
    println!("     {} This is MOCK data for demonstration only", "⚠".bright_yellow().bold());
    println!("     {} Real attestation requires Web3 integration", "ℹ".bright_cyan());
    
    match chain {
        "swarmgate" => {
            Ok(ChainAttestation {
                chain: "swarmgate [MOCK]".to_string(),
                transaction_hash: format!("0x{} [MOCK]", &hash[..40]),
                block_number: 1234567,
            })
        }
        "ethereum" => {
            Ok(ChainAttestation {
                chain: "ethereum [MOCK]".to_string(),
                transaction_hash: format!("0x{} [MOCK]", &hash[..40]),
                block_number: 19000000,
            })
        }
        _ => {
            anyhow::bail!("Unsupported blockchain: {}. Use 'swarmgate', 'ethereum', or 'none'", chain)
        }
    }
}

fn print_proof(proof: &IntegrityProof, verbose: bool) {
    println!("\n  🔐 {}", "Integrity Proof".bright_cyan().bold());
    println!("     Algorithm: {}", proof.algorithm);
    println!("     Hash: {}", proof.hash.bright_yellow());
    println!("     File: {}", proof.file_path);
    
    if verbose {
        println!("     Timestamp: {}", proof.timestamp);
    }
    
    if let Some(attestation) = &proof.chain_attestation {
        println!("\n  ⛓️  {}", "Chain Attestation".bright_cyan().bold());
        println!("     Chain: {}", attestation.chain);
        println!("     TX: {}", attestation.transaction_hash.bright_yellow());
        println!("     Block: {}", attestation.block_number);
    }
    
    println!("\n  {}", "═══════════════════════════════════════".bright_blue());
    println!("  {}", "PROOF OF SOVEREIGN DOCUMENT".bright_blue().bold());
    println!("  {}", "This hash cryptographically proves the".bright_blue());
    println!("  {}", "integrity and authenticity of the document.".bright_blue());
    println!("  {}", "═══════════════════════════════════════".bright_blue());
}

/// Verify a proof against a file
pub fn verify_proof(file: &PathBuf, proof_file: &PathBuf) -> Result<bool> {
    let content = std::fs::read(file)
        .context("Failed to read file")?;
    
    let proof_json = std::fs::read_to_string(proof_file)
        .context("Failed to read proof file")?;
    
    let proof: IntegrityProof = serde_json::from_str(&proof_json)
        .context("Failed to parse proof")?;
    
    let current_hash = calculate_sha256(&content);
    
    Ok(current_hash == proof.hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sha256() {
        let data = b"Hello, VesselMirror!";
        let hash = calculate_sha256(data);
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
    }
}
