#!/usr/bin/env python3
"""
AetherViz Ultra - Pure Just Intonation + MIDI
Ben Johnston Notation Implementation for FlameLang

This module generates audio using extended just intonation with Ben Johnston's
notation system. It creates both WAV and MIDI files with pure harmonic ratios.

Kappa Methodology Integration:
- Water control → constraint-based flow (7-limit ratios)
- Dish on head → critical resource (sample rate precision)
- Cucumber offering → charity trigger (7% glissando probability)
- Bridge building → connective transformation layers (stereo panning)

Ratio Ex Nihilo | Strategickhaos DAO LLC © 2025
"""

import numpy as np
import wave
import hashlib
import struct
from pathlib import Path

try:
    import mido
    from mido import MidiFile, MidiTrack, Message, MetaMessage
    MIDO_AVAILABLE = True
except ImportError:
    MIDO_AVAILABLE = False
    print("⚠️  Warning: mido not installed. MIDI generation disabled.")
    print("   Install with: pip install mido")

# Ben Johnston 7-limit just intonation ratios
# Based on harmonic series with syntonic comma adjustments
RATIOS = [
    1,      # Unison (C)
    9/8,    # Major second (D) - Pythagorean whole tone
    5/4,    # Major third (E) - Just intonation major third
    4/3,    # Perfect fourth (F)
    3/2,    # Perfect fifth (G) - Pure fifth
    5/3,    # Major sixth (A) - Just intonation major sixth
    15/8,   # Major seventh (B) - Just major seventh
]

# Audio configuration
SAMPLE_RATE = 44100
BASE_FREQ = 110.0  # A2 base frequency
DURATION = 8.0  # seconds

# Kappa methodology: carrier frequencies from harmonic ratios
CARRIERS = [BASE_FREQ * r for r in RATIOS]

# MIDI configuration
MIDI_BASE = 45  # A2 MIDI note number

# Note sequences for harmonic exploration
# Each sublist represents a chord or harmonic moment
NOTES = [
    [0, 7, 0],           # Root emphasis
    [7, 12, 10],         # Fifth, octave, sixth
    [10, 0, 7, 12],      # Sixth, root, fifth, octave
    [0],                 # Root solo
    [7, 12],             # Fifth and octave
    [10, 4, 9]           # Sixth, third, complex harmony
]

# Stereo panning matrix (bridge building - connective layers)
# Each carrier gets a different stereo position
PANNING = np.array([
    [0.5, 0.5],  # Center
    [0.7, 0.3],  # Right bias
    [0.3, 0.7],  # Left bias
    [0.8, 0.2],  # Strong right
    [0.2, 0.8],  # Strong left
    [0.6, 0.4],  # Slight right
])

# Kappa offering rate (cucumber tribute)
CHARITY_RATE = 0.07  # 7% probability for glissando


def triangle_wave(freq, t):
    """
    Generate a triangle wave for modulation.
    
    Args:
        freq: Frequency of the triangle wave
        t: Time array
        
    Returns:
        Triangle wave signal
    """
    return 2 * np.abs(2 * ((freq * t) % 1.0) - 1) - 1


def adsr_envelope(t, attack=0.3, decay=0.5, sustain=0.7, release=2.0):
    """
    Generate ADSR (Attack, Decay, Sustain, Release) envelope.
    
    This implements the "water flow" constraint - gradual power buildup
    and graceful release, like the kappa's dish water.
    
    Args:
        t: Time array
        attack: Attack time multiplier
        decay: Decay time multiplier
        sustain: Sustain level (0-1)
        release: Release time multiplier
        
    Returns:
        Envelope array
    """
    envelope = np.ones_like(t)
    
    # Attack phase: exponential rise (water filling dish)
    attack_phase = 1 - np.exp(-t * attack)
    
    # Release phase: exponential decay (water spilling)
    release_phase = np.exp(-t * release)
    
    return attack_phase * release_phase


def render_carrier(carrier, duration=DURATION, sample_rate=SAMPLE_RATE):
    """
    Render a single carrier frequency with Ben Johnston pure ratio.
    
    Kappa methodology: Each carrier represents a bridge pillar.
    The 7% charity trigger adds glissando modulation.
    
    Args:
        carrier: Carrier frequency (Hz)
        duration: Duration in seconds
        sample_rate: Audio sample rate
        
    Returns:
        Rendered audio signal
    """
    t = np.linspace(0, duration, int(sample_rate * duration), False)
    
    # Base sinusoid - pure harmonic
    sig = np.sin(2 * np.pi * carrier * t)
    
    # Kappa offering: 7% chance of glissando modulation
    if np.random.rand() < CHARITY_RATE:
        # Add triangle wave modulation with frequency sweep
        # This creates the "cucumber offering" effect
        modulator_freq = carrier * 1.5
        freq_sweep = t * 110  # Frequency increases over time
        sig += triangle_wave(modulator_freq + freq_sweep, t) * 0.38
    
    # Apply ADSR envelope (water control)
    envelope = adsr_envelope(t, attack=3, decay=0.5, release=0.5)
    
    return sig * envelope


def generate_audio(output_path="aetherviz_output.wav"):
    """
    Generate the complete audio mix with stereo panning.
    
    This is the "bridge building" phase - connecting all harmonic
    elements into a unified structure.
    
    Args:
        output_path: Path for output WAV file
        
    Returns:
        Stereo mix array and SHA256 hash
    """
    print("🔥 Generating AetherViz audio with Ben Johnston ratios...")
    print(f"   Base frequency: {BASE_FREQ} Hz (A2)")
    print(f"   7-limit just intonation ratios: {RATIOS}")
    print(f"   Carrier frequencies: {[f'{c:.2f} Hz' for c in CARRIERS]}")
    print(f"   Kappa charity rate: {CHARITY_RATE * 100}%")
    
    # Render all carriers
    rendered_carriers = []
    for i, carrier in enumerate(CARRIERS):
        print(f"   Rendering carrier {i+1}/{len(CARRIERS)}: {carrier:.2f} Hz (ratio {RATIOS[i]})")
        rendered_carriers.append(render_carrier(carrier))
    
    # Create stereo mix with panning
    # Each carrier gets its own position in the stereo field
    stereo_mix = np.zeros((len(rendered_carriers[0]), 2))
    
    for i, carrier_signal in enumerate(rendered_carriers):
        # Apply panning for this carrier
        panning = PANNING[i % len(PANNING)]
        stereo_mix[:, 0] += carrier_signal * panning[0]
        stereo_mix[:, 1] += carrier_signal * panning[1]
    
    # Normalize to prevent clipping
    max_val = np.max(np.abs(stereo_mix))
    if max_val > 0:
        stereo_mix = stereo_mix / max_val * 0.9  # Leave 10% headroom
    
    # Calculate hash for verification (water dish integrity check)
    mix_hash = hashlib.sha256(stereo_mix.tobytes()).hexdigest()
    
    # Write WAV file
    print(f"\n💾 Writing WAV file: {output_path}")
    write_wav(output_path, stereo_mix)
    
    print(f"✅ Audio generation complete!")
    print(f"🔐 SHA256 Hash: {mix_hash}")
    
    return stereo_mix, mix_hash


def write_wav(filename, audio_data, sample_rate=SAMPLE_RATE):
    """
    Write stereo audio data to WAV file.
    
    Args:
        filename: Output filename
        audio_data: Stereo audio array (N, 2)
        sample_rate: Sample rate in Hz
    """
    # Convert to 16-bit PCM
    audio_data_int = np.int16(audio_data * 32767)
    
    with wave.open(filename, 'w') as wav_file:
        # Set parameters: 2 channels (stereo), 2 bytes per sample, sample rate
        wav_file.setnchannels(2)
        wav_file.setsampwidth(2)
        wav_file.setframerate(sample_rate)
        
        # Write interleaved stereo data
        wav_file.writeframes(audio_data_int.tobytes())


def generate_midi(output_path="aetherviz_output.mid"):
    """
    Generate MIDI file with note sequences based on harmonic ratios.
    
    Ben Johnston notation cannot be fully represented in standard MIDI,
    but we approximate the harmonic structure.
    
    Args:
        output_path: Path for output MIDI file
    """
    if not MIDO_AVAILABLE:
        print("⚠️  MIDI generation skipped (mido not installed)")
        return
    
    print(f"\n🎹 Generating MIDI file: {output_path}")
    
    mid = MidiFile()
    track = MidiTrack()
    mid.tracks.append(track)
    
    # Add track name
    track.append(MetaMessage('track_name', name='AetherViz - Ben Johnston JI', time=0))
    
    # Set tempo (120 BPM)
    track.append(MetaMessage('set_tempo', tempo=500000, time=0))
    
    # Generate note sequences
    ticks_per_beat = 480
    beat_duration = ticks_per_beat
    
    for note_group in NOTES:
        # Note on messages
        for offset in note_group:
            note = MIDI_BASE + offset
            track.append(Message('note_on', note=note, velocity=80, time=0))
        
        # Hold for one beat
        track.append(Message('note_on', note=0, velocity=0, time=beat_duration))
        
        # Note off messages
        for offset in note_group:
            note = MIDI_BASE + offset
            track.append(Message('note_off', note=note, velocity=0, time=0))
        
        # Gap before next chord
        track.append(Message('note_on', note=0, velocity=0, time=beat_duration // 2))
    
    # Save MIDI file
    mid.save(output_path)
    print(f"✅ MIDI file generated with {len(NOTES)} harmonic moments")


def display_johnston_notation():
    """
    Display Ben Johnston notation explanation for the ratios used.
    """
    print("\n" + "=" * 70)
    print("BEN JOHNSTON NOTATION - 7-LIMIT JUST INTONATION")
    print("=" * 70)
    print("\nRatio Interpretation (from base A2 = 110 Hz):")
    print("-" * 70)
    
    notes = ["C", "D", "E", "F", "G", "A", "B"]
    johnston_symbols = [
        "1/1 (Unison, no accidentals)",
        "9/8 (Pythagorean major second)",
        "5/4 (Just major third, syntonic comma below Pythagorean)",
        "4/3 (Perfect fourth)",
        "3/2 (Perfect fifth, pure)",
        "5/3 (Just major sixth)",
        "15/8 (Just major seventh)"
    ]
    
    for i, (ratio, freq, notation) in enumerate(zip(RATIOS, CARRIERS, johnston_symbols)):
        print(f"{notes[i]:2} | {ratio:6.4f} | {freq:7.2f} Hz | {notation}")
    
    print("-" * 70)
    print("\nKappa Methodology Applied:")
    print("  🌊 Water control    → Constraint-based 7-limit ratios")
    print("  🍽️  Dish on head    → Critical resource (sample precision)")
    print("  🥒 Cucumber tribute → 7% glissando probability")
    print("  🌉 Bridge building  → Stereo panning transformations")
    print("=" * 70 + "\n")


def main():
    """
    Main execution function - The kappa builds its bridge.
    """
    print("\n" + "🔥" * 35)
    print("   AETHERVIZ ULTRA - BEN JOHNSTON PURE RATIOS")
    print("   Strategickhaos DAO LLC | FlameLang v2.0")
    print("🔥" * 35 + "\n")
    
    # Display notation information
    display_johnston_notation()
    
    # Set output directory
    output_dir = Path(__file__).parent
    wav_path = output_dir / "aetherviz_output.wav"
    midi_path = output_dir / "aetherviz_output.mid"
    
    # Generate audio
    mix, hash_value = generate_audio(str(wav_path))
    
    # Generate MIDI
    generate_midi(str(midi_path))
    
    print("\n" + "🔥" * 35)
    print("   GENERATION COMPLETE")
    print("   The flame now sings in pure ratios — Johnston-approved.")
    print("   Kappa methodology: constraint + offering = power.")
    print("   We bow to the water — and build unbreakable bridges.")
    print("🔥" * 35 + "\n")
    
    print(f"📁 Output files:")
    print(f"   WAV:  {wav_path}")
    print(f"   MIDI: {midi_path}")
    print(f"\n🎧 Play the WAV file to hear the harmonic resonance!")
    print(f"🎹 Import the MIDI file into your DAW for further exploration.\n")
    
    print("Flame pure. 🔥")
    print("Empire resonant. 🌊")
    print("Vessel infinite. 河童\n")


if __name__ == "__main__":
    main()
