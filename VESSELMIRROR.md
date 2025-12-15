# INV-079: VesselMirror Implementation

## Overview

VesselMirror is a **Sovereign Web Capture & Synthesis Engine** that automates the F12 DevTools → Save As → Yin-Yang remix methodology. It's part of the FlameLang v2.0.0 ecosystem and implements the **Ratio Ex Nihilo** principle.

## Architecture

The system implements a 6-stage transformation pipeline:

```
┌─────────────────────────────────────────────────────────────────┐
│                    VESSELMIRROR PIPELINE                        │
├─────────────────────────────────────────────────────────────────┤
│  Stage 1: CAPTURE     → Headless browser snapshot               │
│  Stage 2: DISSECT     → Parse DOM into atomic components        │
│  Stage 3: ANALYZE     → Extract patterns, styles, structure     │
│  Stage 4: SYNTHESIZE  → Yin-Yang merge from multiple sources    │
│  Stage 5: SOVEREIGN   → Output clean, dependency-free HTML      │
│  Stage 6: PROVE       → SHA-256 hash + on-chain attestation     │
└─────────────────────────────────────────────────────────────────┘
```

## Installation

Build VesselMirror from the root of the repository:

```bash
cargo build --release -p vessel
```

The binary will be available at:
```
target/release/vessel
```

## Command Reference

### 1. vessel capture

Capture a complete webpage with all assets.

```bash
vessel capture <URL> [OPTIONS]

Options:
  -o, --output <FILE>    Output file path (default: sanitized URL)
  -j, --js               Enable JavaScript execution (headless browser)
```

**Example:**
```bash
vessel capture https://github.com/settings/keys --output github-keys.html
```

### 2. vessel dissect

Parse HTML into atomic components (forms, buttons, layouts, styles, scripts).

```bash
vessel dissect <FILE> [OPTIONS]

Options:
  -e, --extract <TYPES>  Component types to extract (default: forms,buttons,layout)
  -o, --output <DIR>     Output directory (default: vessel_components)
```

**Example:**
```bash
vessel dissect github-keys.html --extract forms,buttons,layout --output components/
```

### 3. vessel analyze

Analyze page structure, styles, and patterns.

```bash
vessel analyze <FILE> [OPTIONS]

Options:
  -v, --verbose          Show detailed analysis
```

**Example:**
```bash
vessel analyze github-keys.html --verbose
```

**Output includes:**
- DOM depth and element count
- Semantic HTML elements
- Color palette extraction
- Font family analysis
- Pattern detection (repeated classes)

### 4. vessel merge

Synthesize multiple sources using Da Vinci methodology.

```bash
vessel merge <FILES...> [OPTIONS]

Options:
  -s, --synthesis <METHOD>  Synthesis method (davinci, golden, fibonacci)
  -o, --output <FILE>       Output file (default: vessel_merged.html)
```

**Synthesis Methods:**
- **davinci**: Golden ratio + harmonic colors + Fibonacci timing (default)
- **golden**: Focus on proportions and layout
- **fibonacci**: Focus on timing and animation sequences

**Example:**
```bash
vessel merge source1.html source2.html source3.html \
  --synthesis davinci \
  --output synthesized.html
```

### 5. vessel purify

Remove external dependencies and tracking mechanisms.

```bash
vessel purify <FILE> [OPTIONS]

Options:
  -i, --inline-all      Inline all external assets
  -o, --output <FILE>   Output file (default: <input>_sovereign.html)
```

**Removes:**
- Tracking scripts (Google Analytics, Facebook Pixel, etc.)
- CDN dependencies (jQuery, Bootstrap, etc.)
- Analytics iframes
- Tracking pixels
- External fonts

**Example:**
```bash
vessel purify synthesized.html --inline-all --output sovereign.html
```

### 6. vessel prove

Generate cryptographic proof of document integrity.

```bash
vessel prove <FILE> [OPTIONS]

Options:
  -c, --chain <NAME>    Blockchain for attestation (swarmgate, ethereum, none)
  -v, --verbose         Show detailed proof information
```

**Example:**
```bash
vessel prove sovereign.html --chain swarmgate --verbose
```

**Output:**
- SHA-256 hash of the document
- JSON proof file (`<input>.proof.json`)
- Optional blockchain attestation

## Da Vinci Synthesis

The heart of VesselMirror is the **Da Vinci synthesis mode**, which combines:

### Golden Ratio (φ = 1.618...)

Automatically applies golden section grid layout:

```css
:root {
    --golden-ratio: 1.618033988749895;
    --golden-small: calc(100% / (1 + var(--golden-ratio)));
    --golden-large: calc(100% * var(--golden-ratio) / (1 + var(--golden-ratio)));
}
```

### Harmonic Colors

Colors based on golden angle rotation (360° / φ):

```css
:root {
    --vessel-primary: hsl(0, 70%, 50%);
    --vessel-secondary: hsl(222, 70%, 50%);  /* 360° / φ */
    --vessel-tertiary: hsl(137, 70%, 50%);   /* 222° / φ */
    --vessel-accent: hsl(85, 70%, 50%);      /* 137° / φ */
}
```

### Fibonacci Sequences

Animation timing based on Fibonacci numbers:

```javascript
const vesselFibonacci = [0, 1, 1, 2, 3, 5, 8, 13, 21];

function vesselFibDelay(index) {
    return vesselFibonacci[index] * 100; // milliseconds
}
```

## Complete Workflow Example

Here's a complete workflow from capture to proof:

```bash
# Step 1: Capture multiple pages
vessel capture https://github.com/settings/keys -o github.html
vessel capture https://console.cloud.google.com -o gcloud.html

# Step 2: Analyze both sources
vessel analyze github.html --verbose
vessel analyze gcloud.html --verbose

# Step 3: Dissect into components
vessel dissect github.html --extract forms,buttons -o github_components/
vessel dissect gcloud.html --extract layout,styles -o gcloud_components/

# Step 4: Synthesize with Da Vinci method
vessel merge github.html gcloud.html \
  --synthesis davinci \
  --output synthesized.html

# Step 5: Purify to remove dependencies
vessel purify synthesized.html \
  --inline-all \
  --output sovereign.html

# Step 6: Prove integrity
vessel prove sovereign.html \
  --chain swarmgate \
  --verbose
```

## Implementation Details

### Module Structure

```
tools/vessel/src/
├── main.rs          # CLI entry point with clap
├── types.rs         # Core data structures
├── capture.rs       # Stage 1: Web capture
├── dissect.rs       # Stage 2: DOM parsing
├── analyze.rs       # Stage 3: Pattern analysis
├── davinci.rs       # Golden ratio & Fibonacci
├── synthesize.rs    # Stage 4: Yin-Yang merge
├── sovereign.rs     # Stage 5: Dependency removal
└── prove.rs         # Stage 6: Cryptographic proof
```

### Key Dependencies

- **scraper**: HTML/DOM parsing with CSS selectors
- **reqwest**: HTTP client for webpage capture
- **headless_chrome**: Headless browser for JS execution
- **sha2**: SHA-256 hashing
- **clap**: CLI framework
- **colored**: Terminal colors
- **serde/serde_json**: Serialization

### Design Principles

1. **Ratio Ex Nihilo**: Creating order from captured chaos
2. **Sovereignty**: No external dependencies in final output
3. **Da Vinci Methodology**: Anatomical deconstruction and synthesis
4. **Yin-Yang Fusion**: Combining the best of multiple sources
5. **Golden Ratio**: Harmonic proportions in layout and color
6. **Fibonacci**: Natural sequences for timing and spacing

## Future Enhancements

- [ ] Full headless browser integration (Puppeteer/Chrome)
- [ ] Asset inlining (images, fonts, videos)
- [ ] Real blockchain attestation (Ethereum, Swarmgate)
- [ ] Browser extension version
- [ ] Live synthesis preview server
- [ ] Component library management
- [ ] Template system for common patterns
- [ ] Diff and merge conflict resolution
- [ ] Version control for synthesized documents

## Philosophy

VesselMirror embodies the **Ratio Ex Nihilo** principle:

1. **Capture** reality as-is (the vessel)
2. **Dissect** into atomic truth (da Vinci's anatomy)
3. **Analyze** patterns and harmony (the ratio)
4. **Synthesize** something greater (Yin-Yang fusion)
5. **Sovereign** output (no dependencies, pure code)
6. **Prove** cryptographically (immutable truth)

## Integration with FlameLang

VesselMirror is one of four tools in the FlameLang v2.0.0 ecosystem:

1. **flamec**: FlameLang compiler (English → Hebrew → Unicode → Wave → DNA → LLVM)
2. **flamefmt**: Code formatter
3. **flamelsp**: Language Server Protocol
4. **vessel**: Web capture & synthesis (VesselMirror)

All tools share the same principles of sovereignty, transformation, and ratio-based design.

## License

MIT License - © 2025 Strategickhaos DAO LLC

---

**Built with 🔥 by Strategickhaos DAO LLC**

*"Greater than the sum of its parts"*
