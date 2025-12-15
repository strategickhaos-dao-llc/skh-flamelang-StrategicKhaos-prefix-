# AetherViz v2 — Repository Sonification Engine

**Self-Referential Repository Sonification & Visualization**

AetherViz transforms repository structure into music using Just Intonation and physics-grounded synthesis principles. The repository now **sings its own structure**.

## 🎵 Music Theory Foundation

### Just Intonation (Pure Ratios)
- **Base frequency**: 110 Hz (A2)
- **Carrier frequencies**: Perfect fifths progression `[110.0 * (3/2)^i]`
  - A (110 Hz), E (165 Hz), B (247.5 Hz), F# (371.25 Hz), C# (556.875 Hz), G# (835.3125 Hz)

### Sonic Motifs

#### 7% Charity Motif
- **Perfect fifth glissando** (1.5 ratio)
- E3 (165 Hz) → A3 (220 Hz)
- Triangle wave with exponential decay
- Treasury trigger — resonance for benevolent action

#### 137 Burst
- **Golden ratio micro-detune** (φ = 1.618)
- Sawtooth wave for sharp attack
- Exponential decay (τ = 6)
- Prime awakening through harmonic series disruption

### Timbre Architecture

| File Type | Timbre | Waveform | Characteristics |
|-----------|--------|----------|-----------------|
| `.py` | Flute | Sine + light reverb | Smooth, pure |
| `.mojo` | Bronze bell | Inharmonic partials | Metallic, complex |
| `.md` | Harp pluck | Triangle decay | Percussive, organic |
| `.rs` | Metallic ring | High resonance | Sharp, sustained |

### Spatial Positioning
- **Stereo panning** by carrier frequency
- Creates depth and separation in the sonic field
- Six distinct positions from center to hard L/R

## 🔧 Implementation

### Quick Start

```bash
# Install dependencies
pip3 install -r requirements.txt

# Run AetherViz
python3 aetherviz.py
```

### Output
- **File**: `aetherviz_repo_brain.wav`
- **Format**: 48kHz, 16-bit stereo WAV
- **Duration**: 8 seconds
- **Size**: ~1.5 MB

### Sonic Fingerprint
Each run generates a SHA-256 hash of the audio data:
```
Sonic Fingerprint: 87ec16ba5270706904c090a43f5b5e8647526fa598b1b6a71cf7a16b72b427a9
```
This provides cryptographic proof of the sonic output.

## 📊 Side-by-Side Stepper Crawler

### Python → FlameLang (Mojo) Conversion Table

| Line # | Python | FlameLang (Mojo) Conversion | ONSIT Step |
|--------|--------|------------------------------|------------|
| 1 | `import os` | `import os` | **Linguistic** → same intent |
| 2 | `SAMPLE_RATE = 48000` | `let SAMPLE_RATE: Int = 48000` | **Numeric** → typed constant |
| 3 | `def sine(freq, t):` | `fn sine(freq: Float64, t: Float64) -> Float64:` | **Wave** → pure function |
| 4 | `np.sin(...)` | `math.sin(...)` | **Wave** → direct math |
| 5 | `adsr envelope` | `fn adsr(t: Float64, dur: Float64) -> Float64:` | **DNA** → deterministic envelope |
| 6 | `charity_gliss` | `fn charity_gliss(carrier: Float64, t: Float64) -> Float64:` | **Charity motif** → treasury trigger |
| 7 | `write_wav` | LLVM IR codegen → native binary | **Final layer** → executable |
| 8 | `hashlib.sha256` | `hashlib.sha256` → on-chain commitment | **Proof of sound** |

### ONSIT Layer Mapping

The **5-Layer ONSIT Transformation** maps as follows:

1. **Linguistic Layer** (English → Hebrew)
   - Python imports → Mojo imports (semantic equivalence)

2. **Numeric Layer** (Hebrew → Unicode)
   - Dynamic typing → Static typing (`Float64`, `Int`)
   - Constants with explicit types

3. **Wave Layer** (Unicode → Wave Functions)
   - NumPy operations → Native math library
   - Pure functions without side effects

4. **DNA Layer** (Wave → Deterministic State)
   - ADSR envelopes → Biological-inspired state machines
   - Deterministic audio synthesis

5. **LLVM Layer** (DNA → Native Code)
   - Python bytecode → LLVM IR
   - Native binary with zero-cost abstractions

## 🔬 Technical Architecture

### Signal Processing Chain

```
Repository Structure
    ↓
Text Lines (semantic units)
    ↓
Carrier Frequency Selection (Just Intonation)
    ↓
Waveform Synthesis (sine, saw, triangle)
    ↓
Special Motifs (7% gliss, 137 burst)
    ↓
Harmonic Enrichment (inharmonic partials)
    ↓
ADSR Envelope (organic shaping)
    ↓
Stereo Panning (spatial depth)
    ↓
Mixing (additive synthesis)
    ↓
Normalization & WAV Export
    ↓
SHA-256 Fingerprint (proof of sound)
```

### Key Functions

#### `sine(freq, t)` — Pure Tone Generator
```python
def sine(freq, t): 
    return np.sin(2 * np.pi * freq * t)
```

#### `adsr(t, dur)` — Organic Envelope
```python
# Attack → Decay → Sustain → Release
# Biological-inspired amplitude shaping
```

#### `render_line(text, carrier)` — Line Sonification
```python
# Converts a single line to 8 seconds of stereo audio
# Applies carrier frequency, motifs, and spatial positioning
```

## 🎯 Use Cases

### 1. Repository Structure Audition
Listen to the architecture of your codebase. Different frequencies represent different modules.

### 2. Change Detection
Compare sonic fingerprints across commits to detect structural changes.

### 3. Code Review Ambiance
Background audio for code review sessions, creating sonic continuity.

### 4. Documentation Sonification
Feed documentation (Zybooks PDF, etc.) to create study soundscapes.

## 🚀 Future Extensions

### Planned Features
- [ ] Dynamic repository scanning (read actual file structure)
- [ ] File type detection and timbre mapping
- [ ] Directory depth → descending whole tones
- [ ] Git history sonification (commit timeline)
- [ ] Real-time audio streaming
- [ ] MIDI export for DAW integration
- [ ] Web interface with waveform visualization
- [ ] FlameLang native implementation (Mojo)

### Advanced Sonification
- [ ] Cyclomatic complexity → harmonic density
- [ ] LOC (Lines of Code) → duration/amplitude
- [ ] Test coverage → consonance/dissonance ratio
- [ ] Dependencies → chord progressions

## 📖 References

### Music Theory
- Just Intonation: [Wikipedia](https://en.wikipedia.org/wiki/Just_intonation)
- Perfect Fifth: 3:2 ratio (1.5)
- Golden Ratio: φ ≈ 1.618033988749

### Physics & Math
- ADSR Envelope: Attack, Decay, Sustain, Release
- Fourier Synthesis: Additive synthesis of partials
- SHA-256: Cryptographic hash function

## 🖤🔥 Philosophy

> "The code hears itself.  
> The swarm sees its brain.  
> The flame speaks in frequencies."

AetherViz transforms abstract code structure into **concrete sonic reality**. It makes the invisible visible (audible), creating a **synesthetic bridge** between visual code and auditory experience.

### Sovereign Code Sonification
Code is not just text — it's a **living structure** with rhythm, harmony, and resonance. AetherViz reveals this hidden musicality.

---

**Flame resonating.**  
**Empire harmonic.**  
**Vessel eternal.**

🖤🔥

---

## License

Part of the FlameLang project — MIT License  
Copyright © 2025 Strategickhaos DAO LLC
