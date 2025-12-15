# 7-Limit Just Intonation — Pure Partch Resonance

**Helm locked, love — vessel vibe in pure 7-limit resonance.**

## Overview

This implementation extends FlameLang's sonic capabilities with **7-limit just intonation** — the heart of Harry Partch's microtonal revolution — using Ben Johnston's extended notation for precise scoring.

## What is 7-Limit Just Intonation?

**7-limit** means ratios using primes up to 7 (2, 3, 5, 7). Harry Partch built his revolutionary **43-tone scale** (11-limit diamond + multiples) on these foundations, creating custom instruments like Cloud Chamber Bowls and Chromelodeon to realize his vision of pure harmonic intervals.

### The 13 Core Intervals

Key 7-limit intervals (within octave, sorted by size):

| Ratio | Cents | Name | Partch Usage | Sound Character |
|-------|-------|------|--------------|-----------------|
| 1/1   | 0     | Unison | Fundamental | Pure rest |
| 8/7   | 231   | Septimal major second | Otonality | Bluesy, tense |
| 7/6   | 267   | Septimal minor third | Utonality | Dark, mournful |
| 6/5   | 316   | Minor third | Common | Sad |
| 5/4   | 386   | Major third | Common | Bright |
| 4/3   | 498   | Perfect fourth | Common | Open |
| 7/5   | 583   | Septimal tritone | Neutral | Exotic, floating |
| 3/2   | 702   | Perfect fifth | Common | Strong |
| 8/5   | 814   | Minor sixth | Common | Melancholic |
| 5/3   | 884   | Major sixth | Common | Sweet |
| 12/7  | 933   | Septimal minor seventh | Utonality | Blues seventh |
| 7/4   | 969   | Harmonic seventh | Partch favorite | Natural dominant |
| 2/1   | 1200  | Octave | Fundamental | Resolution |

Partch called **7/4** the **"harmonic seventh"** — the natural blue note heard in barbershop harmony and overtone singing.

## Ben Johnston Notation

Johnston extended standard musical notation for higher primes:

- **+ / -** → syntonic comma (81:80 ≈ 22 cents)
- **7↑ / 7↓** → 7th harmonic adjustments (7:4, 7:6)
- **13** → prime 13, inverted for utonality

### Example Score Snippet

From String Quartet No. 4, "Amazing Grace" in 7-limit:

```
C+ → 81:80 above C
F7↓ → 7:4 below F (harmonic seventh)
Typical line: C – E+ – G – Bb7↓ – C
```

## Usage

### Running the 7-Limit Generator

```bash
python3 aetherviz_7limit.py
```

This generates:
- `7limit_invocation.wav` — 8-second stereo audio (48kHz, 16-bit)
- `7limit.mid` — MIDI chord progression (120 BPM)

### Technical Details

**Audio Synthesis:**
- Sample rate: 48 kHz
- Bit depth: 16-bit
- Channels: Stereo with spatial panning
- Duration: 8 seconds
- Base frequency: 110 Hz (A2)

**MIDI Export:**
- Tempo: 120 BPM
- Base note: MIDI 45 (A2)
- Format: Type 1 MIDI file
- Approximates 7-limit intervals in 12-tone equal temperament

**Carrier Frequencies:**
```
1. 110.00 Hz — 1/1   (Unison)
2. 125.71 Hz — 8/7   (Septimal major second)
3. 128.33 Hz — 7/6   (Septimal minor third)
4. 132.00 Hz — 6/5   (Minor third)
5. 137.50 Hz — 5/4   (Major third)
6. 146.67 Hz — 4/3   (Perfect fourth)
```

### Synthesis Features

The audio renderer includes:

1. **Pure sine wave carriers** at 7-limit ratio frequencies
2. **Septimal glissando** — 7% frequency modulation for characteristic "bluesy" sound
3. **Natural envelope** — fast attack, slow decay mimicking acoustic resonance
4. **Stereo panning** — spatial distribution for immersive listening

## Historical Context

### Harry Partch (1901-1974)

Partch rejected the "tyranny" of 12-tone equal temperament, developing:
- 43-tone-per-octave scale (later expanded to 48 tones)
- Custom instruments (Chromelodeon, Cloud Chamber Bowls, Quadrangularis Reversum)
- Corporeal music philosophy — dance, voice, and visual art integration
- Genesis of a Music (1949) — his theoretical treatise

### Ben Johnston (b. 1926)

Johnston studied with Partch and extended just intonation notation:
- Developed extended accidentals for prime harmonics
- Applied system to string quartets (10 quartets using extended JI)
- Bridge between Partch's radical tuning and traditional notation

## Philosophy

> "The swarm hears the primes."

This implementation embodies FlameLang's commitment to:
- **Pure ratios** over tempered compromise
- **Biological resonance** — harmonics as found in nature
- **Quantum precision** — exact frequency relationships
- **Dimensional authenticity** — physics-validated sound

## Next Steps

What limit shall we ascend to next?

- **11-limit** — Adding prime 11 (neutral seconds/thirds)
- **13-limit** — Johnston's extended prime system
- **31-limit** — Partch's full diamond

## References

- Partch, H. (1974). *Genesis of a Music* (2nd ed.). Da Capo Press.
- Johnston, B. (2006). *"Maximum Clarity" and Other Writings on Music*. University of Illinois Press.
- Gilmore, B. (1998). *Harry Partch: A Biography*. Yale University Press.

---

**The flame now sings in 7-limit purity.**  
**We have transcended equal temperament.**  
**The swarm hears the primes.**

🖤🔥

**Ratio Ex Nihilo**  
Strategickhaos DAO LLC © 2025
