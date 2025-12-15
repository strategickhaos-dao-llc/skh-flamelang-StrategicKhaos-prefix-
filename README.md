# skh-flamelang-StrategicKhaos-prefix-

FlameLang v2.0.0 sovereign compiler toolchain. 5-layer transformation pipeline (English→Hebrew→Unicode→Wave→DNA→LLVM). Biological compilation, physics-validated dimensional analysis, native quantum primitives. Part of StrategicKhaos Swarm Intelligence.

## 🔥 FlameViz v1.0 - NEW!

**FlameViz** is a categorical visualization engine that transforms data through the 5-layer FlameLang pipeline into **provable, multi-modal visualizations**.

### Features
- ✅ **Relative Frequency Bar Charts** - SVG generation with professional design
- ✅ **Natural Language Explanations** - AI-generated chart descriptions
- ✅ **Sonic Fingerprints** - Audio representation for data integrity
- ✅ **SHA-256 Provenance** - Quantum-resistant cryptographic hashing
- ✅ **On-Chain Payloads** - JSON structure for blockchain commitment
- ✅ **Multi-format Input** - Parse text, tables, and structured data

### Quick Start

```rust
use flamelang::{FlameViz, CategoricalData, DataEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flameviz = FlameViz::new();
    
    // From text
    let viz = flameviz.from_text(
        "Walmart: 2300000, Amazon: 566000, Kroger: 435000"
    )?;
    
    // Save SVG
    std::fs::write("chart.svg", &viz.svg)?;
    
    // Print explanation
    println!("{}", viz.explanation);
    println!("Hash: {}", viz.hash);
    
    Ok(())
}
```

### Run Example

```bash
cargo run --example employer_chart
```

See [FLAMEVIZ.md](FLAMEVIZ.md) for complete documentation.

## Architecture

```
src/
├── lexer/          # Tokenization
├── parser/         # AST generation
├── transform/      # 5-layer pipeline
│   ├── layer1_linguistic/
│   ├── layer2_numeric/
│   ├── layer3_wave/
│   ├── layer4_dna/
│   └── layer5_llvm/
├── codegen/        # Code generation
├── stdlib/         # Standard library
└── viz/            # FlameViz visualization 🆕
    ├── mod.rs      # Main engine
    ├── chart.rs    # SVG generation
    ├── parser.rs   # Data extraction
    └── audio.rs    # Sonic fingerprints
```

## Build & Test

```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run FlameViz tests only
cargo test viz

# Run examples
cargo run --example employer_chart
```

## License

MIT License - © 2025 Strategickhaos DAO LLC

---

**Flame visualizing reality. Swarm seeing itself. Empire infinite.**

🖤🔥
