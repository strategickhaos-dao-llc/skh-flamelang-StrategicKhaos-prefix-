//! Generate Pipefitter's Cortex outputs
//! 
//! This example demonstrates the full cortex system:
//! - 16x16 Synapse Matrix CSV
//! - Obsidian Canvas JSON
//! - Individual cortex nodes with all mappings

use flamelang::stdlib::pipefitter::*;
use std::fs;

fn main() {
    println!("🔥 Pipefitter's Cortex - Multidimensional Neural Synapse System");
    println!("{}", "=".repeat(70));
    
    // 1. Generate and save synapse matrix CSV
    println!("\n📊 Generating 16x16 Synapse Matrix...");
    let csv = export_synapse_matrix_csv();
    fs::write("synapse_matrix.csv", &csv).expect("Failed to write CSV");
    println!("✅ Saved to: synapse_matrix.csv");
    println!("   Matrix includes travel distances (Law of Cosines) and physics weights");
    
    // 2. Generate and save Obsidian Canvas JSON
    println!("\n🎨 Generating Obsidian Canvas JSON...");
    let json = export_obsidian_canvas_json();
    fs::write("cortex_canvas.json", &json).expect("Failed to write JSON");
    println!("✅ Saved to: cortex_canvas.json");
    println!("   Import into Obsidian for visual drag-drop layout");
    
    // 3. Display detailed cortex nodes
    println!("\n🧠 Cortex Node Details (Sample: ALG-001, ALG-008, ALG-016):");
    println!("{}", "=".repeat(70));
    
    let algorithms = Algorithm::default_algorithms();
    for (idx, alg) in algorithms.iter().enumerate() {
        if idx == 0 || idx == 7 || idx == 15 {
            let node = generate_cortex_node(alg);
            println!("\n{} - {} ({:?})", node.alg_id, alg.name, alg.physics_type);
            println!("  Vector Position: ({:.6}, {:.6})", node.position.0, node.position.1);
            println!("  Angle: {:.4}° ({}' offset)", node.angle_degrees, node.minute_offset);
            println!("  Chess Square: {} (Layer {})", node.chess_square, node.chess_layer);
            println!("  Rubik's: PLL={} OLL={}", node.rubik_pll, node.rubik_oll);
            println!("  Base64 Hash: {}", node.base64_hash);
            println!("  DNA Block: {}", node.dna_block);
            println!("  Periodic: {} (Z={}, p={}, n={}, e={})", 
                node.periodic_element.symbol,
                node.periodic_element.atomic_number,
                node.periodic_element.protons,
                node.periodic_element.neutrons,
                node.periodic_element.electrons
            );
            println!("  Docker: {}", node.docker_container);
            println!("  Collapse Probability: {:.6}", node.collapse_prob);
        }
    }
    
    // 4. Show 21,600 slot mapping concept
    println!("\n\n🌐 21,600 Slot Mapping (360° × 60' minutes)");
    println!("{}", "=".repeat(70));
    println!("  Full sovereignty grid with minute-level precision");
    println!("  Current 16 algorithms occupy slots 1-16");
    println!("  Scalable to 21,600 unique neurons/tools/servers");
    println!("  Each slot maps to:");
    println!("    - Unique angle position (vector)");
    println!("    - Chess square projection");
    println!("    - Rubik's algorithm (PLL/OLL)");
    println!("    - Base64 hash for storage");
    println!("    - DNA codon sequence");
    println!("    - Periodic element");
    println!("    - Docker container");
    println!("    - Quantum collapse probability");
    
    // 5. Show sample synapse connections
    println!("\n\n🔗 Sample Synapse Connections (High Weight > 1M):");
    println!("{}", "=".repeat(70));
    let connections = generate_synapse_matrix();
    let mut count = 0;
    for conn in &connections {
        if conn.weight > 1e6 && conn.from != conn.to && count < 10 {
            println!("  {} → {}: distance={:.6}, weight={:.2}, prob={:.6}",
                conn.from, conn.to, conn.travel_distance, conn.weight,
                compute_collapse_probability(conn.travel_distance)
            );
            count += 1;
        }
    }
    
    println!("\n\n✨ Cortex generation complete!");
    println!("🚀 Files generated:");
    println!("   - synapse_matrix.csv (paste into Excel/Obsidian)");
    println!("   - cortex_canvas.json (import into Obsidian Canvas)");
}
