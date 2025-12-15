#!/bin/bash
# FlameLang MCP Tools - Quick Start Script
# This script demonstrates the main features of the MCP system

echo "🔥 FlameLang MCP Tools - Quick Start"
echo "===================================="
echo ""

# Check if Python 3 is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed."
    exit 1
fi

echo "✅ Python 3 detected: $(python3 --version)"
echo ""

# Change to script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$REPO_ROOT"

echo "📁 Repository: $REPO_ROOT"
echo ""

# Step 1: Check status
echo "Step 1: Checking MCP Tools status..."
echo "------------------------------------"
python3 mcp/tools/flamelang_mcp_tools.py status
echo ""

# Step 2: Create GraphView Brain Arsenal
echo "Step 2: Creating GraphView Brain Arsenal..."
echo "--------------------------------------------"
python3 mcp/tools/flamelang_mcp_tools.py create-arsenal
echo ""

# Step 3: Generate Video Tutorials
echo "Step 3: Generating AI Video Tutorial Scripts..."
echo "------------------------------------------------"
python3 mcp/tools/flamelang_mcp_tools.py generate-videos
echo ""

# Summary
echo "============================================"
echo "✅ Quick Start Complete!"
echo "============================================"
echo ""
echo "📊 Generated Files:"
echo "  - GraphView Brains: mcp/graphview/brains/"
echo "  - Video Scripts:    mcp/video/scripts/"
echo ""
echo "📖 Next Steps:"
echo "  1. Open mcp/graphview/brains/flamelang_repository_brain/index.md in Obsidian"
echo "  2. Review video scripts in mcp/video/scripts/"
echo "  3. Read mcp/USAGE_GUIDE.md for detailed instructions"
echo "  4. Try: python3 mcp/tools/flamelang_mcp_tools.py --help"
echo ""
echo "🔗 Documentation:"
echo "  - README:      mcp/README.md"
echo "  - Usage Guide: mcp/USAGE_GUIDE.md"
echo "  - Summary:     mcp/IMPLEMENTATION_SUMMARY.md"
echo ""
echo "🚀 Happy coding with FlameLang!"
