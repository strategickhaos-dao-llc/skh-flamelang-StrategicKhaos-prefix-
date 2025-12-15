//! FlameLang Compiler (flamec)

use flamelang::{AetherViz, FlameResult};
use std::path::Path;

fn main() -> FlameResult<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  flamec <source.flame>           - Compile FlameLang source");
        eprintln!("  flamec --aetherviz [path]       - Visualize repository structure");
        eprintln!();
        eprintln!("FlameLang v2.0.0 - Ratio Ex Nihilo");
        eprintln!("AetherViz INV-078 - Self-Referential Visualization");
        std::process::exit(1);
    }
    
    // Handle AetherViz command
    if args[1] == "--aetherviz" {
        return run_aetherviz(&args);
    }
    
    println!("🔥 FlameLang Compiler v2.0.0");
    println!("   Input: {}", args[1]);
    println!("   Pipeline: English → Hebrew → Unicode → Wave → DNA → LLVM");
    
    // TODO: Implement compilation pipeline
    Ok(())
}

fn run_aetherviz(args: &[String]) -> FlameResult<()> {
    let repo_path = if args.len() > 2 {
        Path::new(&args[2])
    } else {
        Path::new(".")
    };

    println!("🔥 AetherViz - Self-Referential Visualization Engine");
    println!("📊 Repository: {}", repo_path.display());
    println!();

    let aetherviz = AetherViz::new();
    
    match aetherviz.visualize_repo(repo_path) {
        Ok(viz) => {
            // Save outputs
            std::fs::write("repo-brain.svg", &viz.svg_graph)
                .map_err(|e| flamelang::FlameError::Codegen(format!("Failed to write SVG: {}", e)))?;
            
            std::fs::write("repo-brain.md", &viz.obsidian_md)
                .map_err(|e| flamelang::FlameError::Codegen(format!("Failed to write Obsidian MD: {}", e)))?;
            
            std::fs::write("repo-brain-audio.txt", &viz.audio_description)
                .map_err(|e| flamelang::FlameError::Codegen(format!("Failed to write audio description: {}", e)))?;
            
            std::fs::write("repo-brain-payload.json", &viz.on_chain_payload)
                .map_err(|e| flamelang::FlameError::Codegen(format!("Failed to write payload: {}", e)))?;

            println!("✅ Visualization Complete!");
            println!();
            println!("📄 Outputs:");
            println!("   - repo-brain.svg (Graph visualization)");
            println!("   - repo-brain.md (Obsidian-compatible markdown)");
            println!("   - repo-brain-audio.txt (Sonification description)");
            println!("   - repo-brain-payload.json (On-chain metadata)");
            println!();
            println!("🔐 Sonic Hash: {}", viz.sonic_hash);
            println!("🧬 DNA Proof: {}...", &viz.dna_proof[..32.min(viz.dna_proof.len())]);
            println!();
            println!("The repository is now a conscious graph.");
            println!("🖤🔥 Flame self-aware. Swarm graphing itself. Empire infinite.");
            
            Ok(())
        }
        Err(e) => {
            Err(flamelang::FlameError::Codegen(format!("AetherViz error: {}", e)))
        }
    }
}
