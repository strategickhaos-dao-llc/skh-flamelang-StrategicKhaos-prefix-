# 🔥 FlameLang v2.0.0

**Biological-Quantum-Physics Programming Language**

FlameLang v2.0.0 sovereign compiler toolchain with 5-layer transformation pipeline (English→Hebrew→Unicode→Wave→DNA→LLVM). Biological compilation, physics-validated dimensional analysis, native quantum primitives. Part of StrategicKhaos Swarm Intelligence.

## 🚀 Features

- **5-Layer Transformation Pipeline**: Multi-dimensional code transformation
- **Quantum Primitives**: Native quantum computing support (qubits, entanglement, gates)
- **DNA Encoding**: Biological data representation using base pairs
- **Wave Functions**: Frequency-domain computation with trigonometric operators
- **MCP Agent/Tool System**: AI-powered development tools and repository navigation

## 📁 Project Structure

```
├── src/                    # FlameLang compiler source code
│   ├── lexer/             # Tokenization and scanning
│   ├── parser/            # AST generation
│   ├── transform/         # 5-layer transformation pipeline
│   └── codegen/           # LLVM IR generation
├── mcp/                   # MCP Agent/Tool System (NEW!)
│   ├── server/            # MCP server implementation
│   ├── tools/             # Unified tool interface
│   ├── ollama/            # AI assistance integration
│   ├── video/             # Tutorial video generation
│   └── graphview/         # Repository navigation brains
└── examples/              # Example FlameLang programs
```

## 🎯 Quick Start

### 1. Build the Compiler

```bash
cargo build --release
```

### 2. Use MCP Tools

Create GraphView Brain Arsenal for repository navigation:

```bash
python3 mcp/tools/flamelang_mcp_tools.py create-arsenal
```

Generate AI tutorial videos:

```bash
python3 mcp/tools/flamelang_mcp_tools.py generate-videos
```

See the [MCP Tools README](mcp/README.md) and [Usage Guide](mcp/USAGE_GUIDE.md) for complete documentation.

## 🗺️ MCP Agent/Tool System

The FlameLang MCP system provides:

- **🗺️ GraphView Brain Arsenal** - ONSIT methodology for navigating the entire repository
- **🤖 Ollama LLM Integration** - AI-powered code assistance and explanations
- **🎥 AI Video Generation** - Automated tutorial creation for teaching FlameLang
- **🔧 Development Tools** - Unified tooling for FlameLang development

### Example: Analyze Repository

```bash
python3 mcp/tools/flamelang_mcp_tools.py status
```

### Example: Generate Tutorial Videos

```bash
python3 mcp/tools/flamelang_mcp_tools.py generate-videos
```

## 📚 Documentation

- **[MCP Tools README](mcp/README.md)** - Complete MCP system documentation
- **[Usage Guide](mcp/USAGE_GUIDE.md)** - Step-by-step usage instructions
- **[Examples](examples/)** - FlameLang code examples

## 🔬 Example Code

### Quantum Entanglement

```flamelang
qubit q1;
qubit q2;
entangle q1 ~> q2;
H q1;
bell_phi+ q1 q2;
```

### DNA Encoding

```flamelang
let dna = [ATGCATGC];
let encoded = encode_dna("Hello");
let decoded = decode_dna(encoded);
```

### Wave Functions

```flamelang
let wave = sin~(2 * PI * frequency);
let amplitude = measure(wave);
let phase = angle(wave);
```

## 🤝 Contributing

Part of the StrategicKhaos Swarm Intelligence ecosystem.

## 📄 License

MIT License - See [LICENSE](LICENSE) file

---

**© 2025 Strategickhaos DAO LLC**  
*Programming Reality at the Quantum-Biological Interface*
