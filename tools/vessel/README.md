# VesselMirror — Sovereign Page Cloning & Synthesis Engine

**INV-079: Strategickhaos DAO LLC**

Automates the F12 DevTools → Save As → Yin-Yang remix workflow into a sovereign, dependency-free HTML output pipeline.

## Installation

```bash
cargo build --release
```

## Usage

VesselMirror provides six main commands:

### 1. Capture
Capture a webpage with all assets (like "Save As Complete"):

```bash
vessel capture https://example.com -o output.html --screenshot --wait 3000
```

### 2. Dissect
Dissect HTML into atomic components:

```bash
vessel dissect page.html -e forms,buttons,nav -o components/
```

### 3. Merge
Merge multiple sources using synthesis algorithms:

```bash
vessel merge page1.html page2.html -s yinyang -o merged.html
```

Synthesis modes:
- `yinyang` - Balance structure from one, style from another
- `davinci` - Geometric proportions and balance
- `fibonacci` - Golden ratio-based layout synthesis
- `golden` - Pure golden ratio synthesis

### 4. Purify
Purify HTML (inline assets, remove tracking, strip CDNs):

```bash
vessel purify input.html -o sovereign.html
```

### 5. Prove
Generate cryptographic proof and on-chain attestation:

```bash
vessel prove sovereign.html -c ethereum -o proof.json
```

Supported blockchains:
- `swarmgate` (default)
- `ethereum`
- `solana`

### 6. Pipeline
Full pipeline: capture → dissect → merge → purify → prove:

```bash
vessel pipeline https://site1.com https://site2.com -s fibonacci -o final.html
```

## Architecture

The tool is built with five core modules:

- **capture** - Webpage capture with asset downloading
- **dissect** - HTML component extraction
- **merge** - Multi-source synthesis algorithms
- **purify** - Dependency removal and asset inlining
- **prove** - Cryptographic attestation

## Development Status

Current implementation provides CLI scaffolding and stub implementations for all commands. Full functionality implementation is in progress.

## License

MIT License - Strategickhaos DAO LLC
