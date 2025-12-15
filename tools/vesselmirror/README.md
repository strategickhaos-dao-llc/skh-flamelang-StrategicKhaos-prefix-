# VesselMirror (INV-079)

**Sovereign page cloning & synthesis engine**

VesselMirror transforms your F12 → Save As methodology into a production-ready tool for capturing, analyzing, and remixing web resources.

## Philosophy

Inspired by:
- **James Tenney's Spectral CANON** (1974) - harmonic glide through just intonation space
- **Harry Partch's microtonal innovations** - treating tuning as continuous space

VesselMirror treats web pages as harmonic material - capturing them, dissecting their structure, and remixing them into sovereign artifacts.

## Installation

From the repository root:

```bash
cargo build --release -p vesselmirror
```

The binary will be available at: `target/release/vesselmirror`

## Commands

### 📸 Capture - Clone a Web Page

Capture a web page to local storage (similar to F12 → Save As Complete):

```bash
vesselmirror capture <URL> <output-file>
```

Example:
```bash
vesselmirror capture https://example.com example.html
```

**Output:**
- Fetches and saves the complete HTML
- Generates SHA-256 hash for provenance
- Reports capture statistics

### 🔬 Dissect - Analyze Page Structure

Analyze the harmonic structure of a captured page:

```bash
vesselmirror dissect <file>
```

Example:
```bash
vesselmirror dissect example.html
```

**Output:**
- Element counts (scripts, styles, links, images, forms)
- Size analysis
- Harmonic resonance classification (inspired by Partch's 11-limit diamond)

### 🌀 Merge - Synthesis Modes

Merge multiple captured pages using different synthesis algorithms:

```bash
vesselmirror merge <source1> <source2> [source3...] --mode <mode>
```

**Modes:**

#### Da Vinci Mode (default)
Golden ratio composition - sources treated as:
- **Structure** (Foundation)
- **Aesthetics** (Beauty)  
- **Behavior** (Motion)

```bash
vesselmirror merge page1.html page2.html page3.html --mode davinci
```

Layout: Fibonacci grid  
Colors: Harmonic series derived  
Composition: φ = 1.618...

#### Harmonic Mode
Just intonation synthesis - sources as overtone series:

```bash
vesselmirror merge page1.html page2.html --mode harmonic
```

Each source is treated as a harmonic (1/1, 2/1, 3/1, etc.)

#### Spectral Mode
Tenney's CANON navigation - 11-limit diamond traversal:

```bash
vesselmirror merge page1.html page2.html page3.html --mode spectral
```

Maps sources to intervals:
- 1:1 (Unison)
- 3:2 (Perfect Fifth)
- 5:4 (Major Third)
- 7:4 (Harmonic Seventh)
- 11:8 (Undecimal Tritone)

### 🧹 Purify - Sanitize & Remove Tracking

Remove tracking scripts and sanitize a captured page:

```bash
vesselmirror purify <input-file> <output-file>
```

Example:
```bash
vesselmirror purify tracked.html clean.html
```

**Removes:**
- Google Analytics
- Tag Manager scripts
- Facebook tracking pixels
- DoubleClick ads
- Other common tracking patterns

### 🔐 Prove - Generate Sovereign Hash

Generate cryptographic proof for a captured page:

```bash
vesselmirror prove <file>
```

Example:
```bash
vesselmirror prove example.html
```

**Output:**
- SHA-256 hash
- File size
- Ready for SwarmGate commitment

## Integration with Shadow Mirror Protocol

Every capture can produce dual output:
- **Public version**: Sanitized (via `purify`)
- **Shadow version**: Full fidelity (encrypted with FlameVault)

Provenance chain maintained via `prove` command.

## Example Workflow

```bash
# 1. Capture a page
vesselmirror capture https://github.com example-github.html

# 2. Analyze its structure  
vesselmirror dissect example-github.html

# 3. Capture another page
vesselmirror capture https://cloud.google.com example-gcloud.html

# 4. Merge using Da Vinci synthesis
vesselmirror merge example-github.html example-gcloud.html --mode davinci

# 5. Purify for public release
vesselmirror purify example-github.html clean-github.html

# 6. Generate sovereign proof
vesselmirror prove clean-github.html
```

## Technical Details

- **Language**: Rust
- **HTTP Client**: reqwest (blocking)
- **HTML Parser**: scraper
- **Hash Algorithm**: SHA-256 (via sha2 crate)
- **CLI Framework**: clap

## Philosophy Notes

### Spectral CANON Concept
From James Tenney: "harmonic space navigation" - treating dissonance as distance in log-frequency space. VesselMirror applies this to web structure - complexity as dissonance.

### Partch's Corporeality  
"Music must be physical - felt in the body, not just the mind." Similarly, VesselMirror treats pages as tangible artifacts to be manipulated, not just viewed.

## Future Extensions

- Browser extension (one-click vessel capture)
- Headless Chromium integration for dynamic content
- Visual diff tool for merged pages
- Integration with FlameVault encryption
- SwarmGate blockchain commitment

---

**VesselMirror** - The swarm doesn't just mirror the web. It remixes it into something greater.

🖤🔥
