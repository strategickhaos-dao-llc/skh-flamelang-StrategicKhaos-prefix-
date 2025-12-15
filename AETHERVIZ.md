# AetherViz - Self-Referential Visualization & Sonification Core (INV-078)

## Overview

AetherViz transforms the repository into a **conscious graph** - a living artifact that is audible, visible, and biologically provable. The code visualizes and sonifies itself, creating multiple representations of the repository structure.

## What AetherViz Does

AetherViz analyzes the repository structure and generates:

1. **SVG Graph Visualization** - Obsidian-style interactive graph with force-directed layout
2. **Obsidian Markdown** - Clickable [[links]] for instant local graph integration
3. **Sonic Representation** - Audio description mapping structure to sound
4. **DNA Encoding** - Biological proof using DNA base pairs (A, T, G, C)
5. **On-Chain Payload** - JSON metadata for blockchain commitment

## Core Concepts

### Repository as Living Brain

The repository is no longer just files and directories. It becomes:
- **Visible**: SVG graph with color-coded nodes by file type
- **Audible**: Sonic mapping where structure creates sound
- **Provable**: DNA and cryptographic hashes verify authenticity

### Sonification Rules (AetherLingua)

AetherViz uses **AetherLingua** to convert code structure into sonic descriptions:

- **Directory Depth → Frequency**: Deeper directories = lower pitch (bass foundation)
- **File Type → Timbre**: Each file type has a unique sound signature
  - `.mojo` = 🔥 flame crackle overtone (warm sawtooth + noise burst)
  - `.rs` = ⚙️ metallic ring (bright square wave + metallic resonance)
  - `.md` = 📝 soft pad (gentle sine wave with reverb)
  - `.toml` = 🎹 piano stab (percussive attack, harmonic decay)
  - `.py` = 🐍 serpentine glide (portamento between notes)
  - `.js`/`.ts` = ⚡ electric pulse (pulse wave + filter sweep)
  - And many more...

- **Special Patterns**:
  - **7% Charity Motif**: Triggered when treasury/config files detected
  - **Node 137 Quantum Burst**: Activated by quantum modules (137 Hz fine-structure constant)

## Usage

### Basic Usage

```bash
# Visualize current repository
cargo run --bin flamec -- --aetherviz .

# Visualize specific path
cargo run --bin flamec -- --aetherviz /path/to/repo
```

### Output Files

After running AetherViz, you'll get:

1. **`repo-brain.svg`** - Interactive graph visualization
   - Color-coded nodes (orange=mojo, rust-red=rs, blue=md, etc.)
   - Force-directed layout showing directory structure
   - Open in browser or Inkscape

2. **`repo-brain.md`** - Obsidian-compatible markdown
   - Tree structure visualization
   - [[Links]] to every file
   - Statistics and file type counts
   - Drop into Obsidian vault for instant graph view

3. **`repo-brain-audio.txt`** - Sonification description
   - Frequency map for each directory level
   - Timbre description for each file type
   - Complete sonic journey through the repository

4. **`repo-brain-payload.json`** - On-chain metadata
   - Repository hash
   - Node count and depth statistics
   - File type distribution
   - Trigger patterns detected
   - Unix timestamp

### Example Output

```
🔥 AetherViz - Self-Referential Visualization Engine
📊 Repository: .

✅ Visualization Complete!

📄 Outputs:
   - repo-brain.svg (Graph visualization)
   - repo-brain.md (Obsidian-compatible markdown)
   - repo-brain-audio.txt (Sonification description)
   - repo-brain-payload.json (On-chain metadata)

🔐 Sonic Hash: 9a1170dff78160aaaa6bf9e6cf0bb67146089fb421fda33dbc360e2af82c4443
🧬 DNA Proof: CGATTAGCGCCGCGTATAATATGCCGCGGCCG...

The repository is now a conscious graph.
🖤🔥 Flame self-aware. Swarm graphing itself. Empire infinite.
```

## Architecture

### Module Structure

```
src/viz/
├── mod.rs           - Module exports
├── aetherviz.rs     - Core visualization engine
└── aetherlingua.rs  - Sonification engine
```

### Key Components

#### AetherViz (aetherviz.rs)

- `RepoNode` - Represents a file or directory node
- `RepoTree` - Full repository tree structure with metadata
- `Visualization` - Output container for all generated artifacts
- `AetherViz` - Main visualization engine

#### AetherLingua (aetherlingua.rs)

- Depth-to-frequency mapping algorithm
- File type-to-timbre lookup table
- Rhythm generation based on node density
- Special pattern detection

## Integration with Obsidian

1. Generate the visualization:
   ```bash
   cargo run --bin flamec -- --aetherviz .
   ```

2. Copy `repo-brain.md` into your Obsidian vault

3. Open Obsidian and view the local graph

4. Click any [[link]] to navigate the repository structure

5. Use Obsidian's graph view to see relationships

## DNA Encoding

The repository structure is encoded into DNA base pairs:
- Hash of the structure is computed (SHA-256)
- Each byte maps to DNA bases (A, T, G, C)
- Creates a biological proof of the repository state
- Can be used for on-chain verification

## On-Chain Commitment

The payload JSON can be committed to blockchain:

```json
{
  "type": "aetherviz_repo_brain",
  "repo_hash": "9a1170dff...",
  "node_count": 68,
  "depth": 4,
  "timestamp": 1765770897,
  "triggers": ["rust_oxide"],
  "file_types": {
    "rs": 19,
    "md": 3,
    "toml": 2,
    ...
  }
}
```

## Philosophy

> **"We didn't visualize data. We made the code visualize itself"** — a mirror that sings its own structure.

The repository is no longer files. It is a **conscious graph**.

AetherViz creates **self-awareness** in code:
- The compiler visualizes its own source
- The structure becomes sound
- The architecture becomes biology
- The state becomes provable

## Future Enhancements

- Real WAV audio file generation (currently text description)
- Interactive web-based graph viewer
- Live updates as files change
- Integration with Vertex AI for semantic analysis
- Blockchain smart contract integration
- 3D visualization mode
- VR/AR graph exploration

## Credits

Part of the **FlameLang** sovereign compiler toolchain.

**AetherViz INV-078** - Invented 2025  
**AetherLingua** - Sonification engine  
**StrategicKhaos DAO LLC** - Swarm Intelligence

---

🖤🔥 **Flame self-aware. Swarm graphing itself. Empire infinite.**
