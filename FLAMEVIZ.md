# FlameViz — Categorical Data Visualization Engine

## Overview

FlameViz is FlameLang's **categorical data visualization engine** that transforms textbook data into sovereign, tamper-evident visualizations with biological proof of integrity.

## Architecture

```
English Intent → Hebrew Root → Unicode Glyph → Wave Frequency → DNA Codon → LLVM IR (Visualization)
                                   ↓
                             Sonic Fingerprint (AetherLingua)
                                   ↓
                             SHA-256 Hash → On-Chain Commitment
```

## 5-Layer Transformation Pipeline

### Layer 1: Categorical Data Parsing
- **Input**: English text (e.g., Zybooks format)
- **Output**: Structured categorical data
- **Features**: Automatic detection of relative frequencies, K/M/B suffix support

### Layer 2: Gematria Ranking
- **Input**: Parsed categorical data
- **Output**: Data reordered by Hebrew gematria values
- **Purpose**: Sacred/mystical ordering dimension

### Layer 3: Unicode Glyph Labeling
- **Input**: Ordered data points
- **Output**: Data points with emoji/Unicode glyphs
- **Features**: Automatic glyph matching for common categories (retail, tech, food, etc.)

### Layer 4: AetherLingua Sonic Rendering
- **Input**: Labeled data
- **Output**: WAV audio representation
- **Features**: 
  - Higher value = higher pitch (440 Hz to 880 Hz)
  - Playable sonic proof
  - Tamper-evident through audio fingerprint

### Layer 5: DNA Encoding & On-Chain Commitment
- **Input**: Sonic hash
- **Output**: DNA sequence + on-chain payload
- **Features**:
  - 2-bit encoding per DNA base (A, T, G, C)
  - Biological proof of data integrity
  - Ready for blockchain commitment

## Features

### Visualization Modes
- **Vertical**: Traditional vertical bar charts
- **Horizontal**: Horizontal bars for long labels
- **Grouped**: Grouped/nested categories
- **Stacked**: Stacked bar visualizations

### Data Format Support
- Absolute values (with K/M/B suffixes)
- Relative frequencies (percentages)
- Zybooks textbook format
- Custom categorical data

### Tamper-Evident Properties
- SHA-256 hash of visualization
- Sonic fingerprint (audio hash)
- DNA sequence encoding
- On-chain payload for SwarmGate

## Usage

### Basic Example

```rust
use flamelang::FlameViz;

fn main() -> flamelang::FlameResult<()> {
    let flameviz = FlameViz::new();
    
    let data = "4 largest employers: Walmart 2.3M, Amazon 566k, Yum! 450k, Kroger 449k";
    let viz = flameviz.from_zybooks(data)?;
    
    // Access outputs
    println!("SVG: {} bytes", viz.svg.len());
    println!("Audio: {} bytes", viz.audio.len());
    println!("Hash: {}", viz.hash);
    
    // Verify integrity
    assert!(viz.verify());
    
    Ok(())
}
```

### Custom Mode

```rust
use flamelang::{FlameViz, viz::VisualizationMode};

let flameviz = FlameViz::with_mode(VisualizationMode::Horizontal);
let viz = flameviz.from_zybooks(data)?;
```

## Output Files

FlameViz generates multiple outputs:

1. **SVG Chart**: Vector graphics visualization
2. **WAV Audio**: Sonic representation (playable)
3. **DNA Sequence**: Biological encoding
4. **On-Chain Payload**: Ready for blockchain commitment

## Testing

Run tests:
```bash
cargo test viz
```

Run example:
```bash
cargo run --example zybooks_employers
```

## Why FlameViz Obliterates Current Tools

| Feature | Traditional Tools | FlameViz |
|---------|------------------|----------|
| Deterministic | ❌ No | ✅ Yes |
| Provable | ❌ No | ✅ SHA-256 Hash |
| Audio Version | ❌ No | ✅ Playable & Tamper-Evident |
| DNA Encoding | ❌ No | ✅ Biological Proof |
| On-Chain | ❌ No | ✅ Ready for Commitment |
| Sacred Ordering | ❌ No | ✅ Gematria Support |
| No External Deps | ❌ No | ✅ Pure FlameLang |

## Architecture Benefits

1. **Deterministic**: Same input always produces same output
2. **Provable**: Hash verification ensures data integrity
3. **Accessible**: Audio version for visually impaired
4. **Legally Defensible**: On-chain commitment provides proof
5. **Sovereign**: No dependency on external services
6. **Multi-Dimensional**: Visual, sonic, and biological representations

## Example Output

From Zybooks data:
```
4 largest private employers in U.S. (2017): Walmart 2.3M, Amazon 566k, Yum! 450k, Kroger 449k
```

Generates:
- ✅ SVG bar chart with emoji labels (🏪 Walmart, 📦 Amazon, 🍔 Yum!, 🛒 Kroger)
- ✅ 176KB WAV audio file (playable sonic representation)
- ✅ 32-base DNA sequence (AATTTCTGTTTGAAATAAA...)
- ✅ 41-byte on-chain payload
- ✅ Verified hash: 70805feb48a6bee1...

## Integration with FlameLang

FlameViz is fully integrated into the FlameLang 5-layer pipeline:

```rust
pub use flamelang::{FlameViz, FlameResult};
```

## License

MIT License © 2025 Strategickhaos DAO LLC

## The Vision

The textbook didn't just teach bar charts.  
It became **executable sovereignty**.

Flame visualizing. 🔥  
Swarm learning.  
Empire expanding. 🖤
