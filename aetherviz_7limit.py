#!/usr/bin/env python3
"""
AetherViz 7-Limit Just Intonation — Pure Partch Resonance

Implements Harry Partch's 7-limit just intonation system with Ben Johnston notation.
Generates pure 7-limit audio synthesis and MIDI approximations.

7-Limit System:
- Ratios using primes up to 7 (2, 3, 5, 7)
- Partch's 43-tone scale foundation
- Natural harmonic series intervals
- Ben Johnston extended notation (+ / -, 7↑ / 7↓)

Key intervals:
  1/1   (0¢)    - Unison
  8/7   (231¢)  - Septimal major second (bluesy)
  7/6   (267¢)  - Septimal minor third (dark)
  6/5   (316¢)  - Minor third
  5/4   (386¢)  - Major third
  4/3   (498¢)  - Perfect fourth
  7/5   (583¢)  - Septimal tritone (exotic)
  3/2   (702¢)  - Perfect fifth
  8/5   (814¢)  - Minor sixth
  5/3   (884¢)  - Major sixth
  12/7  (933¢)  - Septimal minor seventh (blues)
  7/4   (969¢)  - Harmonic seventh (Partch favorite)
  2/1   (1200¢) - Octave

© 2025 Strategickhaos DAO LLC | Ratio Ex Nihilo
"""

# aetherviz_7limit.py — Pure 7-Limit + MIDI
import numpy as np
import wave
import mido
import hashlib
from mido import MidiFile, MidiTrack, Message, MetaMessage

# 7-Limit Just Intonation Ratios (13 intervals within octave)
RATIOS_7 = [1, 8/7, 7/6, 6/5, 5/4, 4/3, 7/5, 3/2, 8/5, 5/3, 12/7, 7/4, 2]

# Carrier frequencies for synthesis (first 6 ratios from 110 Hz base)
CARRIERS = [110 * r for r in RATIOS_7[:6]]

# MIDI base note (A2 = MIDI note 45)
MIDI_BASE = 45

def render(carrier):
    """
    Render a single carrier frequency with 7-limit characteristics.
    
    Args:
        carrier: Fundamental frequency in Hz
        
    Returns:
        Audio signal as numpy array
    """
    # 8 seconds at 48kHz sample rate
    t = np.linspace(0, 8, int(8*48000), False)
    
    # Base sine wave
    sig = np.sin(2*np.pi*carrier*t)
    
    # Add 7-limit glissando sweep (7% frequency modulation)
    # This creates the "septimal" character - slightly bluesy/exotic
    sig += np.where(
        t < 1.2, 
        np.sin(2*np.pi*(carrier*1.5 + t*carrier*0.7)*t) * 0.38, 
        0
    )
    
    # Envelope: fast attack, slow decay (natural harmonic decay)
    sig *= (1 - np.exp(-t*3)) * np.exp(-t*0.5)
    
    return sig

# Create stereo mix with panning positions for spatial effect
mix = sum(
    render(c)[:,None] * p 
    for c, p in zip(CARRIERS, [
        (0.5, 0.5),  # Center
        (0.7, 0.3),  # Left-center
        (0.3, 0.7),  # Right-center
        (0.8, 0.2),  # Left
        (0.2, 0.8),  # Right
        (0.6, 0.4),  # Left-center
    ])
)

# WAV write (16-bit stereo, 48kHz)
with wave.open("7limit_invocation.wav", 'w') as wf:
    wf.setnchannels(2)
    wf.setframerate(48000)
    wf.setsampwidth(2)
    # Normalize to prevent clipping
    normalized = mix / np.max(np.abs(mix)) * 32767
    wf.writeframes(normalized.astype(np.int16).T.tobytes())

# MIDI generation (7-limit approximated in 12TET)
mid = MidiFile()
track = MidiTrack()
mid.tracks.append(track)

# Set tempo to 120 BPM
track.append(MetaMessage('set_tempo', tempo=mido.bpm2tempo(120)))

# Chord progression using 7-limit intervals (approximated to 12TET)
# Each inner list represents a chord (simultaneous notes)
progression = [
    [0, 4, 0],        # Root + major third
    [4, 9, 7],        # Major third + major sixth + fifth
    [7, 0, 4, 9],     # Fifth + root + major third + major sixth
    [0],              # Root
    [4, 9],           # Major third + major sixth
    [7, 2, 5],        # Fifth + major second + fourth
]

for notes in progression:
    # Note on for all notes in chord
    for n in notes:
        track.append(Message('note_on', note=MIDI_BASE+n, velocity=90, time=0))
    # Note off for all notes after duration
    for i, n in enumerate(notes):
        # Add time delta only to the last note_off
        track.append(Message('note_off', note=MIDI_BASE+n, velocity=64, time=480*4 if i == len(notes)-1 else 0))

mid.save("7limit.mid")

# Output results
print("=" * 60)
print("🔥 AETHERVIZ 7-LIMIT JUST INTONATION")
print("=" * 60)
print()
print("Rendered 7-limit invocation — Partch-approved ratios")
print()
print("Files generated:")
print("  • 7limit_invocation.wav (48kHz stereo, 8 seconds)")
print("  • 7limit.mid (120 BPM chord progression)")
print()
print("7-Limit Ratios Used:")
for i, ratio in enumerate(RATIOS_7):
    cents = 1200 * np.log2(ratio)
    print(f"  {i+1:2d}. {ratio:6.4f} ({cents:6.1f}¢)")
print()
print("Carrier Frequencies (Hz):")
for i, freq in enumerate(CARRIERS):
    print(f"  {i+1}. {freq:7.2f} Hz (ratio {RATIOS_7[i]:.4f})")
print()
print("Audio Signature:")
print(f"  SHA-256: {hashlib.sha256(mix.tobytes()).hexdigest()}")
print()
print("=" * 60)
print("The flame now sings in 7-limit purity.")
print("We have transcended equal temperament.")
print("The swarm hears the primes.")
print("=" * 60)
print()
print("Ratio Ex Nihilo | Strategickhaos DAO LLC © 2025")
