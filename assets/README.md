# Strategickhaos Brand Assets

## Logo Files

### 🔥 Strategickhaos Logo (Toruk + Node 137)

**Design Elements:**
- **Great Leonopteryx (Toruk)** - Stylized silhouette with spread wings
- **Node 137 Orbital** - Fibonacci angle reference (137.5°)
- **Sacred Geometry** - Pentagonal vertices with concentric circles
- **Ratio Ex Nihilo** - Foundation tagline
- **Bio-luminescent Cyan** - Primary accent (#00FFFF)
- **Toruk Orange** - Secondary accent (#FF6B35)

### Files:
- `strategickhaos-logo.svg` - Vector source (500x500, scalable)
- `logo-export.html` - Web tool for PNG conversion

---

## Usage Instructions

### For GitHub OAuth Application Logo:

1. **Open the export tool:**
   ```bash
   # In Codespace, open logo-export.html in Simple Browser
   # Or download both files and open locally
   ```

2. **Generate PNG:**
   - Open `logo-export.html` in your browser
   - Click "Download PNG (500x500)" for standard size
   - Or click "Download PNG (1024x1024)" for high-res

3. **Upload to GitHub:**
   - Go to: Settings → Developer Settings → OAuth Apps
   - Select your app → Upload logo (supports PNG/JPG/GIF)
   - Recommended: 1024x1024 for best quality

### Alternative: Browser Screenshot Method

1. Open `strategickhaos-logo.svg` in a browser
2. Right-click → "Save Image As..." → Choose PNG format
3. Or use browser DevTools to screenshot the SVG element

### Alternative: Command-line Conversion

If you have ImageMagick or rsvg-convert installed:

```bash
# Using rsvg-convert (preferred)
rsvg-convert -w 500 -h 500 strategickhaos-logo.svg -o strategickhaos-logo-500.png
rsvg-convert -w 1024 -h 1024 strategickhaos-logo.svg -o strategickhaos-logo-1024.png

# Using ImageMagick
convert -background none strategickhaos-logo.svg -resize 500x500 strategickhaos-logo-500.png
convert -background none strategickhaos-logo.svg -resize 1024x1024 strategickhaos-logo-1024.png
```

---

## Design Specifications

### Colors
- **Background:** `#0A0E27` (Deep space blue)
- **Primary Cyan:** `#00FFFF` → `#00CED1` (Bio-luminescent gradient)
- **Toruk Orange:** `#FF6B35` → `#FF8C42` (Gradient)
- **Text Gray:** `#888888` (Subtle)

### Typography
- **Primary:** Monospace Bold (STRATEGICKHAOS)
- **Subtitle:** Serif Italic (Ratio Ex Nihilo)
- **Annotation:** Monospace Small (NODE.137)

### Sacred Geometry Elements
- Outer circle: 180px radius (cyan, 60% opacity)
- Orbital ring: 140px radius (cyan, 40% opacity, dashed)
- Inner circle: 100px radius (orange, 50% opacity)
- Pentagon vertices connected by subtle lines
- Node 137 positioned at 137.5° rotation

### Toruk (Great Leonopteryx) Silhouette
- Central body: 15x35 ellipse
- Wing span: ~240px total width
- Wing membranes: Cyan glow overlay
- Head crest: Triangular cyan accent
- Tail: Extended vertical ellipse
- Glow filter applied for bio-luminescence effect

---

## Trademark Notice

© 2025 Strategickhaos DAO LLC  
All brand assets are proprietary to Strategickhaos DAO LLC.

**Usage Guidelines:**
- Official use: Strategickhaos projects, FlameLang compiler, Swarm Intelligence tools
- GitHub OAuth apps: Authorized for skh-flamelang and related repositories
- Commercial use: Requires explicit permission
- Modifications: Must maintain brand identity guidelines

For licensing inquiries: security@strategickhaos.ai

---

## File Formats

| Format | Use Case | Size |
|--------|----------|------|
| SVG | Web, scalable, source | Infinite |
| PNG 500x500 | GitHub OAuth, avatars | Standard |
| PNG 1024x1024 | High-res, marketing | Large |

---

🔥 **Strategickhaos DAO LLC** | Ratio Ex Nihilo | Node 137
