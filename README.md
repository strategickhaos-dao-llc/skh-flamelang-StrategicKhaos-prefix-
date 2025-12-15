# 🔥 FlameLang v2.0.0

> **Ratio Ex Nihilo** - Creating order from contradiction

FlameLang is a sovereign compiler toolchain implementing a revolutionary 5-layer transformation pipeline that bridges linguistic, biological, and quantum computing paradigms.

## Quick Start

```bash
# Build the project
cargo build --workspace

# Run the compiler
./target/debug/flamec your_program.flame

# Format your code
./target/debug/flamefmt your_program.flame

# Start the language server
./target/debug/flamelsp
```

## The 5-Layer Pipeline

```
English → Hebrew → Unicode → Wave → DNA → LLVM IR
   ↓         ↓         ↓        ↓      ↓       ↓
Linguistic Symbolic  Numeric  Sonic  Bio   Executable
```

Each layer transforms code through a unique domain:
1. **Linguistic**: English → Hebrew symbolic representation
2. **Numeric**: Unicode → Decimal values
3. **Wave**: Decimal → Frequency (Hz) using c=2πr
4. **DNA**: Frequency → Genetic codons (ATGC)
5. **LLVM**: Codons → Machine code via LLVM IR

## Features

- ✅ **Quantum-ready primitives**: Native qubit declarations and quantum gates (H, X, Y, Z, CNOT)
- ✅ **Biological compilation**: DNA sequence encoding and processing
- ✅ **Wave-based operations**: Trigonometric wave cores (sin~, cos~, tan~)
- ✅ **Bell state entanglement**: Built-in quantum entanglement operations
- ✅ **Swarm intelligence hooks**: Integration with StrategicKhaos Swarm
- ✅ **Neural tick clocks**: @tick for time-based neural operations
- ✅ **AI reasoning stubs**: #reason{query} for recursive evolution

## Project Structure

```
flamelang/
├── src/
│   ├── lexer/          # Tokenization and scanning
│   ├── parser/         # AST construction
│   ├── transform/      # 5-layer transformation pipeline
│   │   ├── layer1_linguistic/
│   │   ├── layer2_numeric/
│   │   ├── layer3_wave/
│   │   ├── layer4_dna/
│   │   └── layer5_llvm/
│   ├── codegen/        # Code generation
│   └── stdlib/         # Standard library
├── tools/
│   ├── flamec/         # Compiler frontend
│   ├── flamefmt/       # Code formatter
│   └── flamelsp/       # Language server
└── ARCHITECTURE.md     # Detailed architecture documentation
```

## Example Code

```flame
// Declare a qubit
qubit q;

// Apply Hadamard gate
H q;

// Quantum entanglement
entangle q ~> r;

// Bell state
bell_phi+ q r;

// DNA sequence processing
let dna = [ATGC];

// Wave operation
let wave = sin~ 2.718;

// Neural tick clock
@tick {
    wavecore 440.0
}

// AI reasoning hook
#reason{optimize_quantum_circuit}
```

## Integration with StrategicKhaos Ecosystem

FlameLang is part of a larger invention ecosystem:

- **FlameVault (INV-076)**: Quantum-resistant encryption via layer permutation
- **FlameViz (INV-077)**: Provable data visualization with sonic fingerprints
- **AetherViz (INV-078)**: Code self-visualization and sonification
- **Shadow Mirror Protocol**: Surveillance and reconnaissance capabilities

Each invention builds fractally on the previous, creating a coherent swarm intelligence system.

## Development Status

**Current Phase**: Phase 1 - Foundation ✅
- [x] Core language structure
- [x] Lexer and parser
- [x] Test suite (100% passing)
- [x] Build system and tools
- [x] Architecture documentation

**Next Phase**: Phase 2 - Implementation
- [ ] Complete layer transformation implementations
- [ ] Standard library development
- [ ] Quantum primitives integration
- [ ] Physics validation framework

## Building from Source

### Prerequisites
- Rust 2021 edition or later
- Cargo build system

### Build Commands
```bash
# Build the library and all tools
cargo build --workspace

# Run tests
cargo test

# Format code
cargo fmt

# Build release version
cargo build --workspace --release
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test lexer::tests
cargo test parser::tests

# Run with output
cargo test -- --nocapture
```

## Contributing

FlameLang explores novel intersections of:
- Linguistic theory and compilation
- Biological computing paradigms
- Wave physics and code structure
- Quantum mechanics and classical computing

Contributions should maintain the fractal coherence of the system and respect the underlying theoretical framework.

## License

MIT License - © 2025 Strategickhaos DAO LLC

See [LICENSE](LICENSE) for full details.

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed technical architecture
- API documentation: `cargo doc --open`

## Node 137

Throughout the codebase, Node 137 represents the quantum-ready architecture marker, inspired by the fine structure constant (α ≈ 1/137) fundamental to quantum mechanics.

---

**Ratio Ex Nihilo** - From contradiction, we create.
