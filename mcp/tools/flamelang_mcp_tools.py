#!/usr/bin/env python3
"""
FlameLang MCP Tools Integration
Unified interface for all FlameLang MCP agent tools
"""

import sys
import os
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from server.mcp_server import FlameLangMCPServer
from ollama.flamelang_ollama_client import OllamaClient, FlameLangCompilerWithOllama
from video.video_generator import VideoScriptGenerator


class FlameLangMCPTools:
    """Unified interface for FlameLang MCP tools"""
    
    def __init__(self, repo_root: str = None):
        self.repo_root = repo_root or os.getcwd()
        self.mcp_server = FlameLangMCPServer()
        self.ollama_client = OllamaClient()
        self.video_generator = VideoScriptGenerator()
        self.compiler = FlameLangCompilerWithOllama()
    
    def create_complete_arsenal(self, brain_name: str = "FlameLang Repository Brain") -> dict:
        """
        Create complete graphview brain arsenal for the repository
        
        This implements the ONSIT methodology by creating graph views for
        each directory and subdirectory in the repository source tree.
        
        Args:
            brain_name: Name for the graphview brain collection
        
        Returns:
            Dictionary with status and generated files
        """
        print(f"🔥 Creating GraphView Brain Arsenal for FlameLang Repository")
        print(f"   Repository: {self.repo_root}")
        print(f"   Brain Name: {brain_name}\n")
        
        # Step 1: Analyze complete repository structure
        print("Step 1: Analyzing repository structure...")
        repo_analysis = self.mcp_server.analyze_repository(self.repo_root, depth=-1)
        print(f"✓ Analyzed {self._count_nodes(repo_analysis)} files and directories\n")
        
        # Step 2: Create graphview brain for repository
        print("Step 2: Generating graphview brains...")
        brain_result = self.mcp_server.create_graphview_brain(
            self.repo_root,
            brain_name
        )
        
        if brain_result.get("status") == "success":
            print(f"✓ Generated {brain_result['generated_views']} graph views")
            print(f"  Brain directory: {brain_result['brain_directory']}")
            print(f"  Index file: {brain_result['index_file']}\n")
        else:
            print(f"✗ Error: {brain_result.get('error', 'Unknown error')}\n")
        
        # Step 3: Generate individual graph views for key directories
        print("Step 3: Creating detailed graph views...")
        key_dirs = ['src', 'mcp', 'docs', 'examples']
        detailed_views = []
        
        for dir_name in key_dirs:
            dir_path = Path(self.repo_root) / dir_name
            if dir_path.exists():
                output_path = Path(brain_result['brain_directory']) / f"{dir_name}_detailed.md"
                result = self.mcp_server.generate_graphview(str(dir_path), str(output_path))
                
                if result.get('status') == 'success':
                    detailed_views.append(dir_name)
                    print(f"  ✓ {dir_name}: {result['nodes']} nodes")
        
        print(f"\n✓ Generated {len(detailed_views)} detailed views\n")
        
        return {
            "status": "success",
            "repository": self.repo_root,
            "brain_name": brain_name,
            "brain_directory": brain_result.get('brain_directory'),
            "total_views": brain_result.get('generated_views', 0),
            "detailed_views": detailed_views,
            "index_file": brain_result.get('index_file')
        }
    
    def compile_with_ai_assistance(
        self, 
        source_file: str,
        model: str = "codellama"
    ) -> dict:
        """
        Compile FlameLang file with Ollama AI assistance
        
        Args:
            source_file: Path to FlameLang source file
            model: Ollama model to use
        
        Returns:
            Compilation results with AI insights
        """
        print(f"🔥 Compiling FlameLang with AI Assistance")
        print(f"   Source: {source_file}")
        print(f"   AI Model: {model}\n")
        
        if not self.ollama_client.is_available():
            print("⚠️  Ollama not available - continuing without AI assistance")
            print("   To enable AI features, install and run Ollama:")
            print("   https://ollama.ai\n")
        
        result = self.compiler.compile_with_assistance(source_file, model=model)
        
        if "ai_insights" in result:
            insights = result["ai_insights"]
            
            if "explanation" in insights:
                print("AI Code Explanation:")
                print(insights["explanation"][:200] + "...\n")
            
            if "suggestions" in insights:
                print("AI Improvement Suggestions:")
                print(insights["suggestions"][:200] + "...\n")
            
            if "test_cases" in insights:
                print(f"Generated {len(insights['test_cases'])} test cases\n")
        
        return result
    
    def generate_tutorial_videos(
        self, 
        topics: list = None,
        code_examples: dict = None
    ) -> dict:
        """
        Generate AI video tutorial scripts for teaching FlameLang
        
        Args:
            topics: List of tutorial topics
            code_examples: Dictionary mapping topics to code examples
        
        Returns:
            Dictionary with generated video scripts
        """
        if topics is None:
            topics = [
                "Getting Started with FlameLang",
                "Quantum Entanglement in FlameLang",
                "DNA Encoding and Biological Computing",
                "Wave Functions and Frequency Analysis",
                "5-Layer Transformation Pipeline"
            ]
        
        if code_examples is None:
            code_examples = {
                "Getting Started with FlameLang": """
// Hello FlameLang
let message = "Hello, Quantum World!";
print(message);
""",
                "Quantum Entanglement in FlameLang": """
qubit q1;
qubit q2;
entangle q1 ~> q2;
H q1;
bell_phi+ q1 q2;
""",
                "DNA Encoding and Biological Computing": """
let dna = [ATGCATGC];
let codon = encode(dna);
let result = transcribe(codon);
""",
                "Wave Functions and Frequency Analysis": """
let wave = sin~(2 * PI * frequency);
let amplitude = measure(wave);
let phase = angle(wave);
""",
                "5-Layer Transformation Pipeline": """
// This code flows through all 5 layers:
// 1. Linguistic: English → Hebrew
// 2. Numeric: Unicode → Decimal
// 3. Wave: c=2πr → Hz
// 4. DNA: Freq → Codon
// 5. LLVM: IR generation
qubit q;
entangle q ~> sin~([ATGC]);
"""
            }
        
        print(f"🔥 Generating AI Video Tutorial Scripts")
        print(f"   Topics: {len(topics)}\n")
        
        results = {}
        output_dir = Path(self.repo_root) / "mcp" / "video" / "scripts"
        
        for topic in topics:
            print(f"Generating: {topic}")
            
            code = code_examples.get(topic, "// Example code")
            
            script = self.video_generator.generate_tutorial_script(
                topic=topic,
                code_examples=[code],
                learning_objectives=[
                    f"Understand {topic.lower()}",
                    "Write FlameLang code",
                    "Explore the 5-layer pipeline"
                ],
                difficulty="beginner",
                target_duration=300
            )
            
            output_file = self.video_generator.save_script(script, str(output_dir))
            results[topic] = {
                "script_file": output_file,
                "duration": script["metadata"]["duration_seconds"],
                "scenes": len(script["scenes"])
            }
            
            print(f"  ✓ Saved to {output_file}")
            print(f"    {script['metadata']['duration_seconds']}s, {len(script['scenes'])} scenes\n")
        
        return {
            "status": "success",
            "generated_videos": len(results),
            "output_directory": str(output_dir),
            "videos": results
        }
    
    def _count_nodes(self, structure: dict) -> int:
        """Recursively count nodes in structure"""
        count = 1
        for child in structure.get("children", []):
            count += self._count_nodes(child)
        return count
    
    def print_status(self):
        """Print status of all MCP tools"""
        print("🔥 FlameLang MCP Tools Status\n")
        print(f"Repository: {self.repo_root}")
        print(f"MCP Server: {self.mcp_server.name} v{self.mcp_server.version}")
        print(f"Ollama Available: {self.ollama_client.is_available()}")
        
        if self.ollama_client.is_available():
            models = self.ollama_client.list_models()
            print(f"Ollama Models: {', '.join(models) if models else 'None'}")
        
        print(f"Video Generator: Ready")
        print(f"\nAvailable Tools: {len(self.mcp_server.tools)}")
        for tool_name in self.mcp_server.tools:
            print(f"  - {tool_name}")


def main():
    """Main CLI interface"""
    import argparse
    
    parser = argparse.ArgumentParser(
        description="FlameLang MCP Tools - AI-powered development tools"
    )
    parser.add_argument(
        "command",
        choices=["status", "create-arsenal", "compile", "generate-videos", "all"],
        help="Command to execute"
    )
    parser.add_argument(
        "--repo",
        default=".",
        help="Repository root path"
    )
    parser.add_argument(
        "--source",
        help="Source file for compilation"
    )
    parser.add_argument(
        "--model",
        default="codellama",
        help="Ollama model to use"
    )
    
    args = parser.parse_args()
    
    tools = FlameLangMCPTools(repo_root=args.repo)
    
    if args.command == "status":
        tools.print_status()
    
    elif args.command == "create-arsenal":
        result = tools.create_complete_arsenal()
        print("\n" + "="*60)
        print("ARSENAL CREATION COMPLETE")
        print("="*60)
        print(f"Brain Directory: {result['brain_directory']}")
        print(f"Index File: {result['index_file']}")
        print(f"\nOpen the index file in Obsidian to explore the graph view!")
    
    elif args.command == "compile":
        if not args.source:
            print("Error: --source required for compile command")
            sys.exit(1)
        tools.compile_with_ai_assistance(args.source, model=args.model)
    
    elif args.command == "generate-videos":
        result = tools.generate_tutorial_videos()
        print("\n" + "="*60)
        print("VIDEO GENERATION COMPLETE")
        print("="*60)
        print(f"Generated {result['generated_videos']} video scripts")
        print(f"Output Directory: {result['output_directory']}")
    
    elif args.command == "all":
        print("="*60)
        print("EXECUTING ALL OPERATIONS")
        print("="*60 + "\n")
        
        # Status
        tools.print_status()
        print("\n" + "-"*60 + "\n")
        
        # Create arsenal
        arsenal_result = tools.create_complete_arsenal()
        print("\n" + "-"*60 + "\n")
        
        # Generate videos
        video_result = tools.generate_tutorial_videos()
        
        print("\n" + "="*60)
        print("ALL OPERATIONS COMPLETE")
        print("="*60)
        print(f"\nGraphView Arsenal: {arsenal_result['brain_directory']}")
        print(f"Video Scripts: {video_result['output_directory']}")


if __name__ == "__main__":
    main()
