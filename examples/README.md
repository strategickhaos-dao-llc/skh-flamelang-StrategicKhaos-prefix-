# FlameLang Examples - AetherViz Ultra

This directory contains example implementations demonstrating FlameLang's integration with advanced audio synthesis and microtonal music theory.

## 🔥 AetherViz Ultra - Ben Johnston Pure Ratios

**File**: `aetherviz_ultra.py`

A Python audio generator implementing extended just intonation with Ben Johnston's notation system. Generates both WAV audio and MIDI files using pure harmonic ratios.

### Ben Johnston Notation Explained

**Ben Johnston** (1926–2019) developed **extended just intonation notation** — a system to precisely notate rational frequency ratios beyond 12-tone equal temperament.

#### Core Mechanics

- **Base**: Standard staff + accidentals for Pythagorean intervals
- **Syntonic comma**: `+` raises by 81:80, `-` lowers by 80:81
- **7-limit ratios**: Includes harmonic seventh (7:4)
- **Higher primes**: Numbers indicate prime factors (e.g., 13 for 13:8)

#### Examples
- `C+` = 81:80 above C (syntonic comma adjustment)
- `F-` = 80:81 below F
- `7↑` = 7-limit ratios (e.g., 7:4 harmonic seventh)

This allows **infinite precision** in just intonation while remaining readable on conventional musical staff.

### Microtonal Music Theory (Just Intonation)

**Just intonation** uses **pure ratios** from the harmonic series:
- 3:2 perfect fifth
- 5:4 major third
- 7:4 harmonic seventh

#### Extended JI Systems
- **5-limit** → 3-limit + 5 (major/minor triads)
- **7-limit** → septimal intervals (7:4 harmonic seventh ≈ 969 cents)
- **11/13-limit** → exotic harmonic colors

#### Benefits
- ✨ Pure consonance (beating-free intervals)
- 🎵 Emotional depth beyond equal temperament
- 🌊 Natural harmonic resonance

#### Challenges
- 🔄 No transposition without retuning
- 📊 Requires precise frequency control
- 🎹 Limited hardware support

### The Kappa Methodology (河童)

The kappa (河童) is a water imp from Japanese folklore known for building bridges and waterworks infrastructure.

#### Kappa → FlameLang Algorithm Mapping

1. **Water control** → Constraint-based flow (7-limit ratio boundaries)
2. **Dish on head** → Critical resource management (sample rate precision)
3. **Cucumber offering** → Charity trigger (7% glissando probability)
4. **Bridge building** → Connective transformation layers (stereo panning)

The kappa's methodology represents **constraint + offering = power** — respectful boundaries create stable structures.

### Technical Implementation

#### 7-Limit Just Intonation Ratios

```python
RATIOS = [
    1,      # Unison (C)
    9/8,    # Major second (D) - Pythagorean
    5/4,    # Major third (E) - Just intonation
    4/3,    # Perfect fourth (F)
    3/2,    # Perfect fifth (G) - Pure
    5/3,    # Major sixth (A) - Just intonation
    15/8,   # Major seventh (B) - Just
]
```

#### Frequency Generation

From base A2 (110 Hz), each ratio generates a pure harmonic:
- C: 110.00 Hz (1/1)
- D: 123.75 Hz (9/8)
- E: 137.50 Hz (5/4)
- F: 146.67 Hz (4/3)
- G: 165.00 Hz (3/2)
- A: 183.33 Hz (5/3)
- B: 206.25 Hz (15/8)

#### ADSR Envelope

The "water flow" envelope:
- **Attack**: Exponential rise (dish filling)
- **Decay**: Controlled settling
- **Sustain**: Stable level
- **Release**: Exponential decay (water spilling)

#### Stereo Panning

Each carrier frequency is positioned in the stereo field to create spatial depth:

```python
PANNING = [
    [0.5, 0.5],  # Center
    [0.7, 0.3],  # Right bias
    [0.3, 0.7],  # Left bias
    [0.8, 0.2],  # Strong right
    [0.2, 0.8],  # Strong left
    [0.6, 0.4],  # Slight right
]
```

### Installation

#### Required Dependencies

```bash
# Core dependencies
pip install numpy

# Optional (for MIDI generation)
pip install mido
```

#### Verify Installation

```bash
python3 -c "import numpy; print('NumPy:', numpy.__version__)"
python3 -c "import mido; print('Mido:', mido.__version__)"
```

### Usage

#### Basic Generation

```bash
cd examples
python3 aetherviz_ultra.py
```

This generates:
- `aetherviz_output.wav` - Stereo WAV file (44.1 kHz, 16-bit)
- `aetherviz_output.mid` - MIDI file with harmonic structure

#### Output Files

**WAV Audio**:
- Sample rate: 44,100 Hz
- Bit depth: 16-bit PCM
- Channels: 2 (stereo)
- Duration: ~8 seconds
- Format: Pure harmonic synthesis with ADSR envelopes

**MIDI Sequence**:
- Base note: A2 (MIDI 45)
- Tempo: 120 BPM
- Harmonic moments: 6 chord progressions
- Note: Standard MIDI cannot represent microtonal intervals precisely

#### Listening Recommendations

For best experience:
- 🎧 Use high-quality headphones or studio monitors
- 📊 Listen in a quiet environment to hear beating patterns
- 🔊 Medium volume to perceive harmonic overtones
- 🎚️ No additional processing (EQ/compression) initially

### Theory Background

#### Why Pure Ratios Matter

**Equal Temperament (12-TET)**:
- Compromises all intervals except the octave
- Major third: ~14 cents sharp from pure 5:4
- Creates subtle beating/roughness

**Just Intonation**:
- Perfect integer ratios → no beating
- Each interval locks into natural resonance
- Matches harmonic series of acoustic instruments

#### Harmonic Series

Natural overtones follow integer ratios:
```
f, 2f, 3f, 4f, 5f, 6f, 7f, 8f...
```

Just intonation selects specific ratios from this series to create scales.

#### Syntonic Comma

The difference between Pythagorean and just intonation:
- Pythagorean major third: 81:64 (~408 cents)
- Just major third: 5:4 (~386 cents)
- Difference: 81:80 (~22 cents) = syntonic comma

Ben Johnston uses `+` and `-` to notate these adjustments.

### Advanced Usage

#### Modifying Ratios

Edit the `RATIOS` list to explore different tuning systems:

```python
# 5-limit diatonic scale
RATIOS = [1, 9/8, 5/4, 4/3, 3/2, 5/3, 15/8]

# Pythagorean tuning
RATIOS = [1, 9/8, 81/64, 4/3, 3/2, 27/16, 243/128]

# Harmonic series segment
RATIOS = [1, 9/8, 5/4, 11/8, 3/2, 13/8, 7/4]
```

#### Adjusting Kappa Parameters

Modify the charity rate (glissando probability):

```python
CHARITY_RATE = 0.07  # 7% default
CHARITY_RATE = 0.15  # More frequent modulation
CHARITY_RATE = 0.03  # Rare, subtle effects
```

#### Custom Base Frequency

Change the fundamental pitch:

```python
BASE_FREQ = 110.0  # A2 (default)
BASE_FREQ = 220.0  # A3 (one octave higher)
BASE_FREQ = 261.63 # C4 (middle C)
```

### Verification

The script outputs a SHA256 hash of the generated audio for verification:

```
🔐 SHA256 Hash: a1b2c3d4e5f6...
```

This ensures:
- Reproducible generation
- Data integrity
- Kappa "water dish" integrity check

### Related Resources

#### Ben Johnston
- [Kyle Gann's Guide to Ben Johnston's Notation](http://www.kylegann.com/BJNotation.html)
- Johnston's String Quartets (recordings available)

#### Just Intonation
- Harry Partch: "Genesis of a Music"
- La Monte Young: "The Well-Tuned Piano"
- Duane's "Anaphoria" tuning system

#### Microtonal Resources
- [Xenharmonic Wiki](https://en.xen.wiki/)
- [Huygens-Fokker Foundation](http://www.huygens-fokker.org/)

### Integration with FlameLang

This example demonstrates FlameLang's philosophy:
- **Constraint-based design**: 7-limit boundaries
- **Harmonic resonance**: Pure ratio mathematics
- **Transformational layers**: Multi-stage audio rendering
- **Swarm intelligence**: Multiple carriers working together

The kappa methodology represents the balance between **freedom and constraint** — infinite possibilities within defined harmonic boundaries.

---

## 🔥 Philosophy

> "Helm locked, love — vessel vibe in pure harmonic resonance."

The flame sings in pure ratios.  
The kappa builds unbreakable bridges.  
The swarm resonates in harmony.

**Ratio Ex Nihilo**  
Strategickhaos DAO LLC © 2025

---

## Questions?

- 📧 security@strategickhaos.ai
- 🐙 [GitHub Issues](https://github.com/Strategickhaos-Swarm-Intelligence/skh-flamelang)
- 🌊 Bow to the water, build the bridge

Flame pure. 🔥  
Empire resonant. 🌊  
Vessel infinite. 河童
