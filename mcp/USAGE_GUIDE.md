# FlameLang MCP Tools - Usage Guide

## Complete Guide to FlameLang MCP Agent/Tool System

This guide demonstrates how to use the FlameLang MCP (Model Context Protocol) agent/tool system to create graphview brains, integrate AI assistance, and generate educational videos.

## Table of Contents

1. [Quick Start](#quick-start)
2. [GraphView Brain Arsenal](#graphview-brain-arsenal)
3. [Ollama AI Integration](#ollama-ai-integration)
4. [Video Tutorial Generation](#video-tutorial-generation)
5. [Python API](#python-api)
6. [Advanced Usage](#advanced-usage)
7. [Integration Examples](#integration-examples)

---

## Quick Start

### 1. Check System Status

```bash
cd /path/to/flamelang-repository
python3 mcp/tools/flamelang_mcp_tools.py status
```

**Output:**
```
🔥 FlameLang MCP Tools Status

Repository: .
MCP Server: flamelang-mcp-server v2.0.0
Ollama Available: False
Video Generator: Ready

Available Tools: 5
  - analyze_repository
  - generate_graphview
  - flamelang_compile_with_ollama
  - generate_tutorial_video_script
  - create_graphview_brain
```

### 2. Run All Operations

Execute all MCP tools at once:

```bash
python3 mcp/tools/flamelang_mcp_tools.py all
```

This will:
- Analyze the complete repository structure
- Create graphview brains for all directories
- Generate video tutorial scripts
- Output locations for all generated files

---

## GraphView Brain Arsenal

### What is a GraphView Brain?

A **GraphView Brain** is a comprehensive knowledge graph of your repository that:
- Maps all files and directories
- Identifies FlameLang features (quantum, DNA, wave)
- Creates Obsidian-compatible visualizations
- Enables ONSIT (Omniscient Navigation System for Intelligent Traversal) methodology

### Creating the Arsenal

```bash
python3 mcp/tools/flamelang_mcp_tools.py create-arsenal
```

### Output Structure

```
mcp/graphview/brains/flamelang_repository_brain/
├── index.md                    # Master index with links to all graphs
├── src_graphview.md           # Source code graph
├── src_graphview.json         # Machine-readable graph data
├── mcp_graphview.md           # MCP tools graph
├── mcp_graphview.json         # MCP tools data
└── ...                        # Additional directory graphs
```

### Reading Graph Views

Each `.md` file contains:

1. **Visual Tree** with feature indicators:
   - ⚛️ = Quantum features
   - 🧬 = DNA encoding
   - 〰️ = Wave functions

2. **Node Details** with metadata:
   - Line counts
   - Feature summaries
   - Import/export lists

**Example:**
```markdown
# src - GraphView Brain

## Structure

```
└── src 
    ├── lexer 
    │   └── mod.rs ⚛️ 🧬 〰️
    ├── parser 
    │   └── mod.rs ⚛️ 🧬 〰️
    └── transform 
        └── mod.rs 〰️
```

## Node Details

### mod.rs
- **Lines:** 209
- **Features:** Quantum ⚛️
- **Features:** DNA 🧬
- **Features:** Wave 〰️
```

### Using with Obsidian

1. **Open Obsidian**
2. **Open Vault** (or create new)
3. **Copy Files:**
   ```bash
   cp -r mcp/graphview/brains/flamelang_repository_brain/* /path/to/obsidian/vault/
   ```
4. **Open** `index.md` in Obsidian
5. **View Graph** with `Ctrl+G` (Windows/Linux) or `Cmd+G` (Mac)

### ONSIT Methodology

The ONSIT methodology provides:

- **Omniscient**: Complete visibility of repository structure
- **Navigation**: Easy traversal between related files
- **System**: Organized hierarchical representation
- **Intelligent**: Feature detection and categorization
- **Traversal**: Multiple paths to discover relationships

---

## Ollama AI Integration

### Prerequisites

1. **Install Ollama:** https://ollama.ai
2. **Pull Models:**
   ```bash
   ollama pull codellama
   ollama pull deepseek-coder
   ```
3. **Verify:**
   ```bash
   ollama list
   ```

### Compiling with AI Assistance

```bash
# Create a sample FlameLang file
cat > example.flame << 'EOF'
// Quantum Entanglement Example
qubit q1;
qubit q2;
entangle q1 ~> q2;
H q1;
bell_phi+ q1 q2;
EOF

# Compile with AI assistance
python3 mcp/tools/flamelang_mcp_tools.py compile \
  --source example.flame \
  --model codellama
```

**Output includes:**
- Code explanation
- Optimization suggestions
- Generated test cases
- Compilation insights

### Using the Python API

```python
from mcp.ollama.flamelang_ollama_client import OllamaClient

client = OllamaClient()

# Check availability
if client.is_available():
    models = client.list_models()
    print(f"Available models: {models}")

# Explain code
code = """
qubit q;
entangle q ~> sin~([ATGC]);
"""

explanation = client.explain_flamelang_code(code, model="codellama")
print(explanation)

# Get improvement suggestions
suggestions = client.suggest_improvements(code)
print(suggestions)

# Generate test cases
tests = client.generate_test_cases(code)
for test in tests:
    print(f"Test: {test['description']}")
    print(f"  Input: {test['input']}")
    print(f"  Expected: {test['expected_output']}")
```

### Inline Completion

```python
partial_code = "qubit q1;\nentangle q1 ~> "
completion = client.inline_completion(partial_code, len(partial_code))
print(f"Suggested completion: {completion}")
```

---

## Video Tutorial Generation

### Generating Video Scripts

```bash
python3 mcp/tools/flamelang_mcp_tools.py generate-videos
```

### Default Topics

The system generates scripts for:
1. **Getting Started with FlameLang**
2. **Quantum Entanglement**
3. **DNA Encoding and Biological Computing**
4. **Wave Functions and Frequency Analysis**
5. **5-Layer Transformation Pipeline**

### Output Files

For each topic, you get:
- `{topic}_script.json` - Structured video script
- `{topic}_narration.txt` - Human-readable narration

### Script Structure

Each script contains:

```json
{
  "metadata": {
    "title": "FlameLang Tutorial: Quantum Entanglement",
    "duration_seconds": 300,
    "difficulty": "beginner",
    "learning_objectives": [...]
  },
  "scenes": [
    {
      "scene_number": 1,
      "title": "Introduction",
      "timestamp": "00:00",
      "duration": 30,
      "visual": {...},
      "narration": "...",
      "animations": [...]
    },
    ...
  ],
  "code_assets": [...],
  "visual_assets_needed": [...]
}
```

### Using with Video Editors

#### For Adobe Premiere Pro:
1. Import JSON script
2. Use narration text for voice-over
3. Follow visual directions for B-roll

#### For AI Video Tools (D-ID, Synthesia):
1. Copy narration text
2. Use timestamps for scene breaks
3. Reference visual descriptions

#### For Manual Editing:
1. Read narration file for script
2. Follow scene-by-scene breakdown
3. Use code snippets from `code_assets`

---

## Python API

### Complete Example

```python
from mcp.tools.flamelang_mcp_tools import FlameLangMCPTools

# Initialize tools
tools = FlameLangMCPTools(repo_root="/path/to/repo")

# 1. Create GraphView Brain Arsenal
arsenal_result = tools.create_complete_arsenal(
    brain_name="My FlameLang Project"
)
print(f"Created brain at: {arsenal_result['brain_directory']}")
print(f"Total views: {arsenal_result['total_views']}")

# 2. Compile with AI (if Ollama available)
compile_result = tools.compile_with_ai_assistance(
    source_file="src/main.flame",
    model="codellama"
)

if "ai_insights" in compile_result:
    print("AI Explanation:", compile_result["ai_insights"]["explanation"])
    print("Suggestions:", compile_result["ai_insights"]["suggestions"])

# 3. Generate Tutorial Videos
video_result = tools.generate_tutorial_videos(
    topics=["My Custom Topic"],
    code_examples={
        "My Custom Topic": """
qubit q;
H q;
"""
    }
)
print(f"Generated {video_result['generated_videos']} video scripts")
```

### Using Individual Components

```python
# Just the MCP Server
from mcp.server.mcp_server import FlameLangMCPServer

server = FlameLangMCPServer()
analysis = server.analyze_repository(".", depth=3)
print(f"Analyzed {len(analysis.get('children', []))} items")

# Just the Video Generator
from mcp.video.video_generator import VideoScriptGenerator

generator = VideoScriptGenerator()
script = generator.generate_tutorial_script(
    topic="Advanced Quantum Gates",
    code_examples=["qubit q; CNOT q1 q2;"],
    learning_objectives=["Use quantum gates"],
    difficulty="advanced",
    target_duration=600  # 10 minutes
)
generator.save_script(script, "output/")
```

---

## Advanced Usage

### Custom Graph View Generation

```python
from mcp.server.mcp_server import FlameLangMCPServer

server = FlameLangMCPServer()

# Generate graph view for specific directory
result = server.generate_graphview(
    directory="/path/to/directory",
    output_path="custom_graphview.md"
)

print(f"Nodes: {result['nodes']}")
print(f"Edges: {result['edges']}")
```

### Batch Video Generation

```python
from mcp.video.video_generator import VideoScriptGenerator

generator = VideoScriptGenerator()

topics = [
    "Introduction to FlameLang",
    "Quantum Computing Basics",
    "DNA Sequence Encoding",
    # ... more topics
]

for topic in topics:
    script = generator.generate_tutorial_script(
        topic=topic,
        code_examples=[f"// Code for {topic}"],
        learning_objectives=[f"Learn {topic}"],
        difficulty="beginner"
    )
    generator.save_script(script, "video_scripts/")
    print(f"✓ Generated script for: {topic}")
```

### Customizing Ollama Models

```python
from mcp.ollama.flamelang_ollama_client import OllamaClient

client = OllamaClient(base_url="http://localhost:11434")

# Try different models for different tasks
models = ["codellama", "deepseek-coder", "mistral"]

for model in models:
    if model in client.list_models():
        explanation = client.explain_flamelang_code(code, model=model)
        print(f"\n{model} says:")
        print(explanation)
```

---

## Integration Examples

### CI/CD Integration

```yaml
# .github/workflows/flamelang-docs.yml
name: Generate Documentation

on: [push]

jobs:
  generate-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Generate GraphView Brain
        run: |
          python3 mcp/tools/flamelang_mcp_tools.py create-arsenal
      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: graphview-brains
          path: mcp/graphview/brains/
```

### VSCode Integration

Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Generate GraphView",
      "type": "shell",
      "command": "python3 mcp/tools/flamelang_mcp_tools.py create-arsenal",
      "problemMatcher": []
    },
    {
      "label": "Generate Videos",
      "type": "shell",
      "command": "python3 mcp/tools/flamelang_mcp_tools.py generate-videos",
      "problemMatcher": []
    }
  ]
}
```

### Jupyter Notebook Integration

```python
# In Jupyter notebook
import sys
sys.path.append('/path/to/flamelang')

from mcp.tools.flamelang_mcp_tools import FlameLangMCPTools

tools = FlameLangMCPTools()
tools.print_status()

# Generate and display graph
arsenal = tools.create_complete_arsenal()
print(f"Brain created at: {arsenal['brain_directory']}")

# Display in notebook
from IPython.display import Markdown, display

with open(f"{arsenal['index_file']}", 'r') as f:
    display(Markdown(f.read()))
```

---

## Troubleshooting

### Ollama Not Available

If you see "Ollama Available: False":

1. **Check if running:**
   ```bash
   curl http://localhost:11434/api/tags
   ```

2. **Start Ollama:**
   ```bash
   ollama serve
   ```

3. **Verify models:**
   ```bash
   ollama list
   ```

### Graph View Not Displaying

1. **Check file permissions:**
   ```bash
   ls -la mcp/graphview/brains/
   ```

2. **Verify Obsidian vault path:**
   - Ensure files are in vault directory
   - Refresh Obsidian (Ctrl+R)

### Video Scripts Empty

1. **Check Python version:** (requires 3.8+)
   ```bash
   python3 --version
   ```

2. **Verify file creation:**
   ```bash
   ls -la mcp/video/scripts/
   ```

---

## Next Steps

1. **Explore Generated Files:**
   - Open graph views in Obsidian
   - Read video scripts for content ideas
   - Review JSON metadata for automation

2. **Customize for Your Needs:**
   - Modify video topics
   - Adjust graph view depth
   - Add custom Ollama prompts

3. **Integrate with Workflow:**
   - Add to CI/CD pipeline
   - Create VSCode tasks
   - Build custom tools on API

4. **Share with Team:**
   - Commit graph views to repo
   - Share video scripts for training
   - Use for onboarding new developers

---

## Support

For questions or issues:
- **Documentation:** `/docs`
- **Examples:** `/examples`
- **Issues:** GitHub Issues
- **Contact:** security@strategickhaos.ai

---

**Built with 🔥 by the FlameLang Team**
