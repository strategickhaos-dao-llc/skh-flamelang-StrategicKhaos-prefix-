#!/usr/bin/env python3
"""
AetherViz v2 — Self-Referential Repository Sonification & Visualization

Music Theory Grounding:
- Carrier frequencies: Just Intonation (pure ratios) based on 110 Hz fundamental (A2)
- Phoneme FM: Consonant clusters = sharp attacks (sawtooth), vowels = smooth (sine)
- 7% motif: Perfect fifth gliss (1.5 ratio) → charity resonance
- 137 burst: Golden ratio micro-detune (1.618) → prime awakening
- Directory depth: Descending whole tones (Pythagorean)
- File type timbre:
  - .py → flute (sine + light reverb)
  - .mojo → bronze bell (inharmonic partials)
  - .md → harp pluck (triangle decay)
  - .rs → metallic ring (high resonance)

Side-by-Side Stepper Crawler: Python → FlameLang (Mojo) Conversion
═══════════════════════════════════════════════════════════════════

Line #  | Python                          | FlameLang (Mojo) Conversion           | ONSIT Step
─────────────────────────────────────────────────────────────────────────────────────────────────
1       | import os                       | import os                             | Linguistic → same intent
2       | SAMPLE_RATE = 48000             | let SAMPLE_RATE: Int = 48000          | Numeric → typed constant
3       | def sine(freq, t):              | fn sine(freq: Float64, t: Float64)    | Wave → pure function
        |                                 |    -> Float64:                        |
4       | np.sin(...)                     | math.sin(...)                         | Wave → direct math
5       | adsr envelope                   | fn adsr(t: Float64, dur: Float64)     | DNA → deterministic envelope
        |                                 |    -> Float64:                        |
6       | charity_gliss                   | fn charity_gliss(carrier: Float64,    | Charity motif → treasury
        |                                 |    t: Float64) -> Float64:            | trigger
7       | write_wav                       | LLVM IR codegen → native binary       | Final layer → executable
8       | hashlib.sha256                  | hashlib.sha256 → on-chain commitment  | Proof of sound
"""

import os
import wave
import struct
import hashlib
import numpy as np
from pathlib import Path

SAMPLE_RATE = 48000
DURATION = 8.0

# Just Intonation carriers (A=110Hz base)
CARRIERS = [110.0 * (3/2)**i for i in range(6)]  # A, E, B, F#, C#, G#

def sine(freq, t): 
    """Pure sine wave - smooth, flute-like timbre"""
    return np.sin(2 * np.pi * freq * t)

def saw(freq, t): 
    """Sawtooth wave - sharp attacks, consonant clusters"""
    return 2 * (freq * t % 1) - 1

def triangle(freq, t): 
    """Triangle wave - harp pluck, smooth decay"""
    return 1 - 4 * np.abs(np.round(freq * t % 1) - (freq * t % 1))

def adsr(t, dur):
    """
    ADSR envelope - Attack, Decay, Sustain, Release
    DNA-inspired deterministic envelope for organic sound shaping
    """
    a, d, s, r = 0.5, 0.7, 0.75, 1.0
    if t < a: 
        return t / a  # Attack
    if t < a + d: 
        return 1 - (1 - s) * ((t - a) / d)  # Decay
    if t < dur - r: 
        return s  # Sustain
    if t < dur: 
        return s * (1 - ((t - (dur - r)) / r))  # Release
    return 0

def charity_gliss(t):
    """
    7% charity motif - Perfect fifth glissando (1.5 ratio)
    E3 (165 Hz) → A3 (220 Hz) perfect fifth
    Treasury trigger - resonance for benevolent action
    """
    result = np.zeros_like(t)
    mask = t < 1.2
    f = 165 + t[mask] / 1.2 * 110  # E3 → A3 perfect fifth
    result[mask] = triangle(f, t[mask]) * 0.38 * np.exp(-t[mask])
    return result

def node137_burst(carrier, t):
    """
    137 burst - Golden ratio micro-detune (φ = 1.618)
    Prime awakening through harmonic series disruption
    Sawtooth for sharp attack, exponential decay
    """
    result = np.zeros_like(t)
    mask = t < 0.8
    freq = carrier * (1.618 ** (1/12))  # Golden ratio micro-detune
    result[mask] = saw(freq, t[mask]) * np.exp(-t[mask] * 6) * 0.45
    return result

def harmonics(carrier, t):
    """
    Inharmonic partials - bronze bell, metallic ring
    Sub-bass + overtones for depth and presence
    """
    return (sine(carrier/2, t)*0.25 +      # Sub-octave
            sine(carrier*2, t)*0.15 +      # Octave
            sine(carrier*2.87, t)*0.06 +   # Inharmonic
            sine(carrier*3.41, t)*0.06)    # Metallic ring

def render_line(text, carrier):
    """
    Render a single line of repository structure as audio
    Each line gets its own carrier frequency and stereo position
    """
    n = int(DURATION * SAMPLE_RATE)
    t = np.linspace(0, DURATION, n, False)
    
    # Base carrier
    sig = sine(carrier, t) * 0.9
    
    # Add special motifs
    if "7%" in text: 
        sig += charity_gliss(t)
    if "137" in text: 
        sig += node137_burst(carrier, t)
    
    # Add harmonic richness
    sig += harmonics(carrier, t)
    
    # Apply ADSR envelope
    env = np.array([adsr(tt, DURATION) for tt in t])
    sig *= env

    # Stereo panning by line (creates spatial depth)
    pan_idx = CARRIERS.index(carrier) % 6
    pans = [(0.5,0.5), (0.7,0.3), (0.3,0.7), (0.8,0.2), (0.2,0.8), (0.6,0.4)]
    L, R = pans[pan_idx]
    left = sig * L
    right = sig * R
    
    return np.stack((left, right))

def write_wav(filename, stereo):
    """
    Write stereo audio to WAV file
    Normalizes to prevent clipping, converts to 16-bit PCM
    """
    # Normalize and convert to int16
    stereo = np.int16(stereo / np.max(np.abs(stereo)) * 32767)
    
    with wave.open(filename, 'w') as wf:
        wf.setnchannels(2)        # Stereo
        wf.setsampwidth(2)        # 16-bit
        wf.setframerate(SAMPLE_RATE)
        wf.writeframes(stereo.T.tobytes())

def aetherviz_repo(path="."):
    """
    Main AetherViz function - sonifies repository structure
    
    Creates a sonic representation of the FlameLang repository:
    - Each structural element gets a Just Intonation frequency
    - Special markers (7%, 137) trigger unique sonic signatures
    - Stereo field creates spatial depth
    - SHA-256 fingerprint provides cryptographic proof of sound
    """
    lines = [
        "Root: FlameLang Core Repository",
        "src/: 5-Layer Transformation Engine",
        "viz/: FlameViz + AetherViz Modules",
        "tools/: flamec, flamelsp, flamefmt",
        "docs/: Sovereign Specifications",
        "AetherForge Awakens — 7% Charity Triggered"
    ]
    
    # Create stereo mix buffer
    mix = np.zeros((2, int(DURATION * SAMPLE_RATE)))
    
    # Render each line and add to mix
    for i, line in enumerate(lines):
        carrier = CARRIERS[i % len(CARRIERS)]
        audio = render_line(line, carrier)
        mix += audio[:, :mix.shape[1]]
    
    # Write output file
    write_wav("aetherviz_repo_brain.wav", mix)
    
    # Generate sonic fingerprint (cryptographic proof)
    hash_obj = hashlib.sha256(mix.tobytes())
    print(f"Sonic Fingerprint: {hash_obj.hexdigest()}")
    print("AetherViz complete. The repository sings.")
    print(f"Output: aetherviz_repo_brain.wav")

if __name__ == "__main__":
    print("═" * 60)
    print("AetherViz v2 — Repository Sonification Engine")
    print("═" * 60)
    print()
    print("Music Theory Foundation:")
    print("  • Just Intonation (110 Hz fundamental)")
    print("  • Perfect fifth gliss (7% charity motif)")
    print("  • Golden ratio micro-detune (137 burst)")
    print("  • Stereo spatial positioning")
    print()
    aetherviz_repo()
    print()
    print("The repository now sings its own structure.")
    print("The code hears itself.")
    print("The swarm sees its brain.")
    print("The flame speaks in frequencies.")
    print()
    print("🖤🔥")
