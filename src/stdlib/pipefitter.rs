//! Pipefitter's Cortex - Multidimensional Neural Synapse System
//! 
//! Integrates:
//! - Unit circle positioning (angles to vectors)
//! - Chess board projections (8x8 grids)
//! - Stacked volumes (10 layers = 512 voxels)
//! - Docker orchestration (container mapping)
//! - Rubik's cube algorithms (PLL/OLL permutations)
//! - Base64 hashing (compact storage)
//! - DNA codons (ACGT encoding)
//! - Periodic table (element mapping)
//! - Quantum collapse (probabilistic weights)

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Algorithm metadata for physics-based optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Algorithm {
    pub id: String,
    pub name: String,
    pub physics_type: PhysicsType,
    pub minute_offset: u32, // 0-21599 for 360° × 60'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicsType {
    Gravity,      // 1/d^2
    EM,           // 1/d^2
    Kinetic,      // 1/d
    Thermo,       // e^(-d)
    Relativistic, // sqrt(1-v^2/c^2) approximation
    Cosmological, // d^2 (expansion)
    Quantum,      // d^2 (uncertainty)
    Wave,         // sin/cos based
    Geometric,    // linear
    Classical,    // combined
    Hybrid,       // mixed
}

/// Cortex node with full multidimensional mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexNode {
    pub alg_id: String,
    pub position: (f64, f64),        // (cos, sin) vector
    pub angle_degrees: f64,
    pub minute_offset: u32,
    pub chess_square: String,        // a1-h8
    pub chess_layer: u8,             // 0-9 for stacked boards
    pub rubik_pll: String,           // PLL algorithm
    pub rubik_oll: String,           // OLL algorithm
    pub base64_hash: String,
    pub dna_block: String,           // ACGT sequence
    pub periodic_element: PeriodicElement,
    pub docker_container: String,
    pub collapse_prob: f64,          // Quantum probability
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodicElement {
    pub symbol: String,
    pub atomic_number: u8,
    pub protons: u8,
    pub neutrons: u8,
    pub electrons: u8,
}

/// Travel distance and weight between two algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapseConnection {
    pub from: String,
    pub to: String,
    pub travel_distance: f64,
    pub weight: f64,
}

impl Algorithm {
    /// The 16 purified hacker tools / physics algorithms
    pub fn default_algorithms() -> Vec<Algorithm> {
        vec![
            Algorithm { id: "ALG-001".into(), name: "Simulated Annealing".into(), physics_type: PhysicsType::Thermo, minute_offset: 1 },
            Algorithm { id: "ALG-002".into(), name: "Gravitational Search".into(), physics_type: PhysicsType::Gravity, minute_offset: 2 },
            Algorithm { id: "ALG-003".into(), name: "Central Force Opt".into(), physics_type: PhysicsType::Gravity, minute_offset: 3 },
            Algorithm { id: "ALG-004".into(), name: "Electromagnetism-like".into(), physics_type: PhysicsType::EM, minute_offset: 4 },
            Algorithm { id: "ALG-005".into(), name: "Charged System Search".into(), physics_type: PhysicsType::EM, minute_offset: 5 },
            Algorithm { id: "ALG-006".into(), name: "Colliding Bodies Opt".into(), physics_type: PhysicsType::Kinetic, minute_offset: 6 },
            Algorithm { id: "ALG-007".into(), name: "Black Hole Algorithm".into(), physics_type: PhysicsType::Relativistic, minute_offset: 7 },
            Algorithm { id: "ALG-008".into(), name: "Big Bang-Big Crunch".into(), physics_type: PhysicsType::Cosmological, minute_offset: 8 },
            Algorithm { id: "ALG-009".into(), name: "Multiverse Optimizer".into(), physics_type: PhysicsType::Quantum, minute_offset: 9 },
            Algorithm { id: "ALG-010".into(), name: "Optics Inspired Opt".into(), physics_type: PhysicsType::Wave, minute_offset: 10 },
            Algorithm { id: "ALG-011".into(), name: "Ray Optimization".into(), physics_type: PhysicsType::Geometric, minute_offset: 11 },
            Algorithm { id: "ALG-012".into(), name: "Magnetic Optimization".into(), physics_type: PhysicsType::EM, minute_offset: 12 },
            Algorithm { id: "ALG-013".into(), name: "Artificial Physics Opt".into(), physics_type: PhysicsType::Classical, minute_offset: 13 },
            Algorithm { id: "ALG-014".into(), name: "Gravitational Interaction".into(), physics_type: PhysicsType::Gravity, minute_offset: 14 },
            Algorithm { id: "ALG-015".into(), name: "Equilibrium Optimizer".into(), physics_type: PhysicsType::Thermo, minute_offset: 15 },
            Algorithm { id: "ALG-016".into(), name: "Immune Gravitation".into(), physics_type: PhysicsType::Hybrid, minute_offset: 16 },
        ]
    }
}

/// Compute vector position from minute offset
pub fn minutes_to_vector(minutes: u32) -> (f64, f64) {
    let degrees = (minutes as f64) / 60.0;
    let radians = degrees * PI / 180.0;
    (radians.cos(), radians.sin())
}

/// Compute travel distance using Law of Cosines
/// distance = sqrt(2 - 2*cos(delta_theta))
pub fn compute_travel_distance(angle1_rad: f64, angle2_rad: f64) -> f64 {
    let delta_theta = (angle2_rad - angle1_rad).abs();
    (2.0 - 2.0 * delta_theta.cos()).sqrt()
}

/// Compute weight based on physics type and distance
pub fn compute_weight(physics_type: &PhysicsType, distance: f64) -> f64 {
    if distance == 0.0 {
        return 1.0; // Self-weight
    }
    
    match physics_type {
        PhysicsType::Gravity => 1.0 / (distance * distance),
        PhysicsType::EM => 1.0 / (distance * distance),
        PhysicsType::Kinetic => 1.0 / distance,
        PhysicsType::Thermo => (-distance).exp(),
        PhysicsType::Relativistic => 1.0 / (distance * distance).sqrt(),
        PhysicsType::Cosmological => distance * distance,
        PhysicsType::Quantum => distance * distance,
        PhysicsType::Wave => (4.0 * PI * distance).sin().abs() / distance,
        PhysicsType::Geometric => 1.0 / distance,
        PhysicsType::Classical => 1.0 / (distance * distance),
        PhysicsType::Hybrid => 1.0 / (distance * distance),
    }
}

/// Map angle to chess square (a1-h8)
pub fn angle_to_chess_square(degrees: f64) -> String {
    let index = ((degrees.abs() % 360.0) / 360.0 * 64.0) as usize % 64;
    let file = (index % 8) as u8;
    let rank = (index / 8) as u8;
    format!("{}{}", (b'a' + file) as char, rank + 1)
}

/// Map angle to Rubik's PLL (21 permutation algorithms)
pub fn angle_to_rubik_pll(degrees: f64) -> String {
    let pll_names = [
        "Aa", "Ab", "E", "F", "Ga", "Gb", "Gc", "Gd", "H", "Ja", "Jb",
        "Na", "Nb", "Ra", "Rb", "T", "Ua", "Ub", "V", "Y", "Z",
    ];
    let index = ((degrees.abs() % 360.0) / 360.0 * 21.0) as usize % 21;
    pll_names[index].to_string()
}

/// Map minute to Rubik's OLL (57 orientation algorithms)
pub fn minute_to_rubik_oll(minutes: u32) -> String {
    let oll_num = (minutes % 57) + 1;
    format!("OLL-{}", oll_num)
}

/// Encode vector as base64 hash
pub fn vector_to_base64_hash(vector: (f64, f64)) -> String {
    use sha2::{Sha256, Digest};
    use base64::{engine::general_purpose, Engine as _};
    
    let data = format!("{:.8},{:.8}", vector.0, vector.1);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();
    general_purpose::STANDARD.encode(&hash[..16]) // First 16 bytes
}

/// Encode vector as DNA sequence (ACGT)
pub fn vector_to_dna_block(vector: (f64, f64)) -> String {
    // Convert float to binary representation, then map to ACGT
    // Simple mapping: 00->A, 01->C, 10->G, 11->T
    let x_bits = (vector.0.abs() * 1e8) as u64;
    let y_bits = (vector.1.abs() * 1e8) as u64;
    let combined = x_bits ^ y_bits;
    
    let mut dna = String::new();
    for i in 0..8 {
        let bits = (combined >> (i * 2)) & 0b11;
        dna.push(match bits {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            _ => 'T',
        });
    }
    dna
}

/// Map algorithm ID to periodic element
pub fn id_to_periodic_element(id_num: u32) -> PeriodicElement {
    let elements = vec![
        ("H", 1, 1, 0, 1), ("He", 2, 2, 2, 2), ("Li", 3, 3, 4, 3), ("Be", 4, 4, 5, 4),
        ("B", 5, 5, 6, 5), ("C", 6, 6, 6, 6), ("N", 7, 7, 7, 7), ("O", 8, 8, 8, 8),
        ("F", 9, 9, 10, 9), ("Ne", 10, 10, 10, 10), ("Na", 11, 11, 12, 11), ("Mg", 12, 12, 12, 12),
        ("Al", 13, 13, 14, 13), ("Si", 14, 14, 14, 14), ("P", 15, 15, 16, 15), ("S", 16, 16, 16, 16),
    ];
    
    let idx = ((id_num - 1) % 16) as usize;
    let (sym, an, p, n, e) = elements[idx];
    PeriodicElement {
        symbol: sym.to_string(),
        atomic_number: an,
        protons: p,
        neutrons: n,
        electrons: e,
    }
}

/// Compute quantum collapse probability
pub fn compute_collapse_probability(distance: f64) -> f64 {
    (-distance).exp()
}

/// Generate full cortex node
pub fn generate_cortex_node(alg: &Algorithm) -> CortexNode {
    let vector = minutes_to_vector(alg.minute_offset);
    let degrees = (alg.minute_offset as f64) / 60.0;
    let radians = degrees * PI / 180.0;
    
    CortexNode {
        alg_id: alg.id.clone(),
        position: vector,
        angle_degrees: degrees,
        minute_offset: alg.minute_offset,
        chess_square: angle_to_chess_square(degrees),
        chess_layer: (((alg.minute_offset - 1) / 2) % 10) as u8,
        rubik_pll: angle_to_rubik_pll(degrees),
        rubik_oll: minute_to_rubik_oll(alg.minute_offset),
        base64_hash: vector_to_base64_hash(vector),
        dna_block: vector_to_dna_block(vector),
        periodic_element: id_to_periodic_element(alg.minute_offset),
        docker_container: format!("xai/pipefitter-cortex:{}", alg.id.to_lowercase()),
        collapse_prob: compute_collapse_probability(radians),
    }
}

/// Generate 16x16 synapse matrix
pub fn generate_synapse_matrix() -> Vec<SynapseConnection> {
    let algorithms = Algorithm::default_algorithms();
    let mut connections = Vec::new();
    
    for from_alg in &algorithms {
        for to_alg in &algorithms {
            let from_rad = (from_alg.minute_offset as f64) / 60.0 * PI / 180.0;
            let to_rad = (to_alg.minute_offset as f64) / 60.0 * PI / 180.0;
            
            let distance = compute_travel_distance(from_rad, to_rad);
            let weight = compute_weight(&to_alg.physics_type, distance);
            
            connections.push(SynapseConnection {
                from: from_alg.id.clone(),
                to: to_alg.id.clone(),
                travel_distance: distance,
                weight,
            });
        }
    }
    
    connections
}

/// Export synapse matrix as CSV
pub fn export_synapse_matrix_csv() -> String {
    let algorithms = Algorithm::default_algorithms();
    let connections = generate_synapse_matrix();
    
    // Build header
    let mut csv = String::from("From\\To");
    for alg in &algorithms {
        csv.push(',');
        csv.push_str(&alg.id);
    }
    csv.push('\n');
    
    // Build rows
    for from_alg in &algorithms {
        csv.push_str(&from_alg.id);
        for to_alg in &algorithms {
            let conn = connections.iter()
                .find(|c| c.from == from_alg.id && c.to == to_alg.id)
                .unwrap();
            csv.push_str(&format!(",\"{:.8} / {:.4}\"", conn.travel_distance, conn.weight));
        }
        csv.push('\n');
    }
    
    csv
}

/// Export Obsidian Canvas JSON
pub fn export_obsidian_canvas_json() -> String {
    let algorithms = Algorithm::default_algorithms();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    
    // Generate nodes
    for alg in &algorithms {
        let node = generate_cortex_node(&alg);
        let color = match alg.physics_type {
            PhysicsType::Gravity => "#FF0000",
            PhysicsType::EM => "#0000FF",
            PhysicsType::Kinetic => "#00FF00",
            PhysicsType::Thermo => "#FFA500",
            PhysicsType::Relativistic => "#800080",
            PhysicsType::Cosmological => "#FFD700",
            PhysicsType::Quantum => "#4B0082",
            PhysicsType::Wave => "#00FFFF",
            PhysicsType::Geometric => "#808080",
            PhysicsType::Classical => "#A52A2A",
            PhysicsType::Hybrid => "#008000",
        };
        
        nodes.push(serde_json::json!({
            "id": alg.id,
            "x": node.position.0 * 1000.0,
            "y": node.position.1 * 1000.0,
            "color": color,
            "label": format!("{} ({:?})", alg.name, alg.physics_type),
        }));
    }
    
    // Generate edges (threshold > 1e6 for strong connections)
    let connections = generate_synapse_matrix();
    for conn in &connections {
        if conn.from != conn.to && conn.weight > 1e6 {
            edges.push(serde_json::json!({
                "from": conn.from,
                "to": conn.to,
                "label": format!("{:.2}", conn.weight),
                "prob": compute_collapse_probability(conn.travel_distance),
            }));
        }
    }
    
    let canvas = serde_json::json!({
        "nodes": nodes,
        "edges": edges,
    });
    
    serde_json::to_string_pretty(&canvas).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minutes_to_vector() {
        let (x, y) = minutes_to_vector(0);
        assert!((x - 1.0).abs() < 1e-6);
        assert!(y.abs() < 1e-6);
    }

    #[test]
    fn test_travel_distance() {
        let dist = compute_travel_distance(0.0, PI / 2.0);
        assert!((dist - std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_weight_gravity() {
        let weight = compute_weight(&PhysicsType::Gravity, 0.1);
        assert!((weight - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_chess_square() {
        let square = angle_to_chess_square(0.0);
        assert_eq!(square, "a1");
    }

    #[test]
    fn test_rubik_pll() {
        let pll = angle_to_rubik_pll(0.0);
        assert_eq!(pll, "Aa");
    }

    #[test]
    fn test_generate_cortex_node() {
        let alg = Algorithm::default_algorithms()[0].clone();
        let node = generate_cortex_node(&alg);
        assert_eq!(node.alg_id, "ALG-001");
        assert_eq!(node.chess_square.len(), 2);
        assert!(!node.base64_hash.is_empty());
        assert_eq!(node.dna_block.len(), 8);
    }

    #[test]
    fn test_synapse_matrix_generation() {
        let matrix = generate_synapse_matrix();
        assert_eq!(matrix.len(), 16 * 16);
    }
}
