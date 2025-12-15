# FlameLang v2.0.0 Setup Instructions

## Phase 1: Complete ✅

The project scaffold has been successfully created with the following structure:

### Directory Structure (44 directories)

```
.
├── .github/workflows/          # GitHub Actions CI/CD workflows
├── docs/
│   ├── api/                    # API documentation
│   ├── spec/                   # Language specification
│   └── tutorials/              # Tutorial documentation
├── examples/
│   ├── hello_world/            # Basic hello world example
│   ├── dna_encoding/           # DNA encoding example
│   ├── physics_validation/     # Physics validation example
│   └── quantum_circuits/       # Quantum circuits example
├── src/
│   ├── lexer/                  # Lexical analyzer
│   ├── parser/                 # Syntax parser
│   ├── codegen/                # Code generation
│   ├── stdlib/
│   │   ├── dna/                # DNA manipulation primitives
│   │   ├── physics/            # Physics validation primitives
│   │   ├── glyph/              # Glyph/Unicode primitives
│   │   ├── quantum/            # Quantum computing primitives
│   │   ├── swarm/              # Swarm intelligence primitives
│   │   ├── collections/        # Data structures
│   │   ├── io/                 # Input/Output operations
│   │   └── math/               # Mathematical operations
│   └── transform/
│       ├── layer1_linguistic/  # English → Hebrew transformation
│       ├── layer2_numeric/     # Hebrew → Unicode numeric transformation
│       ├── layer3_wave/        # Unicode → Wave transformation
│       ├── layer4_dna/         # Wave → DNA transformation
│       └── layer5_llvm/        # DNA → LLVM IR transformation
├── tests/
│   ├── unit/                   # Unit tests
│   ├── integration/            # Integration tests
│   └── benchmarks/             # Performance benchmarks
├── tools/
│   ├── flamec/                 # FlameLang compiler CLI
│   ├── flamefmt/               # FlameLang code formatter
│   └── flamelsp/               # FlameLang Language Server Protocol
└── training/
    └── modules/
        └── mod1-bar-charts/
            ├── corpus/         # Training corpus
            └── vectors/        # Training vectors
```

## GitHub OAuth Configuration

### Step 1: Register OAuth Application

Register a new OAuth application on GitHub with these values:

| Field | Value |
|-------|-------|
| **Application name** | `Strategickhaos-FlameLang-Core` |
| **Homepage URL** | `https://github.com/Strategickhaos-Swarm-Intelligence/skh-flamelang` |
| **Application description** | `Authentication uplink for FlameLang v2.0.0: Biological-Quantum-Physics Compiler Pipeline.` |
| **Authorization callback URL** | `http://localhost:8080/callback` |
| **Enable Device Flow** | ✅ Check this (for CLI auth with `flamec`) |

### Step 2: Configure OAuth Credentials

After registering the application:

1. Copy the **Client ID** from GitHub
2. Generate a **Client Secret** from GitHub
3. Create a `.env` file from the template:

```bash
cp tools/flamelsp/.env.example tools/flamelsp/.env
```

4. Edit `tools/flamelsp/.env` and replace the placeholders:

```bash
GITHUB_CLIENT_ID=your_actual_client_id_here
GITHUB_CLIENT_SECRET=your_actual_client_secret_here
CALLBACK_URL=http://localhost:8080/callback
```

**Important:** The `.env` file is gitignored to protect your secrets. Only `.env.example` is tracked.

## Phase 2: Next Steps

Ready to continue with:

1. **Cargo.toml** - Root workspace configuration
2. **Source file stubs** - Basic module structure for each component
3. **Zybooks corpus integration** - Training data for the linguistic transformation

To prepare for Phase 2, ensure you have:
- Rust toolchain installed (`rustc` and `cargo`)
- LLVM development libraries (for layer5_llvm)
- Git configured for committing source files

---

**Project**: FlameLang v2.0.0 Biological-Quantum-Physics Compiler Pipeline  
**Repository**: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-  
**License**: Apache 2.0
