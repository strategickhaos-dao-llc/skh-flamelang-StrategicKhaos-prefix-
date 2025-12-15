# AetherViz - 11-Limit Sonic Visualization

**Spectral visualization and audio synthesis for FlameLang's 11-limit interval system**

## Overview

AetherViz implements Harry Partch's 11-limit just intonation system with James Tenney's stochastic micro-detune, creating living, shimmering harmonic textures.

## Features

- **11-limit intervals**: Full Partch diamond with exotic ratios like 11/8 (551 cents)
- **Tenney stochastic detune**: ±0.5% shimmer at 0.3 Hz for organic, living sound
- **Multi-carrier FM synthesis**: 8 carriers with harmonic weighting
- **Spectral visualization**: Ratio lattice, waveforms, and frequency spectrum
- **Tonality diamond**: Both otonality (harmonic) and utonality (subharmonic) series

## Usage

### Requirements

```bash
pip install numpy matplotlib scipy
```

### Basic Usage

```bash
cd tools/aetherviz
python aetherviz_11limit.py
```

This will:
1. Print 11-limit interval information
2. Generate spectral visualization (PNG)
3. Generate audio sample (WAV) if scipy is available

### Import as Module

```python
from aetherviz_11limit import (
    generate_11limit_chord,
    tenney_detune,
    visualize_11limit_spectrum,
    generate_tonality_diamond
)

# Generate 4-second chord
t, signal = generate_11limit_chord(duration=4.0)

# Create visualization
visualize_11limit_spectrum(save_path="spectrum.png")

# Get tonality diamond frequencies
otonality, utonality = generate_tonality_diamond(fundamental=110.0)
```

## 11-Limit Intervals

Key intervals in the 11-limit system:

- **11/8** (551 cents) - "undecimal neutral fourth" - floating, otherworldly
- **11/10** (165 cents) - narrow major second
- **11/9** - undecimal neutral third
- **11/6** - undecimal neutral seventh

## Theory

### Partch's Tonality Diamond

Harry Partch (1901-1974) developed the concept of the tonality diamond, a lattice structure of just intonation intervals. The 11-limit diamond includes all intervals using prime factors up to 11.

### Tenney's Stochastic Detune

James Tenney (1934-2006) extended microtonal theory with:
- **Harmonic space**: Multi-dimensional tuning lattices
- **Harmonic distance**: Perceptual dissonance measurement
- **Stochastic elements**: Living, breathing harmonies

## Integration with FlameLang

AetherViz is part of FlameLang's Layer 3 (Wave Transform):

```
English → Hebrew → Unicode → Wave → DNA → LLVM
                              ↑
                         AetherViz
                      (11-limit sonics)
```

## Philosophy

> "The swarm sings in primes beyond human hearing.  
> We have transcended temperament.  
> Flame infinite. Empire spectral. Vessel eternal."

## License

MIT License - Part of FlameLang sovereign compiler toolchain
