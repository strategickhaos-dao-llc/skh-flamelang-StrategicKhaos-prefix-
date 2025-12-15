#!/usr/bin/env python3
"""
FlameLang AI Video Tutorial Generator
Creates video scripts and metadata for teaching FlameLang concepts
"""

import json
import os
from pathlib import Path
from typing import Dict, List, Any, Optional
from datetime import datetime


class VideoScriptGenerator:
    """Generates AI video scripts for FlameLang tutorials"""
    
    def __init__(self):
        self.templates = self._load_templates()
    
    def _load_templates(self) -> Dict[str, Any]:
        """Load video script templates"""
        return {
            "intro": {
                "duration": 30,
                "visual_style": "animated_logo",
                "music": "upbeat_tech"
            },
            "code_walkthrough": {
                "duration_per_line": 8,
                "visual_style": "syntax_highlighted_editor",
                "animation": "line_by_line_reveal"
            },
            "layer_visualization": {
                "duration": 20,
                "visual_style": "3d_pipeline_animation",
                "layers": [
                    "English → Hebrew",
                    "Unicode → Decimal", 
                    "Wave Analysis",
                    "DNA Encoding",
                    "LLVM IR"
                ]
            },
            "outro": {
                "duration": 15,
                "visual_style": "call_to_action",
                "elements": ["subscribe", "github_link", "documentation"]
            }
        }
    
    def generate_tutorial_script(
        self,
        topic: str,
        code_examples: List[str],
        learning_objectives: List[str],
        difficulty: str = "beginner",
        target_duration: int = 600
    ) -> Dict[str, Any]:
        """
        Generate comprehensive tutorial video script
        
        Args:
            topic: Tutorial topic/title
            code_examples: List of code snippets to explain
            learning_objectives: What viewers will learn
            difficulty: beginner, intermediate, or advanced
            target_duration: Target video length in seconds
        
        Returns:
            Complete video script with scenes and timing
        """
        script = {
            "metadata": {
                "title": f"FlameLang Tutorial: {topic}",
                "topic": topic,
                "difficulty": difficulty,
                "duration_seconds": target_duration,
                "learning_objectives": learning_objectives,
                "created_at": datetime.now().isoformat(),
                "version": "2.0.0"
            },
            "scenes": [],
            "narration_script": [],
            "visual_assets_needed": [],
            "code_assets": code_examples
        }
        
        current_time = 0
        
        # Scene 1: Introduction
        intro_scene = self._create_intro_scene(topic, learning_objectives, current_time)
        script["scenes"].append(intro_scene)
        current_time += intro_scene["duration"]
        
        # Scene 2: Concept Overview
        overview_scene = self._create_overview_scene(topic, difficulty, current_time)
        script["scenes"].append(overview_scene)
        current_time += overview_scene["duration"]
        
        # Scenes 3-N: Code Examples
        for i, code in enumerate(code_examples):
            code_scene = self._create_code_scene(
                code, 
                f"Example {i+1}", 
                current_time,
                difficulty
            )
            script["scenes"].append(code_scene)
            current_time += code_scene["duration"]
        
        # Scene N+1: 5-Layer Transformation
        layer_scene = self._create_layer_visualization_scene(current_time)
        script["scenes"].append(layer_scene)
        current_time += layer_scene["duration"]
        
        # Scene N+2: Practical Application
        practice_scene = self._create_practice_scene(topic, current_time)
        script["scenes"].append(practice_scene)
        current_time += practice_scene["duration"]
        
        # Final Scene: Conclusion
        outro_scene = self._create_outro_scene(topic, learning_objectives, current_time)
        script["scenes"].append(outro_scene)
        
        # Generate full narration script
        script["narration_script"] = self._compile_narration(script["scenes"])
        
        # Collect visual assets needed
        script["visual_assets_needed"] = self._collect_visual_assets(script["scenes"])
        
        return script
    
    def _create_intro_scene(
        self, 
        topic: str, 
        objectives: List[str], 
        start_time: int
    ) -> Dict[str, Any]:
        """Create introduction scene"""
        return {
            "scene_number": 1,
            "title": "Introduction",
            "timestamp": self._format_timestamp(start_time),
            "duration": 30,
            "visual": {
                "type": "animated_intro",
                "elements": [
                    "FlameLang logo animation",
                    "Title card: " + topic,
                    "Particle effects (DNA strands, quantum waves)"
                ]
            },
            "narration": f"""Welcome to FlameLang! I'm excited to teach you about {topic}.
                
FlameLang is a revolutionary programming language that combines quantum computing, 
biological systems, and wave physics into a unified computational framework.

In this tutorial, you'll learn: {', '.join(objectives[:3])}.

Let's dive in!""",
            "background_music": "upbeat_tech_intro.mp3",
            "text_overlays": [
                {"time": 0, "text": f"FlameLang Tutorial: {topic}", "style": "title"},
                {"time": 15, "text": "Subscribe for more!", "style": "subtitle"}
            ]
        }
    
    def _create_overview_scene(
        self, 
        topic: str, 
        difficulty: str, 
        start_time: int
    ) -> Dict[str, Any]:
        """Create concept overview scene"""
        difficulty_context = {
            "beginner": "We'll start with the basics and build from there.",
            "intermediate": "This builds on fundamental FlameLang concepts.",
            "advanced": "This explores advanced features of the FlameLang compiler."
        }
        
        return {
            "scene_number": 2,
            "title": "Concept Overview",
            "timestamp": self._format_timestamp(start_time),
            "duration": 45,
            "visual": {
                "type": "concept_diagram",
                "elements": [
                    "Topic breakdown diagram",
                    "Key concepts highlighted",
                    "Visual metaphors"
                ]
            },
            "narration": f"""Before we look at code, let's understand what {topic} means in FlameLang.

{difficulty_context.get(difficulty, '')}

Remember, FlameLang operates through five transformation layers: 
Linguistic, Numeric, Wave, DNA, and LLVM. Each layer adds a dimension of computation.

Now let's see how this works in practice.""",
            "animations": [
                "Concept diagram fade-in",
                "Layer pipeline visualization"
            ]
        }
    
    def _create_code_scene(
        self, 
        code: str, 
        example_title: str, 
        start_time: int,
        difficulty: str
    ) -> Dict[str, Any]:
        """Create code walkthrough scene"""
        lines = [line for line in code.split('\n') if line.strip()]
        duration = max(60, len(lines) * 10)
        
        narration = self._generate_code_narration(code, difficulty)
        
        return {
            "scene_number": 3,
            "title": example_title,
            "timestamp": self._format_timestamp(start_time),
            "duration": duration,
            "visual": {
                "type": "code_editor",
                "style": "vs_code_dark",
                "elements": [
                    "Syntax highlighted code",
                    "Line-by-line reveal animation",
                    "Zoom on key sections",
                    "Inline annotations"
                ]
            },
            "code": code,
            "narration": narration,
            "animations": [
                "Type-writer effect for code",
                "Highlight key constructs",
                "Visual indicators for quantum/DNA/wave operations"
            ],
            "annotations": self._generate_code_annotations(code)
        }
    
    def _create_layer_visualization_scene(self, start_time: int) -> Dict[str, Any]:
        """Create 5-layer transformation visualization"""
        return {
            "scene_number": 4,
            "title": "5-Layer Transformation Pipeline",
            "timestamp": self._format_timestamp(start_time),
            "duration": 25,
            "visual": {
                "type": "3d_animation",
                "elements": [
                    "5-layer pipeline 3D model",
                    "Data flow visualization",
                    "Each layer highlighted in sequence",
                    "Transformation examples"
                ]
            },
            "narration": """Now watch as your FlameLang code transforms through all five layers.

First, Layer 1: Linguistic transformation maps English concepts to Hebrew semantics.

Layer 2: Numeric conversion translates Unicode to decimal representations.

Layer 3: Wave analysis applies frequency domain transformations.

Layer 4: DNA encoding maps computational states to biological codons.

Finally, Layer 5: LLVM IR generation produces executable machine code.

This multi-dimensional transformation is what makes FlameLang unique!""",
            "animations": [
                "Pipeline 3D rotation",
                "Data flowing through layers",
                "Color-coded transformations"
            ]
        }
    
    def _create_practice_scene(self, topic: str, start_time: int) -> Dict[str, Any]:
        """Create practice/application scene"""
        return {
            "scene_number": 5,
            "title": "Try It Yourself",
            "timestamp": self._format_timestamp(start_time),
            "duration": 30,
            "visual": {
                "type": "split_screen",
                "elements": [
                    "Challenge prompt on left",
                    "Blank code editor on right",
                    "Timer countdown"
                ]
            },
            "narration": f"""Now it's your turn! Pause the video and try implementing {topic} yourself.

Here's a challenge: modify the code we just saw to add your own twist.

When you're ready, unpause to see a solution and compare your approach.

Remember, there's often more than one way to solve a problem in FlameLang!""",
            "interactive_elements": [
                "Pause prompt",
                "Code challenge text"
            ]
        }
    
    def _create_outro_scene(
        self, 
        topic: str, 
        objectives: List[str], 
        start_time: int
    ) -> Dict[str, Any]:
        """Create conclusion/outro scene"""
        return {
            "scene_number": 6,
            "title": "Conclusion",
            "timestamp": self._format_timestamp(start_time),
            "duration": 20,
            "visual": {
                "type": "recap_and_cta",
                "elements": [
                    "Key points summary",
                    "Subscribe button animation",
                    "Links overlay",
                    "Next tutorial teaser"
                ]
            },
            "narration": f"""Congratulations! You've learned about {topic} in FlameLang.

You can now: {', '.join(objectives[:2])}.

If you found this helpful, hit that subscribe button and check out the FlameLang 
documentation linked below.

In the next video, we'll explore more advanced features of the quantum subsystem.

Keep coding, and remember: with FlameLang, you're not just writing code, 
you're programming reality itself. See you next time!""",
            "text_overlays": [
                {"time": 5, "text": "Subscribe!", "style": "cta_button"},
                {"time": 10, "text": "github.com/strategickhaos/flamelang", "style": "link"},
                {"time": 15, "text": "Next: Advanced Quantum Features", "style": "teaser"}
            ],
            "end_screen": {
                "duration": 5,
                "elements": ["channel_logo", "subscribe_button", "video_thumbnails"]
            }
        }
    
    def _generate_code_narration(self, code: str, difficulty: str) -> str:
        """Generate narration for code explanation"""
        lines = [line.strip() for line in code.split('\n') if line.strip()]
        narration_parts = ["Let me walk you through this code."]
        
        for line in lines[:6]:  # First 6 lines
            if line.startswith('//') or line.startswith('#'):
                continue
            
            # Detect FlameLang constructs
            if 'qubit' in line:
                narration_parts.append(
                    "We declare a quantum bit - this can exist in superposition, "
                    "representing multiple states simultaneously."
                )
            elif 'entangle' in line or '~>' in line:
                narration_parts.append(
                    "Here we entangle qubits using the quantum entanglement operator. "
                    "These qubits are now correlated across space."
                )
            elif any(op in line for op in ['sin~', 'cos~', 'tan~']):
                narration_parts.append(
                    "This wave operator transforms data in the frequency domain, "
                    "using the mathematical relationship c equals 2 pi r."
                )
            elif '[ATGC]' in line.upper() or 'DnaSequence' in line:
                narration_parts.append(
                    "Notice the DNA sequence - FlameLang can encode data using "
                    "biological base pairs, just like your own DNA."
                )
            elif 'H ' in line or 'X ' in line or 'CNOT' in line:
                narration_parts.append(
                    "We apply a quantum gate here - these are the fundamental "
                    "operations in quantum computing."
                )
        
        if len(narration_parts) == 1:
            narration_parts.append(
                "This code demonstrates core FlameLang syntax and patterns."
            )
        
        narration_parts.append(
            "Each line is processed through the 5-layer transformation pipeline "
            "to produce efficient machine code."
        )
        
        return " ".join(narration_parts)
    
    def _generate_code_annotations(self, code: str) -> List[Dict[str, Any]]:
        """Generate inline code annotations"""
        annotations = []
        lines = code.split('\n')
        
        for i, line in enumerate(lines):
            line = line.strip()
            if not line:
                continue
            
            if 'qubit' in line:
                annotations.append({
                    "line": i + 1,
                    "text": "⚛️ Quantum declaration",
                    "color": "blue"
                })
            elif '~>' in line:
                annotations.append({
                    "line": i + 1,
                    "text": "🔗 Entanglement operator",
                    "color": "purple"
                })
            elif any(op in line for op in ['sin~', 'cos~']):
                annotations.append({
                    "line": i + 1,
                    "text": "〰️ Wave transformation",
                    "color": "cyan"
                })
            elif '[ATGC]' in line.upper():
                annotations.append({
                    "line": i + 1,
                    "text": "🧬 DNA encoding",
                    "color": "green"
                })
        
        return annotations
    
    def _format_timestamp(self, seconds: int) -> str:
        """Format seconds as MM:SS timestamp"""
        mins = seconds // 60
        secs = seconds % 60
        return f"{mins:02d}:{secs:02d}"
    
    def _compile_narration(self, scenes: List[Dict[str, Any]]) -> List[Dict[str, str]]:
        """Compile full narration script from scenes"""
        narration = []
        for scene in scenes:
            narration.append({
                "timestamp": scene["timestamp"],
                "scene": scene["title"],
                "text": scene.get("narration", "")
            })
        return narration
    
    def _collect_visual_assets(self, scenes: List[Dict[str, Any]]) -> List[str]:
        """Collect all visual assets needed"""
        assets = set()
        for scene in scenes:
            visual = scene.get("visual", {})
            if visual.get("type"):
                assets.add(f"{visual['type']}_template")
            for element in visual.get("elements", []):
                if isinstance(element, str):
                    assets.add(element)
        return sorted(list(assets))
    
    def save_script(self, script: Dict[str, Any], output_dir: str) -> str:
        """Save video script to file"""
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        # Create safe filename
        safe_title = "".join(
            c if c.isalnum() or c in [' ', '-', '_'] else '_' 
            for c in script["metadata"]["title"]
        )
        safe_title = safe_title.replace(' ', '_').lower()
        
        # Save JSON script
        json_file = output_path / f"{safe_title}_script.json"
        with open(json_file, 'w') as f:
            json.dump(script, f, indent=2)
        
        # Save human-readable narration
        narration_file = output_path / f"{safe_title}_narration.txt"
        with open(narration_file, 'w') as f:
            f.write(f"{script['metadata']['title']}\n")
            f.write("=" * len(script['metadata']['title']) + "\n\n")
            
            for item in script['narration_script']:
                f.write(f"[{item['timestamp']}] {item['scene']}\n")
                f.write(f"{item['text']}\n\n")
        
        return str(json_file)


def main():
    """Test video script generation"""
    generator = VideoScriptGenerator()
    
    sample_code = """// FlameLang Quantum Entanglement Example
qubit q1;
qubit q2;
entangle q1 ~> q2;
H q1;  // Hadamard gate
bell_phi+ q1 q2;  // Create Bell state
"""
    
    script = generator.generate_tutorial_script(
        topic="Quantum Entanglement in FlameLang",
        code_examples=[sample_code],
        learning_objectives=[
            "Declare quantum bits",
            "Create entangled states",
            "Apply quantum gates"
        ],
        difficulty="beginner",
        target_duration=300
    )
    
    # Save the script
    output_file = generator.save_script(script, "mcp/video/scripts")
    print(f"✅ Video script generated: {output_file}")
    print(f"Total scenes: {len(script['scenes'])}")
    print(f"Duration: {script['metadata']['duration_seconds']}s")


if __name__ == "__main__":
    main()
