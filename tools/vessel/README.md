# 🚢 VesselMirror v1.0.0

**Sovereign Web Capture & Synthesis Engine**

> INV-079: Codifying your F12 DevTools → Save As → Yin-Yang remix methodology

---

## Overview

VesselMirror automates the process of capturing, deconstructing, analyzing, and synthesizing web pages into sovereign, dependency-free documents. Inspired by Leonardo da Vinci's anatomical studies and the principle of Yin-Yang fusion.

## Architecture

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

```bash
cargo build --release -p vessel
```

The binary will be available at `target/release/vessel`

## Quick Start

### 1. Capture a Webpage

```bash
vessel capture https://github.com/settings/keys --output github-keys.html
```

Options:
- `--output, -o`: Specify output file path
- `--js`: Enable JavaScript execution (headless browser mode)

### 2. Dissect into Components

```bash
vessel dissect github-keys.html --extract forms,buttons,layout
```

Options:
- `--extract, -e`: Component types to extract (forms, buttons, layout, styles, scripts)
- `--output, -o`: Output directory for components

### 3. Analyze Structure

```bash
vessel analyze github-keys.html --verbose
```

Provides:
- DOM structure analysis (depth, element count)
- Style analysis (color palette, fonts)
- Pattern detection (repeated classes, common structures)

### 4. Synthesize (Yin-Yang Merge)

```bash
vessel merge github-keys.html vertex-ai.html --synthesis davinci -o merged.html
```

Synthesis methods:
- `davinci`: Golden ratio + harmonic colors + Fibonacci timing (default)
- `golden`: Focus on proportions and layout
- `fibonacci`: Focus on timing and sequences

### 5. Purify (Remove Dependencies)

```bash
vessel purify merged.html --inline-all --output sovereign.html
```

Removes:
- Tracking scripts (Google Analytics, Facebook Pixel, etc.)
- CDN dependencies
- Analytics and ads
- External fonts and resources

### 6. Prove Integrity

```bash
vessel prove sovereign.html --chain swarmgate --verbose
```

Creates:
- SHA-256 hash of the document
- Cryptographic proof file (`.proof.json`)
- Optional blockchain attestation

## Da Vinci Mode

### The Special Sauce: Yin-Yang Synthesis

VesselMirror's Da Vinci mode combines the best elements from multiple sources:

- **Structure** from source A (the bones)
- **Styling** from source B (the skin)
- **Functionality** from source C (the nerves)

Combined with:
- **Golden ratio** proportions (φ = 1.618...)
- **Harmonic colors** (hue rotation by golden angle)
- **Fibonacci sequences** for animation timing

### Golden Ratio Grid

Automatically applied CSS:

```css
:root {
    --golden-ratio: 1.618033988749895;
    --golden-small: calc(100% / (1 + var(--golden-ratio)));
    --golden-large: calc(100% * var(--golden-ratio) / (1 + var(--golden-ratio)));
}

.vessel-golden-container {
    display: grid;
    grid-template-columns: var(--golden-large) var(--golden-small);
}
```

### Harmonic Color Palette

```css
:root {
    --vessel-primary: hsl(0, 70%, 50%);
    --vessel-secondary: hsl(222, 70%, 50%);  /* 360° / φ */
    --vessel-tertiary: hsl(137, 70%, 50%);   /* 222° / φ */
}
```

### Fibonacci Animation Timing

```javascript
const vesselFibonacci = [0, 1, 1, 2, 3, 5, 8, 13, 21];

function vesselFibDelay(index) {
    return vesselFibonacci[index] * 100; // milliseconds
}
```

## Methodology Mapping

| Your Manual Action | VesselMirror Command |
|-------------------|---------------------|
| F12 DevTools inspection | `vessel analyze <file>` |
| "Save As Complete" | `vessel capture <url>` |
| Extract specific elements | `vessel dissect <file> --extract forms,buttons` |
| Copy/remix across pages | `vessel merge <source1> <source2> --yinyang` |
| Clean vendor cruft | `vessel purify <file> --inline-all` |
| Document hash | `vessel prove <file>` |

## Examples

### Complete Workflow

```bash
# Step 1: Capture two pages
vessel capture https://github.com/settings/keys -o github.html
vessel capture https://console.cloud.google.com -o gcloud.html

# Step 2: Analyze both
vessel analyze github.html --verbose
vessel analyze gcloud.html --verbose

# Step 3: Dissect components
vessel dissect github.html --extract forms,buttons -o github_components
vessel dissect gcloud.html --extract layout,styles -o gcloud_components

# Step 4: Synthesize with Da Vinci method
vessel merge github.html gcloud.html --synthesis davinci -o synthesized.html

# Step 5: Purify to remove dependencies
vessel purify synthesized.html --inline-all -o sovereign.html

# Step 6: Prove integrity
vessel prove sovereign.html --chain swarmgate --verbose
```

### Quick Sovereign Capture

```bash
vessel capture https://example.com -o example.html && \
vessel purify example.html --inline-all -o sovereign.html && \
vessel prove sovereign.html --verbose
```

## Features

### ✅ Implemented

- Full webpage capture via HTTP
- DOM parsing and component extraction
- Structure and style analysis
- Pattern detection
- Da Vinci synthesis (golden ratio, harmonic colors, Fibonacci)
- Dependency removal and purification
- SHA-256 integrity proofs
- Sovereignty metadata

### 🚧 Future Enhancements

- Full headless browser support (Chrome/Puppeteer)
- Asset inlining (images, fonts)
- Real blockchain attestation (Ethereum, Swarmgate)
- Browser extension version
- Live synthesis preview
- Component library management

## Philosophy

VesselMirror embodies the **Ratio Ex Nihilo** principle:

1. **Capture** reality as-is (the vessel)
2. **Dissect** into atomic truth (da Vinci's anatomy)
3. **Analyze** patterns and harmony (the ratio)
4. **Synthesize** something greater (Yin-Yang fusion)
5. **Sovereign** output (no dependencies, pure code)
6. **Prove** cryptographically (immutable truth)

## License

MIT License - © 2025 Strategickhaos DAO LLC

## Part of the FlameLang Ecosystem

VesselMirror is a tool in the FlameLang v2.0.0 toolchain:
- **flamec**: FlameLang compiler
- **flamefmt**: Code formatter
- **flamelsp**: Language Server Protocol
- **vessel**: Web capture & synthesis (this tool)

---

**Built with 🔥 by Strategickhaos DAO LLC**

*"Greater than the sum of its parts"*
