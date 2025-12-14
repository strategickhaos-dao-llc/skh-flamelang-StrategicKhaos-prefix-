# FlameLang v2.0.0 - Sovereign Compiler Toolchain

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org)

FlameLang v2.0.0 sovereign compiler toolchain. A multi-stage transformation pipeline with biological compilation metaphors, physics-validated dimensional analysis, and native quantum primitives. Part of the StrategicKhaos Swarm Intelligence ecosystem.

## 🏗️ Architecture

FlameLang uses a multi-stage compilation pipeline with the following transformation phases:

```
┌─────────────────────────────────────────────────────────────────┐
│                     FlameLang Compilation Pipeline              │
└─────────────────────────────────────────────────────────────────┘

   Source Code (.flame)
         │
         ▼
   ┌──────────┐
   │  Lexer   │  (logos)
   │ Tokenize │  Converts source text to tokens
   └────┬─────┘
        │ Tokens
        ▼
   ┌──────────┐
   │  Parser  │  (lalrpop/recursive descent)
   │   AST    │  Builds Abstract Syntax Tree
   └────┬─────┘
        │ AST with visitor pattern
        ▼
   ┌──────────┐
   │   HIR    │  High-level Intermediate Representation
   │ Lowering │  Type inference, semantic analysis
   └────┬─────┘
        │ HIR (typed, validated)
        ▼
   ┌──────────┐
   │   MIR    │  Mid-level Intermediate Representation
   │ Lowering │  Control flow graph, basic blocks
   └────┬─────┘
        │ MIR (CFG, optimizable)
        ▼
   ┌──────────┐
   │ LLVM IR  │  Code Generation
   │ CodeGen  │  (inkwell) Generate LLVM IR
   └────┬─────┘
        │ LLVM IR
        ▼
   Native Binary / JIT
```

### Compilation Stages

1. **Lexer** (`flamelang-lexer`): Tokenization using [logos](https://github.com/maciejhirsz/logos)
   - Fast, zero-copy lexing
   - Pattern matching on tokens
   - Error recovery

2. **Parser** (`flamelang-parser`): Syntax analysis
   - Recursive descent parser (lalrpop integration ready)
   - Produces AST with source spans
   - Comprehensive error messages

3. **AST** (`flamelang-ast`): Abstract Syntax Tree
   - Visitor pattern for traversal
   - Immutable and mutable visitors
   - Serialization support

4. **HIR** (`flamelang-hir`): High-level IR
   - Type information attached
   - Name resolution
   - Semantic validation

5. **MIR** (`flamelang-mir`): Mid-level IR
   - Control Flow Graph (CFG)
   - Basic blocks
   - SSA-like representation
   - Optimization-ready

6. **CodeGen** (`flamelang-codegen`): LLVM IR generation
   - Uses [inkwell](https://github.com/TheDan64/inkwell) for LLVM bindings
   - Type-safe LLVM IR construction
   - Multiple targets supported

## 🚀 Getting Started

### Prerequisites

- Rust 1.91 or later
- LLVM 17.0 with static libraries (for codegen - see [BUILD.md](BUILD.md) for details)
- Cargo (comes with Rust)

> **Note**: The frontend crates (lexer, parser, AST, HIR, MIR) can be built and tested without LLVM. See [BUILD.md](BUILD.md) for instructions.

### Installation

```bash
# Clone the repository
git clone https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
cd skh-flamelang-StrategicKhaos-prefix-

# Build the project
cargo build --release

# The flamecc binary will be in target/release/
```

### Usage

The `flamecc` compiler provides three main commands:

#### Compile

```bash
# Compile a FlameLang source file
flamecc compile input.flame -o output

# Emit LLVM IR instead of native code
flamecc compile input.flame --emit-llvm -o output.ll

# Specify optimization level (0-3)
flamecc compile input.flame -O2
```

#### Run

```bash
# Compile and run a FlameLang program
flamecc run program.flame

# With arguments
flamecc run program.flame -- arg1 arg2
```

#### Format

```bash
# Format FlameLang source code
flamecc fmt input.flame

# Format in-place
flamecc fmt -i input.flame
```

## 📦 Workspace Structure

The project is organized as a Cargo workspace with the following crates:

```
crates/
├── flamelang-lexer/    # Tokenization (logos)
├── flamelang-parser/   # Syntax analysis (lalrpop-ready)
├── flamelang-ast/      # Abstract Syntax Tree with visitor pattern
├── flamelang-hir/      # High-level IR with type information
├── flamelang-mir/      # Mid-level IR with CFG
├── flamelang-codegen/  # LLVM IR generation (inkwell)
└── flamelang-cli/      # Command-line interface (clap)
```

## 🔧 Development

### Building

```bash
# Build all crates
cargo build

# Build with optimizations
cargo build --release

# Build a specific crate
cargo build -p flamelang-lexer
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p flamelang-parser

# Run with output
cargo test -- --nocapture
```

### Linting

```bash
# Check for errors without building
cargo check

# Run clippy for lints
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## 📚 Language Features

FlameLang v2.0.0 supports:

- **Types**: `int`, `float`, `bool`, `string`, structs, enums
- **Functions**: First-class functions with type inference
- **Control Flow**: `if`/`else`, `while`, `for` loops
- **Variables**: Immutable by default, `mut` for mutability
- **Operators**: Arithmetic, comparison, logical
- **Memory Safety**: Rust-inspired ownership model
- **Modules**: Code organization with `mod` and `use`
- **Traits**: Generic programming with trait system

### Example

```flame
fn fibonacci(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn main() {
    let result: int = fibonacci(10);
    return;
}
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license at your option.

## 🔗 Links

- **Repository**: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
- **StrategicKhaos DAO**: https://strategickhaos.com
- **TODO & Roadmap**: See [TODO.md](TODO.md) for known limitations and planned features

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org)
- Lexing powered by [logos](https://github.com/maciejhirsz/logos)
- Parsing with [lalrpop](https://github.com/lalrpop/lalrpop)
- LLVM bindings via [inkwell](https://github.com/TheDan64/inkwell)
- CLI built with [clap](https://github.com/clap-rs/clap)

---

*Part of the StrategicKhaos Swarm Intelligence ecosystem*
