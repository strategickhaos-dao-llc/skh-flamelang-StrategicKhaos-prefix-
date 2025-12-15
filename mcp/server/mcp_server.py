#!/usr/bin/env python3
"""
FlameLang MCP Server
Model Context Protocol server for FlameLang language tools and AI integration
"""

import json
import os
import sys
from typing import Any, Dict, List, Optional
from pathlib import Path


class FlameLangMCPServer:
    """MCP Server for FlameLang repository analysis and AI-powered tooling"""
    
    def __init__(self, config_path: str = None):
        if config_path is None:
            config_path = os.path.join(os.path.dirname(__file__), 'config.json')
        
        with open(config_path, 'r') as f:
            self.config = json.load(f)
        
        self.name = self.config['name']
        self.version = self.config['version']
        self.tools = {tool['name']: tool for tool in self.config['tools']}
    
    def list_tools(self) -> List[Dict[str, Any]]:
        """List all available MCP tools"""
        return self.config['tools']
    
    def analyze_repository(self, path: str, depth: int = -1) -> Dict[str, Any]:
        """
        Analyzes repository structure and generates metadata
        
        Args:
            path: Root path to analyze
            depth: Maximum depth to traverse (-1 for unlimited)
        
        Returns:
            Dictionary containing repository structure analysis
        """
        repo_path = Path(path)
        if not repo_path.exists():
            return {"error": f"Path does not exist: {path}"}
        
        structure = {
            "path": str(repo_path.resolve()),
            "type": "directory" if repo_path.is_dir() else "file",
            "children": []
        }
        
        if repo_path.is_dir() and depth != 0:
            try:
                for item in sorted(repo_path.iterdir()):
                    # Skip hidden files and common ignore patterns
                    if item.name.startswith('.') and item.name not in ['.github', '.devcontainer']:
                        continue
                    if item.name in ['target', 'node_modules', '__pycache__']:
                        continue
                    
                    child_analysis = self.analyze_repository(
                        str(item), 
                        depth - 1 if depth > 0 else -1
                    )
                    structure["children"].append(child_analysis)
            except PermissionError:
                structure["error"] = "Permission denied"
        
        # Add file metadata
        if repo_path.is_file():
            structure["size"] = repo_path.stat().st_size
            structure["extension"] = repo_path.suffix
            
            # Analyze FlameLang files
            if repo_path.suffix in ['.flame', '.fl', '.rs']:
                structure["flamelang_metadata"] = self._analyze_flamelang_file(repo_path)
        
        return structure
    
    def _analyze_flamelang_file(self, file_path: Path) -> Dict[str, Any]:
        """Analyze a FlameLang or Rust source file for metadata"""
        metadata = {
            "lines": 0,
            "has_quantum": False,
            "has_dna": False,
            "has_wave": False,
            "imports": [],
            "exports": []
        }
        
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                for line in f:
                    metadata["lines"] += 1
                    
                    # Detect FlameLang features
                    if any(kw in line for kw in ['qubit', 'entangle', 'superpos', 'bell_']):
                        metadata["has_quantum"] = True
                    if any(kw in line for kw in ['[ATGC]', 'DnaSequence', 'codon']):
                        metadata["has_dna"] = True
                    if any(kw in line for kw in ['sin~', 'cos~', 'wave', 'frequency']):
                        metadata["has_wave"] = True
                    
                    # Track imports/exports
                    if 'use ' in line or 'import ' in line:
                        metadata["imports"].append(line.strip())
                    if 'pub ' in line and ('fn ' in line or 'struct ' in line):
                        metadata["exports"].append(line.strip())
        except Exception as e:
            metadata["error"] = str(e)
        
        return metadata
    
    def generate_graphview(self, directory: str, output_path: str) -> Dict[str, Any]:
        """
        Generate Obsidian-compatible graph view brain for a directory
        
        Args:
            directory: Directory to generate graph view for
            output_path: Path to save the graph view metadata
        
        Returns:
            Status and metadata about generated graph view
        """
        dir_path = Path(directory)
        if not dir_path.exists() or not dir_path.is_dir():
            return {"error": f"Invalid directory: {directory}"}
        
        # Analyze the directory structure
        analysis = self.analyze_repository(directory, depth=5)
        
        # Generate Obsidian markdown graph view
        graphview = {
            "name": dir_path.name,
            "path": str(dir_path.resolve()),
            "nodes": [],
            "edges": []
        }
        
        # Create nodes for each file and directory
        node_id = 0
        node_map = {}
        
        def process_structure(struct, parent_id=None):
            nonlocal node_id
            current_id = node_id
            node_id += 1
            
            node = {
                "id": current_id,
                "label": Path(struct["path"]).name,
                "type": struct["type"],
                "path": struct["path"]
            }
            
            # Add metadata if available
            if "flamelang_metadata" in struct:
                node["metadata"] = struct["flamelang_metadata"]
            
            graphview["nodes"].append(node)
            node_map[struct["path"]] = current_id
            
            # Create edge from parent
            if parent_id is not None:
                graphview["edges"].append({
                    "from": parent_id,
                    "to": current_id,
                    "type": "contains"
                })
            
            # Process children
            if "children" in struct:
                for child in struct["children"]:
                    process_structure(child, current_id)
        
        process_structure(analysis)
        
        # Generate Obsidian markdown
        markdown_content = f"# {graphview['name']} - GraphView Brain\n\n"
        markdown_content += f"**Path:** `{graphview['path']}`\n\n"
        markdown_content += f"**Total Nodes:** {len(graphview['nodes'])}\n\n"
        markdown_content += "## Structure\n\n"
        
        # Create visual tree
        def create_tree(struct, prefix="", is_last=True):
            content = ""
            name = Path(struct["path"]).name
            connector = "└── " if is_last else "├── "
            
            # Add FlameLang metadata indicators
            indicators = []
            if "flamelang_metadata" in struct:
                meta = struct["flamelang_metadata"]
                if meta.get("has_quantum"):
                    indicators.append("⚛️")
                if meta.get("has_dna"):
                    indicators.append("🧬")
                if meta.get("has_wave"):
                    indicators.append("〰️")
            
            indicator_str = " ".join(indicators)
            content += f"{prefix}{connector}{name} {indicator_str}\n"
            
            if "children" in struct:
                extension = "    " if is_last else "│   "
                for i, child in enumerate(struct["children"]):
                    is_last_child = i == len(struct["children"]) - 1
                    content += create_tree(child, prefix + extension, is_last_child)
            
            return content
        
        markdown_content += "```\n"
        markdown_content += create_tree(analysis)
        markdown_content += "```\n\n"
        
        # Add node details
        markdown_content += "## Node Details\n\n"
        for node in graphview["nodes"]:
            if node["type"] == "file" and "metadata" in node:
                markdown_content += f"### {node['label']}\n\n"
                meta = node["metadata"]
                markdown_content += f"- **Lines:** {meta.get('lines', 0)}\n"
                if meta.get('has_quantum'):
                    markdown_content += "- **Features:** Quantum ⚛️\n"
                if meta.get('has_dna'):
                    markdown_content += "- **Features:** DNA 🧬\n"
                if meta.get('has_wave'):
                    markdown_content += "- **Features:** Wave 〰️\n"
                markdown_content += "\n"
        
        # Save the graph view
        output_file = Path(output_path)
        output_file.parent.mkdir(parents=True, exist_ok=True)
        
        # Save JSON
        json_file = output_file.with_suffix('.json')
        with open(json_file, 'w') as f:
            json.dump(graphview, f, indent=2)
        
        # Save Markdown
        md_file = output_file.with_suffix('.md')
        with open(md_file, 'w') as f:
            f.write(markdown_content)
        
        return {
            "status": "success",
            "json_output": str(json_file),
            "markdown_output": str(md_file),
            "nodes": len(graphview["nodes"]),
            "edges": len(graphview["edges"])
        }
    
    def create_graphview_brain(self, root_directory: str, brain_name: str) -> Dict[str, Any]:
        """
        Creates an ONSIT methodology graphview brain for complete repository
        
        Args:
            root_directory: Root directory of the repository
            brain_name: Name for the graphview brain
        
        Returns:
            Status and paths to generated brain files
        """
        root_path = Path(root_directory)
        if not root_path.exists():
            return {"error": f"Root directory does not exist: {root_directory}"}
        
        # Create output directory
        brains_dir = root_path / 'mcp' / 'graphview' / 'brains'
        brains_dir.mkdir(parents=True, exist_ok=True)
        
        brain_dir = brains_dir / brain_name.replace(' ', '_').lower()
        brain_dir.mkdir(exist_ok=True)
        
        # Generate graph views for all subdirectories
        results = []
        
        for item in root_path.iterdir():
            if not item.is_dir():
                continue
            
            # Skip hidden and build directories
            if item.name.startswith('.') or item.name in ['target', 'node_modules']:
                continue
            
            output_path = brain_dir / f"{item.name}_graphview.md"
            result = self.generate_graphview(str(item), str(output_path))
            results.append({
                "directory": item.name,
                "result": result
            })
        
        # Create master index
        index_path = brain_dir / 'index.md'
        with open(index_path, 'w') as f:
            f.write(f"# {brain_name} - GraphView Brain Arsenal\n\n")
            f.write("## ONSIT Methodology - Repository Navigation\n\n")
            f.write(f"**Repository:** {root_path.name}\n\n")
            f.write("### Generated GraphViews\n\n")
            
            for result in results:
                if result["result"].get("status") == "success":
                    f.write(f"- [[{result['directory']}_graphview|{result['directory']}]] ")
                    f.write(f"({result['result']['nodes']} nodes)\n")
        
        return {
            "status": "success",
            "brain_directory": str(brain_dir),
            "index_file": str(index_path),
            "generated_views": len(results)
        }
    
    def generate_tutorial_video_script(
        self, 
        code_snippet: str, 
        topic: str, 
        target_duration: int = 300
    ) -> Dict[str, Any]:
        """
        Generate AI video script from FlameLang code
        
        Args:
            code_snippet: FlameLang code to explain
            topic: Tutorial topic/title
            target_duration: Target video duration in seconds
        
        Returns:
            Video script with timestamps and narration
        """
        # Calculate approximate narration pace (150 words per minute)
        words_per_second = 150 / 60
        target_words = int(target_duration * words_per_second)
        
        script = {
            "title": topic,
            "duration_seconds": target_duration,
            "scenes": []
        }
        
        # Introduction scene
        script["scenes"].append({
            "timestamp": "00:00",
            "duration": 30,
            "type": "intro",
            "visual": "FlameLang logo and title screen",
            "narration": f"Welcome to FlameLang! In this tutorial, we'll explore {topic}. "
                        "FlameLang is a revolutionary 5-layer biological-quantum-physics "
                        "programming language that transforms code through multiple dimensions."
        })
        
        # Code explanation scene
        lines = code_snippet.strip().split('\n')
        code_duration = min(target_duration - 60, len(lines) * 10)
        
        script["scenes"].append({
            "timestamp": "00:30",
            "duration": code_duration,
            "type": "code_explanation",
            "visual": "Code editor with syntax highlighting",
            "code": code_snippet,
            "narration": self._generate_code_narration(code_snippet, lines)
        })
        
        # 5-Layer transformation visualization
        script["scenes"].append({
            "timestamp": f"00:{30 + code_duration}",
            "duration": 20,
            "type": "layer_visualization",
            "visual": "Animated 5-layer transformation pipeline",
            "narration": "Watch as FlameLang transforms your code through five revolutionary layers: "
                        "English to Hebrew linguistic mapping, Unicode to decimal numeric conversion, "
                        "wave function analysis, DNA codon encoding, and finally LLVM IR generation."
        })
        
        # Conclusion
        remaining = target_duration - 30 - code_duration - 20
        script["scenes"].append({
            "timestamp": f"00:{target_duration - remaining}",
            "duration": remaining,
            "type": "conclusion",
            "visual": "FlameLang ecosystem overview",
            "narration": "You've just learned about {topic} in FlameLang. "
                        "Try it yourself and explore more tutorials to master this "
                        "quantum-biological programming paradigm. Subscribe for more FlameLang content!"
        })
        
        return script
    
    def _generate_code_narration(self, code: str, lines: List[str]) -> str:
        """Generate narration for code explanation"""
        narration_parts = []
        
        for line in lines[:5]:  # First 5 lines for brevity
            line = line.strip()
            if not line or line.startswith('//'):
                continue
            
            # Detect FlameLang constructs
            if 'qubit' in line:
                narration_parts.append(
                    "We declare a quantum bit that can exist in superposition states."
                )
            elif 'entangle' in line or '~>' in line:
                narration_parts.append(
                    "Here we create quantum entanglement between variables using the tilde-arrow operator."
                )
            elif 'sin~' in line or 'cos~' in line:
                narration_parts.append(
                    "This wave operator applies trigonometric transformations in the frequency domain."
                )
            elif '[ATGC]' in line or 'DnaSequence' in line:
                narration_parts.append(
                    "Notice the DNA sequence encoding using biological base pairs."
                )
            else:
                narration_parts.append(
                    f"Let's look at this line: {line[:50]}..."
                )
        
        return " ".join(narration_parts)
    
    def handle_request(self, method: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Handle MCP tool requests"""
        if method == "tools/list":
            return {"tools": self.list_tools()}
        
        elif method == "tools/call":
            tool_name = params.get("name")
            arguments = params.get("arguments", {})
            
            if tool_name == "analyze_repository":
                return self.analyze_repository(**arguments)
            elif tool_name == "generate_graphview":
                return self.generate_graphview(**arguments)
            elif tool_name == "create_graphview_brain":
                return self.create_graphview_brain(**arguments)
            elif tool_name == "generate_tutorial_video_script":
                return self.generate_tutorial_video_script(**arguments)
            else:
                return {"error": f"Unknown tool: {tool_name}"}
        
        return {"error": f"Unknown method: {method}"}


def main():
    """Main entry point for MCP server"""
    server = FlameLangMCPServer()
    
    print(f"🔥 {server.name} v{server.version}")
    print("MCP Server running...")
    print(f"Available tools: {len(server.tools)}")
    
    # Example: Create graphview brain for current repository
    if len(sys.argv) > 1 and sys.argv[1] == "create-brain":
        repo_path = sys.argv[2] if len(sys.argv) > 2 else "."
        brain_name = sys.argv[3] if len(sys.argv) > 3 else "FlameLang Repository"
        
        print(f"\nCreating graphview brain for: {repo_path}")
        result = server.create_graphview_brain(repo_path, brain_name)
        print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
