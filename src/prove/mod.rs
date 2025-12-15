//! Prove module - Generate cryptographic proof and on-chain attestation
//! Creates verifiable provenance for sovereign HTML artifacts

use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VesselProof {
    /// Schema version
    pub version: String,
    
    /// Type identifier
    #[serde(rename = "type")]
    pub proof_type: String,
    
    /// SHA-256 hash of content
    pub content_hash: String,
    
    /// SHA-256 hash of structure only (DOM without styles/scripts)
    pub structure_hash: String,
    
    /// File size in bytes
    pub size_bytes: usize,
    
    /// Timestamp (Unix epoch)
    pub timestamp: u64,
    
    /// Target blockchain
    pub chain: String,
    
    /// Generator info
    pub generator: GeneratorInfo,
    
    /// Optional on-chain transaction ID (populated after submission)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    
    /// 7% charity trigger (Strategickhaos signature)
    pub charity_trigger: bool,
    
    /// Node 137 resonance marker
    pub node_137: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorInfo {
    pub name: String,
    pub version: String,
    pub entity: String,
    pub ein: String,
}

pub fn run(input: &Path, chain: &str, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input)?;
    
    // Compute content hash
    let mut content_hasher = Sha256::new();
    content_hasher.update(content.as_bytes());
    let content_hash = hex::encode(content_hasher.finalize());
    
    // Compute structure hash (simplified - just strip scripts/styles)
    let structure_only = strip_to_structure(&content);
    let mut structure_hasher = Sha256::new();
    structure_hasher.update(structure_only.as_bytes());
    let structure_hash = hex::encode(structure_hasher.finalize());
    
    // Check for 7% charity trigger
    let charity_trigger = content.contains("7%") || 
                          content.contains("charity") || 
                          content.contains("ValorYield");
    
    // Check for node 137 marker
    let node_137 = content.contains("137") || 
                   content.contains("Node137") ||
                   content.contains("prime");
    
    let proof = VesselProof {
        version: "1.0.0".to_string(),
        proof_type: "vesselmirror_sovereign".to_string(),
        content_hash,
        structure_hash,
        size_bytes: content.len(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "System clock is set before Unix epoch")?
            .as_secs(),
        chain: chain.to_string(),
        generator: GeneratorInfo {
            name: "VesselMirror".to_string(),
            version: "0.1.0".to_string(),
            entity: "Strategickhaos DAO LLC".to_string(),
            ein: "39-2900295".to_string(),
        },
        tx_id: None,
        charity_trigger,
        node_137,
    };
    
    println!("  ├─ Content hash: {}", &proof.content_hash[..16]);
    println!("  ├─ Structure hash: {}", &proof.structure_hash[..16]);
    println!("  ├─ Chain: {}", chain);
    println!("  ├─ Charity trigger: {}", charity_trigger);
    println!("  ├─ Node 137: {}", node_137);
    
    // Write proof JSON
    let proof_json = serde_json::to_string_pretty(&proof)?;
    fs::write(output, &proof_json)?;
    println!("  └─ ✅ Proof: {:?}", output);
    
    // Generate on-chain payload based on target
    match chain.to_lowercase().as_str() {
        "swarmgate" => generate_swarmgate_payload(&proof, output)?,
        "ethereum" | "eth" => generate_ethereum_payload(&proof, output)?,
        "solana" | "sol" => generate_solana_payload(&proof, output)?,
        _ => println!("  └─ ⚠️ Unknown chain: {}, proof saved locally only", chain),
    }
    
    Ok(())
}

fn strip_to_structure(html: &str) -> String {
    let mut result = html.to_string();
    
    // Remove script tags and content
    // Look for <script (with space or >) to ensure proper tag boundaries
    while let Some(start) = result.find("<script") {
        // Check if it's a proper tag by ensuring next char is space or >
        if start + 7 < result.len() {
            let next_char = result.chars().nth(start + 7).unwrap_or('\0');
            if next_char == ' ' || next_char == '>' {
                if let Some(end_pos) = result[start..].find("</script>") {
                    result = format!("{}{}", &result[..start], &result[start + end_pos + 9..]);
                    continue;
                }
            }
        }
        break;
    }
    
    // Remove style tags and content
    // Look for <style (with space or >) to ensure proper tag boundaries
    while let Some(start) = result.find("<style") {
        // Check if it's a proper tag by ensuring next char is space or >
        if start + 6 < result.len() {
            let next_char = result.chars().nth(start + 6).unwrap_or('\0');
            if next_char == ' ' || next_char == '>' {
                if let Some(end_pos) = result[start..].find("</style>") {
                    result = format!("{}{}", &result[..start], &result[start + end_pos + 8..]);
                    continue;
                }
            }
        }
        break;
    }
    
    // Normalize whitespace for structural comparison
    result = result.split_whitespace().collect::<Vec<_>>().join(" ");
    
    result
}

fn generate_swarmgate_payload(proof: &VesselProof, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let payload = serde_json::json!({
        "protocol": "swarmgate_v1",
        "action": "attest",
        "data": {
            "type": proof.proof_type,
            "hash": proof.content_hash,
            "structure_hash": proof.structure_hash,
            "timestamp": proof.timestamp,
            "charity_trigger": proof.charity_trigger,
            "node_137": proof.node_137,
        },
        "signature": "PENDING_GPG_SIGN",
        "entity": {
            "name": proof.generator.entity,
            "ein": proof.generator.ein,
        }
    });
    
    let payload_path = output.with_extension("swarmgate.json");
    fs::write(&payload_path, serde_json::to_string_pretty(&payload)?)?;
    println!("  └─ SwarmGate payload: {:?}", payload_path);
    Ok(())
}

fn generate_ethereum_payload(proof: &VesselProof, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // EIP-712 typed data structure for Ethereum attestation
    let payload = serde_json::json!({
        "types": {
            "EIP712Domain": [
                {"name": "name", "type": "string"},
                {"name": "version", "type": "string"},
                {"name": "chainId", "type": "uint256"}
            ],
            "VesselAttestation": [
                {"name": "contentHash", "type": "bytes32"},
                {"name": "structureHash", "type": "bytes32"},
                {"name": "timestamp", "type": "uint256"},
                {"name": "generator", "type": "string"}
            ]
        },
        "primaryType": "VesselAttestation",
        "domain": {
            "name": "VesselMirror",
            "version": "1",
            "chainId": 1
        },
        "message": {
            "contentHash": format!("0x{}", proof.content_hash),
            "structureHash": format!("0x{}", proof.structure_hash),
            "timestamp": proof.timestamp,
            "generator": proof.generator.entity
        }
    });
    
    let payload_path = output.with_extension("eth.json");
    fs::write(&payload_path, serde_json::to_string_pretty(&payload)?)?;
    println!("  └─ Ethereum payload: {:?}", payload_path);
    Ok(())
}

fn generate_solana_payload(proof: &VesselProof, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let payload = serde_json::json!({
        "program": "VesselMirrorAttestation",
        "instruction": "attest",
        "accounts": [],
        "data": {
            "content_hash": proof.content_hash,
            "structure_hash": proof.structure_hash,
            "timestamp": proof.timestamp,
            "charity_trigger": proof.charity_trigger,
        }
    });
    
    let payload_path = output.with_extension("sol.json");
    fs::write(&payload_path, serde_json::to_string_pretty(&payload)?)?;
    println!("  └─ Solana payload: {:?}", payload_path);
    Ok(())
}
