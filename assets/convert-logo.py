#!/usr/bin/env python3
"""
Strategickhaos Logo Converter
Converts SVG logo to PNG format for GitHub OAuth
"""

import base64
import os
from pathlib import Path

def svg_to_png_html_method():
    """
    Opens the HTML converter in the default browser
    """
    html_path = Path(__file__).parent / "logo-export.html"
    if html_path.exists():
        os.system(f'$BROWSER "{html_path}"' if os.name != 'nt' else f'start "{html_path}"')
        print("✅ Opening logo converter in browser...")
        print("   Click 'Download PNG' button to save the file")
    else:
        print("❌ logo-export.html not found")

def create_data_uri():
    """
    Creates a data URI for the SVG that can be used in HTML/CSS
    """
    svg_path = Path(__file__).parent / "strategickhaos-logo.svg"
    if not svg_path.exists():
        print("❌ strategickhaos-logo.svg not found")
        return
    
    with open(svg_path, 'r') as f:
        svg_content = f.read()
    
    # Base64 encode
    encoded = base64.b64encode(svg_content.encode()).decode()
    data_uri = f"data:image/svg+xml;base64,{encoded}"
    
    print("✅ Data URI created:")
    print(f"\n{data_uri}\n")
    print("📋 Copy this and paste into browser address bar to view")
    
    # Save to file
    uri_file = Path(__file__).parent / "logo-data-uri.txt"
    with open(uri_file, 'w') as f:
        f.write(data_uri)
    print(f"💾 Saved to: {uri_file}")

def instructions():
    """
    Display conversion instructions
    """
    print("""
🔥 STRATEGICKHAOS LOGO CONVERTER
═════════════════════════════════

📁 Files in this directory:
   • strategickhaos-logo.svg    - Source vector logo
   • logo-export.html            - Web-based PNG converter
   • README.md                   - Usage instructions

🎯 QUICK START (Choose one method):

METHOD 1: Web Browser (Easiest)
────────────────────────────────
1. Open 'logo-export.html' in your browser
2. Click "Download PNG (500x500)" or "Download PNG (1024x1024)"
3. Upload the downloaded PNG to GitHub OAuth settings

METHOD 2: Online Converter
────────────────────────────────
1. Go to: https://convertio.co/svg-png/
2. Upload 'strategickhaos-logo.svg'
3. Download the converted PNG
4. Upload to GitHub

METHOD 3: Command Line (Requires tools)
────────────────────────────────
If you have rsvg-convert or ImageMagick:

    # Install rsvg-convert (Ubuntu/Debian)
    sudo apt install librsvg2-bin
    
    # Convert to PNG
    rsvg-convert -w 1024 -h 1024 strategickhaos-logo.svg -o logo.png

METHOD 4: Python (Requires cairosvg)
────────────────────────────────
    pip install cairosvg
    cairosvg strategickhaos-logo.svg -o logo.png -W 1024 -H 1024

📤 GITHUB OAUTH UPLOAD:
────────────────────────────────
1. Go to: https://github.com/settings/developers
2. Click your OAuth App → Edit
3. Upload logo (PNG/JPG, max 1MB, square recommended)
4. Save changes

🎨 DESIGN SPECS:
────────────────────────────────
• Size: 500x500 or 1024x1024 (square)
• Format: PNG with transparency
• Colors: Cyan (#00FFFF) + Orange (#FF6B35)
• Elements: Toruk wings + Node 137 + Sacred geometry

📧 Questions? security@strategickhaos.ai

Ratio Ex Nihilo | Strategickhaos DAO LLC © 2025
    """)

if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1 and sys.argv[1] == "datauri":
        create_data_uri()
    elif len(sys.argv) > 1 and sys.argv[1] == "open":
        svg_to_png_html_method()
    else:
        instructions()
