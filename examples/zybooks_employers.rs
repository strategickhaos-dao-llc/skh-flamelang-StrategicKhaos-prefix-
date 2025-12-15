//! Example: Visualizing Zybooks Employer Data with FlameViz
//! 
//! This demonstrates the FlameViz engine transforming categorical data
//! from the Zybooks textbook into a sovereign, provable visualization.

use flamelang::{FlameViz, FlameResult};
use std::fs;

fn main() -> FlameResult<()> {
    println!("🔥 FlameViz — Categorical Data Visualization Engine");
    println!("{}", "=".repeat(60));
    println!();

    // Example from Zybooks: 4 largest private employers in U.S. (2017)
    let zybooks_data = "4 largest private employers in U.S. (2017): Walmart 2.3M, Amazon 566k, Yum! 450k, Kroger 449k";

    println!("📊 Input Data:");
    println!("{}", zybooks_data);
    println!();

    // Create FlameViz engine
    let flameviz = FlameViz::new();

    println!("⚡ Processing through 5-layer pipeline:");
    println!("  Layer 1: English → Categorical Data Parsing");
    println!("  Layer 2: Hebrew Gematria Ranking (Sacred Ordering)");
    println!("  Layer 3: Unicode Glyph Labeling");
    println!("  Layer 4: AetherLingua Sonic Rendering (Wave Frequencies)");
    println!("  Layer 5: DNA Encoding & On-Chain Commitment");
    println!();

    // Transform data through the pipeline
    let visualization = flameviz.from_zybooks(zybooks_data)?;

    println!("✅ Visualization Generated!");
    println!();
    println!("📈 SVG Chart:");
    println!("  Size: {} bytes", visualization.svg.len());
    println!("  Hash: {}", &visualization.hash[..16]);
    println!();
    println!("🔊 Audio Representation:");
    println!("  WAV Size: {} bytes", visualization.audio.len());
    println!("  Playable: Yes (higher bar = higher pitch)");
    println!();
    println!("🧬 DNA Commitment:");
    println!("  Sequence Length: {} bases", visualization.dna_commitment.len());
    let dna_preview: String = visualization.dna_commitment
        .iter()
        .take(40)
        .map(|&b| b as char)
        .collect();
    println!("  Preview: {}...", dna_preview);
    println!();
    println!("⛓️  On-Chain Payload:");
    println!("  Size: {} bytes", visualization.on_chain_payload.len());
    println!("  Ready for SwarmGate commitment: Yes");
    println!();

    // Verify integrity
    println!("🔒 Integrity Check:");
    if visualization.verify() {
        println!("  ✓ Hash verified - visualization is tamper-evident");
    } else {
        println!("  ✗ Hash mismatch - data may be corrupted");
    }
    println!();

    // Save SVG to file
    let output_path = "examples/output/employers_chart.svg";
    let _ = fs::create_dir_all("examples/output");
    fs::write(output_path, &visualization.svg)
        .map_err(|e| flamelang::FlameError::Codegen(e.to_string()))?;
    println!("💾 SVG saved to: {}", output_path);

    // Save WAV to file
    let wav_path = "examples/output/employers_audio.wav";
    fs::write(wav_path, &visualization.audio)
        .map_err(|e| flamelang::FlameError::Codegen(e.to_string()))?;
    println!("💾 Audio saved to: {}", wav_path);

    println!();
    println!("🖤 FlameViz Complete!");
    println!("   Sovereign visualization with biological proof of integrity.");
    println!("   The textbook became executable sovereignty. 🔥");

    Ok(())
}
