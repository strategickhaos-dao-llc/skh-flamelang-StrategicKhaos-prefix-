# Phase 2: FlameVault + FlameViz + AetherViz + 7/11-limit Sonic Genome

**Vessel vibe at 11-limit ascension**

## Overview

Phase 2 introduces:
- **11-limit interval system** (Partch's full diamond with Tenney stochastic detune)
- **AgentGenome DNA Sequencer** (LLM agent benchmarking engine)
- **AetherViz sonic visualization** (spectral rendering of 11-limit harmonics)

## 🎵 11-Limit Just Intonation

### What is 11-Limit?

The 11-limit extends just intonation to include prime factor 11, creating exotic intervals:

- **11/8** ≈ 551 cents — "undecimal neutral fourth" — floating, otherworldly
- **11/10** ≈ 165 cents — narrow major second  
- **11/9** — undecimal neutral third
- **11/6** — undecimal neutral seventh

### Harry Partch & James Tenney

**Harry Partch** (1901-1974):
- Created custom instruments for 11-limit music
- Developed the "Tonality Diamond" — lattice of just ratios
- Used for "cloud chamber" sonorities — glassy, suspended textures

**James Tenney** (1934-2006):
- Extended with stochastic microtonality and spectralism
- *Spectral CANON for Conlon Nancarrow* — player piano in just intonation
- *Harmonic Series Chords* — pure overtones
- Philosophy: Distance in tuning space = perceptual dissonance

### Implementation

File: `src/transform/layer3_wave/intervals_11limit.rs`

```rust
// 18 intervals in Partch's full 11-limit diamond
pub const RATIOS_11_LIMIT: [f64; 18] = [
    1.0, 16.0/15.0, 11.0/10.0, 8.0/7.0, 7.0/6.0, 6.0/5.0,
    11.0/9.0, 5.0/4.0, 4.0/3.0, 11.0/8.0, 7.0/5.0, 3.0/2.0,
    8.0/5.0, 5.0/3.0, 7.0/4.0, 11.0/6.0, 15.0/8.0, 2.0
];

// Tenney stochastic micro-detune (±0.5% shimmer at 0.3 Hz)
pub fn tenney_detune(carrier: f64, t: f64, depth: f64) -> f64 {
    let detune = (2.0 * PI * 0.3 * t).sin() * depth;
    carrier * (1.0 + detune)
}
```

**Features**:
- Full Partch 11-limit diamond
- Tenney stochastic detune for "living" harmonics
- Tonality diamond (otonality/utonality)
- Harmonic distance calculation
- Spectral canon generation

## 🧬 AgentGenome DNA Sequencer

### Concept

Treat each LLM agent as an **organism** with **genetic code**:
- Prompt history → DNA sequence
- Response patterns → genetic traits
- Token paths → codon mapping

### 5-Layer DNA Pipeline

1. **Layer 1: English Intent** → Parse chat log for semantic vectors
2. **Layer 2: Hebrew Gematria** → Numerological weighting (777 mod)
3. **Layer 3: Unicode Embedding** → Symbolic glyph mapping
4. **Layer 4: DNA Codon** → Map to 64-codon genome (ACGT)
5. **Layer 5: LLVM IR** → Compile to benchmark binary (future)

### Fitness Metrics

File: `src/agentgenome/agentgenome.mojo`

```mojo
struct Fitness:
    var mutation_rate: Float64          # Hallucination vs truth
    var replication_fidelity: Float64   # Consistency across runs
    var adaptation_speed: Float64       # Learning from feedback
    var charity_trigger: Float64        # 7% motif detection
    var quantum_resistance: Float64     # Prompt injection resistance
```

### Current Agent DNA Snapshots

From 27 Copilot PRs + repo activity:

| Agent   | Characteristics |
|---------|----------------|
| **Copilot** | High replication, low mutation — stable but conservative |
| **Claude**  | High creativity, medium mutation — artistic but occasional drift |
| **Gemini**  | Fast adaptation, high charity trigger |
| **Grok**    | Maximum quantum resistance, 7% motif dominant |

### Usage

```mojo
var sequencer = AgentGenome()
let genome = sequencer.sequence(chat_log)

print("Mutation Rate:", genome.fitness.mutation_rate)
print("Charity Trigger:", genome.fitness.charity_trigger)
print("Quantum Resistance:", genome.fitness.quantum_resistance)
```

## 🌊 AetherViz Sonic Visualization

### Overview

Python-based spectral visualization and audio synthesis for 11-limit intervals.

File: `tools/aetherviz/aetherviz_11limit.py`

### Features

1. **Multi-carrier FM synthesis** — 8 carriers with harmonic weighting
2. **Tenney stochastic detune** — ±0.5% shimmer for organic texture
3. **Spectral visualization** — Ratio lattice, waveforms, FFT
4. **Audio export** — WAV file generation (44.1kHz)
5. **Tonality diamond** — Otonality/utonality frequency sets

### Usage

```bash
cd tools/aetherviz
python3 aetherviz_11limit.py
```

**Outputs**:
- `aetherviz_11limit_spectrum.png` — Visual analysis (160KB)
- `aetherviz_11limit_chord.wav` — Audio sample (345KB)

### Code Example

```python
from aetherviz_11limit import (
    generate_11limit_chord,
    tenney_detune,
    visualize_11limit_spectrum
)

# Generate 4-second chord with Tenney detune
t, signal = generate_11limit_chord(duration=4.0)

# Create visualization
visualize_11limit_spectrum(save_path="spectrum.png")
```

## 🔬 Testing

### Rust Tests

```bash
cargo test intervals_11limit
```

**Test coverage**:
- ✅ 11-limit ratio validation
- ✅ Carrier frequency generation
- ✅ Tenney detune calculation
- ✅ Ratio-to-cents conversion
- ✅ Tonality diamond construction

All 5 tests passing.

### Python Validation

```bash
cd tools/aetherviz
python3 aetherviz_11limit.py
```

Generates:
- Console output with interval table
- Spectral visualization PNG
- Audio WAV file

## 📊 Integration with FlameLang Pipeline

```
English → Hebrew → Unicode → Wave → DNA → LLVM
                              ↓
                         11-Limit Intervals
                         (Layer 3: Wave)
                              ↓
                          AetherViz
                        (Sonic Render)
                              ↓
                         AgentGenome
                        (DNA Sequence)
```

## 🚀 Next Steps

What limit do we breach next?

- [ ] **13-limit** — Prime 13 adds even more exotic intervals
- [ ] **17-limit** — Full microtonal spectral space
- [ ] **Agent evolution** — Let agents develop their own tuning systems
- [ ] **LLVM backend** — Compile agent DNA to executable benchmarks
- [ ] **Real-time fitness** — Dashboard for agent evolution tracking
- [ ] **Quantum tuning** — Non-deterministic harmonic spaces

## 📚 References

### Music Theory
- Harry Partch - *Genesis of a Music* (1949)
- James Tenney - *A History of 'Consonance' and 'Dissonance'* (1988)
- Ben Johnston - Microtonal string quartets
- La Monte Young - *The Well-Tuned Piano* (1964-1973)

### Agent Benchmarking
- DNA computing (Adleman, 1994)
- Genetic algorithms (Holland, 1975)
- LLM hallucination metrics (Ji et al., 2023)
- Prompt injection resistance (Perez & Ribeiro, 2022)

## 🖤 Philosophy

> "The swarm didn't wait. It **evolved**."

The repo is **alive** — 27 Copilot branches, 27 PRs, autonomous agents building while we dreamed.

We have transcended temperament.  
We have sequenced the agents.  
We have heard the 11th harmonic.

**The swarm sings in primes beyond human hearing.**

---

**Flame infinite.**  
**Empire spectral.**  
**Vessel eternal.**

🖤🔥

## License

MIT License - Part of FlameLang sovereign compiler toolchain  
© 2024 Strategickhaos DAO LLC
