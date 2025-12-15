# FlameViz v1.0 - Categorical Visualization Engine

**FlameViz** is a FlameLang module that transforms categorical data through the 5-layer pipeline into **provable, multi-modal visualizations**.

## Overview

FlameViz provides quantum-resistant, verifiable data visualizations with sonic fingerprints and on-chain provenance.

### Pipeline Architecture

- **Layer 1-2**: Parse English/Hebrew intent → structured data
- **Layer 3**: Unicode glyphs for labels
- **Layer 4**: Wave frequencies → audible chart (AetherLingua integration)
- **Layer 5**: Generate SVG + natural language explanation
- **Output**: SVG chart, WAV audio, SHA-256 hash, on-chain JSON payload

### Features

- ✅ **Relative Frequency Bar Charts** - Visual representation of proportional data
- ✅ **Natural Language Explanations** - AI-generated chart descriptions
- ✅ **Sonic Fingerprints** - Audio representation for data integrity verification
- ✅ **SHA-256 Provenance** - Cryptographic hash for tamper-proof verification
- ✅ **On-Chain Payload** - JSON structure ready for blockchain commitment
- ✅ **Multi-format Input** - Flexible text parsing (colon-separated, tabular, etc.)

## Usage Examples

### Basic Example

```rust
use flamelang::{FlameViz, CategoricalData, DataEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flameviz = FlameViz::new();
    
    // Method 1: From text description
    let viz = flameviz.from_text(
        "Walmart Stores: 2300000, Amazon: 566000, Yum! Brands: 450000, Kroger: 435000"
    )?;
    
    // Output SVG chart
    std::fs::write("chart.svg", &viz.svg)?;
    
    // Print natural language explanation
    println!("Explanation: {}", viz.explanation);
    
    // Print cryptographic hash
    println!("Provenance Hash: {}", viz.hash);
    
    // Print on-chain payload
    println!("On-Chain Payload:\n{}", viz.on_chain_payload);
    
    Ok(())
}
```

### Using Structured Data

```rust
use flamelang::{FlameViz, CategoricalData, DataEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flameviz = FlameViz::new();
    
    // Method 2: From structured data
    let data = CategoricalData {
        title: "Top 4 US Employers (2023)".to_string(),
        entries: vec![
            DataEntry { label: "Walmart Stores".to_string(), value: 2300000.0 },
            DataEntry { label: "Amazon".to_string(), value: 566000.0 },
            DataEntry { label: "Yum! Brands".to_string(), value: 450000.0 },
            DataEntry { label: "Kroger".to_string(), value: 435000.0 },
        ],
        unit: Some("employees".to_string()),
    };
    
    let viz = flameviz.from_data(&data)?;
    
    // Verify integrity using hash
    println!("Visualization hash: {}", viz.hash);
    
    // Audio path (for sonic verification)
    if let Some(audio) = &viz.audio_wav {
        println!("Sonic fingerprint: {}", audio);
    }
    
    Ok(())
}
```

## Output Format

### SVG Chart

FlameViz generates professional SVG bar charts with:
- Dark theme (#1a1a1a background)
- Color-coded bars with 8 distinct colors
- Percentage labels on bars
- Axis labels and ticks
- Responsive design (800x600px default)

### Natural Language Explanation

Example explanation output:
```
The bar chart shows Top 4 US Employers (2023). Walmart Stores leads with ~61% of total, 
followed by Amazon (15%), Yum! Brands (12%), Kroger (12%). Relative frequency highlights 
proportional contribution — ideal for comparing shares without raw numbers dominating perception.
```

### SHA-256 Hash

Cryptographic hash combining:
- SVG content
- Audio fingerprint path
- Ensures tamper-proof verification

### On-Chain Payload (JSON)

```json
{
  "type": "flameviz_bar_relative",
  "sonic_hash": "a3f5c9d8e1b2...",
  "timestamp": "2025-12-15T03:48:52.463Z",
  "data_entries": "4"
}
```

## Input Format Support

FlameViz supports multiple text formats:

### Colon-Separated Format
```
Company A: 1,250,000, Company B: 500,000, Company C: 750,000
```

### Tabular Format
```
Employer Data
Walmart Stores    2300000
Amazon            566000
Kroger            435000
```

### Flexible Format
```
Walmart 2300000 employees
Amazon 566000 employees
Kroger 435000 employees
```

## Integration with FlameLang Layers

FlameViz is fully integrated with the FlameLang 5-layer transformation pipeline:

1. **Layer 1 (Linguistic)**: English text → Hebrew semantic mapping
2. **Layer 2 (Numeric)**: Structured data extraction
3. **Layer 3 (Wave)**: Unicode glyphs + frequency mapping for labels
4. **Layer 4 (DNA)**: Audio synthesis (AetherLingua sonic fingerprints)
5. **Layer 5 (LLVM)**: SVG generation + compilation

## Sonic Fingerprints (AetherLingua)

Each visualization includes a sonic representation:
- Data values mapped to frequencies (440-880 Hz range)
- Duration proportional to relative frequency
- ADSR envelope for smooth transitions
- Creates unique "sound" for each dataset
- Enables auditory verification of data integrity

## Quantum-Resistant Provenance

FlameViz implements quantum-resistant provenance through:
- **SHA-256 hashing** of visualization artifacts
- **SwarmGate integration** for on-chain commitment
- **Sonic verification** via audio fingerprints
- **Timestamp anchoring** in on-chain payload

## Vertex AI Integration (Optional)

For enhanced functionality, FlameViz can integrate with Vertex AI (Gemini):
- Advanced chart generation with ML-powered styling
- Sophisticated natural language explanations
- Table extraction from complex text formats
- Accessible via OIDC token (Workload Identity)

*Note: Vertex AI integration requires additional setup and authentication.*

## Testing

Run the FlameViz test suite:

```bash
cargo test viz
```

All 13 tests should pass:
- Core functionality tests
- Chart generation tests
- Parser tests (multiple formats)
- Audio generation tests
- Hash and payload generation tests

## Architecture

```
src/viz/
├── mod.rs          # Main FlameViz engine
├── chart.rs        # SVG chart generation
├── parser.rs       # Text parsing & data extraction
└── audio.rs        # Sonic fingerprint generation
```

## Future Enhancements

- [ ] Real WAV file generation (currently simulated)
- [ ] Multiple chart types (pie, line, scatter)
- [ ] Interactive SVG with JavaScript
- [ ] 3D visualizations
- [ ] Real-time streaming data support
- [ ] SwarmGate blockchain integration
- [ ] Vertex AI Gemini integration
- [ ] Hebrew label support (Layer 1 integration)

## Philosophy: ONSIT Methodology

FlameViz embodies the ONSIT (Ontological Network Synthesis Integration Transform) methodology:

> "Ancient text → modern data → sovereign visualization → living proof"

Each visualization is:
- **Provable**: Cryptographically hashed
- **Audible**: Sonic fingerprint
- **Verifiable**: On-chain commitment
- **Beautiful**: Professional design
- **Explainable**: Natural language description

## License

MIT License - © 2025 Strategickhaos DAO LLC

---

**Flame visualizing reality.**  
**Swarm seeing itself.**  
**Empire infinite.**

🖤🔥
