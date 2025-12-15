# Getting Started with FlameLang v2.0.0

## Installation

### Prerequisites

- Rust 1.75 or later
- LLVM 16.0 (for code generation, optional)
- Docker (for containerized development, optional)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
cd skh-flamelang-StrategicKhaos-prefix-

# Build the compiler
cargo build --release

# Run tests
cargo test --all

# Install the CLI tool
cargo install --path compiler/flamelang-cli
```

### Using Docker

```bash
# Build the Docker image
docker build -t flamelang:latest .

# Run the compiler
docker run flamelang:latest info

# Or use docker-compose
docker-compose up compiler
```

## Quick Start

### Your First FlameLang Program

Create a file called `hello.flame`:

```flamelang
fn main() {
    let message = "Hello, FlameLang!";
    return 0;
}
```

### Compile and Run

```bash
# Compile the program
flamelang compile --input hello.flame --output hello.ll --verbose

# Show transformation stages
flamelang transform --input hello.flame --stage dna --format text

# Lex the source (show tokens)
flamelang lex --input hello.flame --show-hebrew
```

## Language Features

### Basic Syntax

#### Variables

```flamelang
// Immutable variable
let x = 42;

// Mutable variable
let mut y = 10;
y = 20;

// Type annotations
let z: int = 100;
```

#### Functions

```flamelang
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() {
    let result = add(5, 3);
    return 0;
}
```

#### Control Flow

```flamelang
fn example() {
    // If-else
    if x > 10 {
        return 1;
    } else {
        return 0;
    }
    
    // While loop
    let mut i = 0;
    while i < 10 {
        i = i + 1;
    }
    
    // For loop
    for item in items {
        // process item
    }
}
```

### Quantum Programming

```flamelang
fn quantum_example() {
    // Create quantum superposition
    let q = quantum superpose(0, 1);
    
    // Entangle qubits
    let q1 = quantum superpose(0, 1);
    let q2 = quantum superpose(0, 1);
    let entangled = entangle(q1, q2);
    
    // Measure quantum state
    let result = measure q;
    
    return result;
}
```

### Wave Programming

```flamelang
fn wave_example() {
    // Create a wave
    let w = wave {
        frequency: 440.0,  // A4 note (Hz)
        amplitude: 1.0,
        phase: 0.0
    };
    
    // Access wave properties
    let f = frequency(w);
    let a = amplitude(w);
    let p = phase(w);
    
    return w;
}
```

### DNA Programming

```flamelang
fn dna_example() {
    // Encode data as DNA
    let data = "Hello";
    let seq = encode(data);
    
    // Create DNA sequence
    let dna_seq = sequence {
        bases: [A, T, C, G, A, T, C, G]
    };
    
    // Decode DNA to data
    let decoded = decode(seq);
    
    return decoded;
}
```

## CLI Commands

### `flamelang compile`

Compile a FlameLang source file through all 5 transformation layers.

```bash
flamelang compile --input program.flame --output program.ll --verbose
```

Options:
- `-i, --input <FILE>`: Input source file (required)
- `-o, --output <FILE>`: Output file (default: input with .ll extension)
- `-v, --verbose`: Show detailed transformation stages

### `flamelang lex`

Tokenize a source file and display tokens.

```bash
flamelang lex --input program.flame --show-hebrew
```

Options:
- `-i, --input <FILE>`: Input source file (required)
- `-H, --show-hebrew`: Display Hebrew transformations

### `flamelang transform`

Transform source through specific pipeline stages.

```bash
flamelang transform --input program.flame --stage dna --format json
```

Options:
- `-i, --input <FILE>`: Input source file (required)
- `-s, --stage <STAGE>`: Output stage (unicode, wave, or dna)
- `-f, --format <FORMAT>`: Output format (text or json)

### `flamelang info`

Display compiler version and pipeline information.

```bash
flamelang info
```

## Examples

The `examples/` directory contains sample programs:

- `hello.flame`: Basic hello world with all primitive types
- `quantum.flame`: Quantum computing examples
- `wave.flame`: Wave manipulation examples
- `dna.flame`: DNA encoding examples

## Understanding the Pipeline

### Layer 1: English → Hebrew

Keywords are transformed to Hebrew equivalents:

```
fn      → פונקציה
let     → תן
const   → קבוע
if      → אם
```

### Layer 2: Hebrew → Unicode

Hebrew text is normalized using Unicode NFC form for consistency.

### Layer 3: Unicode → Wave

Text is transformed to wave properties:
- Frequency (Hz)
- Amplitude (dimensionless)
- Phase (radians)
- Wavelength (m)
- Energy (J)

### Layer 4: Wave → DNA

Wave properties are encoded as DNA sequences:
- A, T, C, G bases
- Grouped into codons (triplets)

### Layer 5: DNA → LLVM

DNA sequences are compiled to LLVM IR (planned).

## Troubleshooting

### Build Errors

**Problem**: Cargo build fails with dependency errors

**Solution**: Update dependencies
```bash
cargo update
cargo build --release
```

**Problem**: LLVM not found

**Solution**: Install LLVM 16.0 or comment out inkwell dependency in Cargo.toml

### Runtime Errors

**Problem**: Unknown token error

**Solution**: Check syntax against language specification

**Problem**: Transform error

**Solution**: Enable verbose logging
```bash
RUST_LOG=debug flamelang compile --input file.flame --verbose
```

## Next Steps

- Read the [Architecture Documentation](ARCHITECTURE.md)
- Explore [Example Programs](../examples/)
- Review the [Language Specification](LANGUAGE_SPEC.md)
- Check the [API Documentation](https://docs.rs/flamelang)

## Community and Support

- GitHub Issues: Report bugs and feature requests
- Discussions: Ask questions and share projects
- DAO Governance: Participate in language evolution

---

**Version**: 2.0.0  
**Last Updated**: 2025-12-14  
**Maintained By**: StrategicKhaos DAO LLC
