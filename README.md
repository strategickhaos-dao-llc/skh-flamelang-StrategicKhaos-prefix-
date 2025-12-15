# 🔥 FlameLang v2.0.0

**Sovereign compiler toolchain with multi-dimensional transformation pipeline**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-2.0.0-green.svg)](https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-)

FlameLang is a revolutionary programming language featuring a 5-layer transformation pipeline that converts code through multiple dimensional representations: **English → Hebrew → Unicode → Wave → DNA → LLVM**. It includes native support for quantum computing, wave physics, and biological computing primitives.

## ✨ Features

- 🌍 **Multi-lingual**: English keywords with Hebrew transformations
- ⚛️ **Quantum Primitives**: Built-in quantum computing operations
- 🌊 **Wave Physics**: Physics-validated dimensional analysis
- 🧬 **DNA Encoding**: Biological computation support
- 🔒 **Sovereign**: Independent, self-contained toolchain
- 🚀 **Fast**: Rust-based compiler with LLVM backend (planned)

## 🔄 Transformation Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   English   │───▶│   Hebrew    │───▶│   Unicode   │───▶│    Wave     │───▶│     DNA     │
│   Source    │    │  Transform  │    │  Normalize  │    │   Physics   │    │   Encoding  │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                                                                      │
                                                                                      ▼
                                                                               ┌─────────────┐
                                                                               │     LLVM    │
                                                                               │   Codegen   │
                                                                               └─────────────┘
```

### The 5 Layers

1. **English → Hebrew**: Linguistic transformation of keywords
2. **Hebrew → Unicode**: Normalization and consistent encoding
3. **Unicode → Wave**: Physics-validated dimensional representation
4. **Wave → DNA**: Biological encoding using DNA bases (A, T, C, G)
5. **DNA → LLVM**: Machine code generation via LLVM IR

## 🚀 Quick Start

### Installation

```bash
# Build from source
git clone https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
cd skh-flamelang-StrategicKhaos-prefix-
cargo build --release

# Install CLI
cargo install --path compiler/flamelang-cli
```

### Your First Program

Create `hello.flame`:

```flamelang
fn main() {
    let message = "Hello, FlameLang!";
    return 0;
}
```

Compile it:

```bash
flamelang compile --input hello.flame --verbose
```

## 💡 Language Examples

### Quantum Computing

```flamelang
fn quantum_teleportation() {
    let alice = quantum superpose(0, 1);
    let bob = quantum superpose(0, 1);
    let entangled = entangle(alice, bob);
    let result = measure alice;
    return result;
}
```

### Wave Physics

```flamelang
fn wave_synthesis() {
    let carrier = wave {
        frequency: 440.0,
        amplitude: 1.0,
        phase: 0.0
    };
    return carrier;
}
```

### DNA Encoding

```flamelang
fn dna_encoding() {
    let data = "Hello, World!";
    let encoded = encode(data);
    let decoded = decode(encoded);
    return decoded;
}
```

## 📚 Documentation

- [Getting Started Guide](docs/GETTING_STARTED.md)
- [Architecture Overview](docs/ARCHITECTURE.md)
- [Invention Inventory](INVENTION_INVENTORY.md)
- [Docker Hub Inventory](PREPROCESSOR_DOCKER_INVENTORY.md)

## 🛠️ CLI Commands

```bash
# Compile a program
flamelang compile --input program.flame --output program.ll --verbose

# Show tokens with Hebrew transformations
flamelang lex --input program.flame --show-hebrew

# Transform through pipeline stages
flamelang transform --input program.flame --stage dna --format json

# Show compiler information
flamelang info
```

## 🏗️ Project Structure

```
skh-flamelang-StrategicKhaos-prefix-/
├── compiler/
│   ├── flamelang-lexer/       # Layer 1: Lexical analysis
│   ├── flamelang-parser/      # Parsing
│   ├── flamelang-ast/         # Abstract syntax tree
│   ├── flamelang-transform/   # Layers 2-4: Transformations
│   ├── flamelang-codegen/     # Layer 5: LLVM codegen
│   └── flamelang-cli/         # Command-line interface
├── runtime/                    # Runtime library
├── stdlib/                     # Standard library
├── examples/                   # Example programs
├── docs/                       # Documentation
├── Dockerfile                  # Docker container
└── docker-compose.yml         # Container orchestration
```

## 🐳 Docker Support

```bash
# Build and run with Docker
docker build -t flamelang:latest .
docker run flamelang:latest info

# Use docker-compose
docker-compose up compiler
```

## 🧪 Testing

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p flamelang-lexer
cargo test -p flamelang-transform

# Run with verbose output
cargo test --all -- --nocapture
```

## 🔬 Native Primitives

### Quantum Operations
- `quantum`: Quantum type
- `superpose`: Create quantum superposition
- `entangle`: Entangle quantum states
- `measure`: Measure quantum state

### Wave Operations
- `wave`: Wave type
- `frequency`: Get/set frequency
- `amplitude`: Get/set amplitude
- `phase`: Get/set phase

### DNA Operations
- `dna`: DNA type
- `encode`: Encode data to DNA
- `decode`: Decode DNA to data
- `sequence`: DNA sequence literal

## 📦 Dependencies

- **Rust**: 1.75+
- **logos**: Fast lexer generation
- **unicode-normalization**: Unicode text processing
- **serde**: Serialization
- **clap**: CLI framework
- **inkwell**: LLVM bindings (planned)

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines (coming soon).

## 📄 License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## 🏢 Governance

Maintained by **StrategicKhaos DAO LLC**  
Part of the **Khaos Catalyst Swarm Intelligence** ecosystem

## 🗺️ Roadmap

### v2.0.0 (Current)
- ✅ Lexer with English→Hebrew transformation
- ✅ AST structure
- ✅ Unicode→Wave→DNA transformation pipeline
- ✅ CLI tool
- ✅ Docker support

### v2.1.0 (Planned)
- Parser implementation
- Type system
- Semantic analysis
- Basic LLVM code generation

### v2.5.0 (Future)
- Full LLVM integration
- Optimization passes
- Standard library
- Package manager

### v3.0.0 (Vision)
- JIT compilation
- Quantum simulator integration
- Biological hardware targets
- Neural network compilation

## 🔗 Links

- [GitHub Repository](https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-)
- [Documentation](docs/)
- [Examples](examples/)
- [StrategicKhaos DAO](https://github.com/strategickhaos-dao-llc)

---

**Built with 🔥 by StrategicKhaos DAO LLC**
