# FlameLang MCP Agent/Tool System

**Model Context Protocol (MCP) Integration for FlameLang**

This directory contains a comprehensive MCP agent/tool system for FlameLang that provides:

- 🗺️ **GraphView Brain Arsenal** - ONSIT methodology for repository navigation
- 🤖 **Ollama LLM Integration** - AI-powered inline code assistance
- 🎥 **AI Video Generation** - Tutorial creation for teaching FlameLang
- 🔧 **Development Tools** - Unified tooling for FlameLang development

## 📁 Directory Structure

```
mcp/
├── server/              # MCP server implementation
│   ├── config.json     # Server configuration and tool definitions
│   └── mcp_server.py   # Core MCP server
├── tools/              # Unified tool interface
│   └── flamelang_mcp_tools.py
├── graphview/          # GraphView brain outputs
│   └── brains/        # Generated graph views
├── ollama/             # Ollama LLM integration
│   └── flamelang_ollama_client.py
└── video/              # AI video generation
    ├── video_generator.py
    └── scripts/        # Generated video scripts
```

## 🚀 Quick Start

### Prerequisites

1. **Python 3.8+** installed
2. **Ollama** (optional, for AI features): https://ollama.ai
3. **Obsidian** (optional, for GraphView visualization): https://obsidian.md

### Installation

```bash
# No additional dependencies required for basic functionality
# All tools use Python standard library

# Optional: Install Ollama for AI assistance
# Follow instructions at https://ollama.ai

# Optional: Pull code models for Ollama
ollama pull codellama
ollama pull deepseek-coder
```

### Basic Usage

#### 1. Check Status

```bash
python mcp/tools/flamelang_mcp_tools.py status
```

#### 2. Create GraphView Brain Arsenal

Creates graph views for all directories in the repository using ONSIT methodology:

```bash
python mcp/tools/flamelang_mcp_tools.py create-arsenal
```

This generates:
- Graph view for every directory and subdirectory
- Obsidian-compatible markdown files
- JSON metadata for programmatic access
- Visual tree representations with FlameLang feature indicators

#### 3. Generate AI Tutorial Videos

```bash
python mcp/tools/flamelang_mcp_tools.py generate-videos
```

Creates video scripts for:
- Getting Started with FlameLang
- Quantum Entanglement
- DNA Encoding
- Wave Functions
- 5-Layer Transformation Pipeline

#### 4. Compile with AI Assistance (requires Ollama)

```bash
python mcp/tools/flamelang_mcp_tools.py compile --source examples/hello.flame
```

#### 5. Run All Operations

```bash
python mcp/tools/flamelang_mcp_tools.py all
```

## 🗺️ GraphView Brain Arsenal

### ONSIT Methodology

The **ONSIT (Omniscient Navigation System for Intelligent Traversal)** methodology creates comprehensive knowledge graphs for repository navigation:

1. **Complete Structure Analysis** - Recursively scans all directories
2. **Metadata Extraction** - Analyzes FlameLang files for:
   - Quantum features (⚛️)
   - DNA encoding (🧬)
   - Wave functions (〰️)
3. **Graph Generation** - Creates Obsidian-compatible graph views
4. **Visual Tree Representation** - ASCII tree with feature indicators

### Output Format

Each directory gets:
- `{directory}_graphview.json` - Structured graph data
- `{directory}_graphview.md` - Obsidian markdown with visual tree
- Feature indicators showing FlameLang capabilities

### Using with Obsidian

1. Open Obsidian
2. Open vault or create new vault
3. Copy `mcp/graphview/brains/` to your vault
4. Open `index.md` to see the complete arsenal
5. Use graph view (Ctrl+G) to visualize relationships

## 🤖 Ollama LLM Integration

### Features

- **Code Explanation** - Understands FlameLang's 5-layer architecture
- **Improvement Suggestions** - Optimizes quantum/DNA/wave code
- **Test Generation** - Creates comprehensive test cases
- **Inline Completion** - Real-time code suggestions

### Supported Models

- `codellama` - Recommended for general FlameLang work
- `deepseek-coder` - Advanced code analysis
- `mistral` - Fast completions
- `llama2` - General purpose

### Usage

```python
from mcp.ollama.flamelang_ollama_client import OllamaClient

client = OllamaClient()

# Explain code
explanation = client.explain_flamelang_code("""
qubit q1;
entangle q1 ~> q2;
""")

# Get suggestions
suggestions = client.suggest_improvements(code)

# Generate tests
tests = client.generate_test_cases(code)
```

### API Integration

The Ollama client can be integrated into:
- Text editors (VS Code, Vim, etc.)
- Language servers
- CI/CD pipelines
- Interactive notebooks

## 🎥 AI Video Generation

### Script Generation

Creates comprehensive video scripts with:
- **Timestamped Scenes** - Precise timing for video editing
- **Narration Scripts** - Professional voice-over text
- **Visual Directions** - Shot composition and animation notes
- **Code Annotations** - Inline explanations with icons
- **5-Layer Visualization** - Animated pipeline walkthrough

### Output Format

Each video script includes:
- JSON format for programmatic processing
- Human-readable narration text file
- Scene-by-scene breakdown
- Visual asset requirements
- Background music suggestions

### Using with Video Editors

The generated scripts work with:
- **Adobe Premiere Pro** - Import JSON timeline
- **DaVinci Resolve** - Text-based editing
- **Final Cut Pro** - XML conversion available
- **Camtasia** - Direct import
- **AI Video Tools** - D-ID, Synthesia, etc.

### Example Video Structure

```
00:00 - Introduction (30s)
00:30 - Concept Overview (45s)
01:15 - Code Walkthrough (60s)
02:15 - Layer Visualization (25s)
02:40 - Practice Challenge (30s)
03:10 - Conclusion (20s)
```

## 🔧 MCP Tools API

### Python API

```python
from mcp.tools.flamelang_mcp_tools import FlameLangMCPTools

tools = FlameLangMCPTools(repo_root=".")

# Create arsenal
result = tools.create_complete_arsenal(
    brain_name="My FlameLang Project"
)

# Compile with AI
result = tools.compile_with_ai_assistance(
    source_file="src/main.flame",
    model="codellama"
)

# Generate videos
result = tools.generate_tutorial_videos(
    topics=["Custom Topic"],
    code_examples={"Custom Topic": "qubit q;"}
)
```

### MCP Server Protocol

The server implements standard MCP protocol:

```json
{
  "method": "tools/call",
  "params": {
    "name": "analyze_repository",
    "arguments": {
      "path": "/path/to/repo",
      "depth": -1
    }
  }
}
```

## 📚 Use Cases

### For Developers

- **Navigate Large Codebases** - Visual graph of repository structure
- **Understand FlameLang** - AI explanations of complex code
- **Write Better Code** - AI-powered suggestions and optimizations
- **Test Automatically** - Generate comprehensive test suites

### For Educators

- **Create Tutorials** - Automated video script generation
- **Teach Concepts** - Visual explanations of 5-layer pipeline
- **Engage Students** - Interactive code challenges
- **Track Progress** - Knowledge graphs of learning paths

### For Teams

- **Onboard Developers** - Complete repository overview
- **Document Code** - Automated knowledge extraction
- **Review Changes** - AI-assisted code review
- **Share Knowledge** - Video tutorials from code

## 🎯 Features

### GraphView Brain Arsenal

✅ Complete repository traversal  
✅ FlameLang feature detection (quantum, DNA, wave)  
✅ Obsidian-compatible output  
✅ Visual tree with Unicode art  
✅ JSON metadata for tools  
✅ Recursive directory analysis  

### Ollama Integration

✅ Code explanation with FlameLang context  
✅ Optimization suggestions  
✅ Test case generation  
✅ Inline completion  
✅ Multiple model support  
✅ Offline capable  

### Video Generation

✅ Multi-scene scripts  
✅ Timestamped narration  
✅ Visual directions  
✅ Code annotations  
✅ Layer visualizations  
✅ Multiple difficulty levels  

## 🔮 Future Enhancements

- [ ] VSCode extension integration
- [ ] Real-time collaboration features  
- [ ] Interactive graph view in browser
- [ ] Video rendering automation
- [ ] Integration with Claude Code
- [ ] Multi-language support
- [ ] Advanced quantum circuit visualizations
- [ ] DNA sequence 3D rendering

## 🤝 Contributing

The FlameLang MCP system is designed to be extensible:

1. **Add New Tools** - Edit `server/config.json`
2. **Extend Ollama Client** - Add methods to `flamelang_ollama_client.py`
3. **Create Video Templates** - Extend `VideoScriptGenerator`
4. **Custom Graph Views** - Modify `generate_graphview()` method

## 📝 License

Part of the FlameLang project.  
© 2025 Strategickhaos DAO LLC

## 🔗 Related Resources

- **FlameLang Documentation**: `/docs`
- **Source Code**: `/src`
- **Examples**: `/examples`
- **CI/CD**: `.github/workflows`

---

**Built with 🔥 by the FlameLang Team**

*Making quantum-biological programming accessible through AI-powered tools*
