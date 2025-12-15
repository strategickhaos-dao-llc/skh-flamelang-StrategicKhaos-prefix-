#!/usr/bin/env python3
"""
FlameLang Ollama Integration
Provides inline LLM assistance for FlameLang code compilation and development
"""

import json
import requests
from typing import Dict, List, Any, Optional


class OllamaClient:
    """Client for Ollama LLM integration with FlameLang"""
    
    def __init__(self, base_url: str = "http://localhost:11434"):
        self.base_url = base_url
        self.api_endpoint = f"{base_url}/api"
    
    def is_available(self) -> bool:
        """Check if Ollama is running and available"""
        try:
            response = requests.get(f"{self.base_url}/api/tags", timeout=2)
            return response.status_code == 200
        except:
            return False
    
    def list_models(self) -> List[str]:
        """List available Ollama models"""
        try:
            response = requests.get(f"{self.api_endpoint}/tags")
            if response.status_code == 200:
                data = response.json()
                return [model["name"] for model in data.get("models", [])]
        except:
            pass
        return []
    
    def generate(
        self, 
        prompt: str, 
        model: str = "codellama",
        system: Optional[str] = None,
        context: Optional[List[int]] = None
    ) -> Dict[str, Any]:
        """
        Generate completion using Ollama
        
        Args:
            prompt: Input prompt for the model
            model: Model name to use
            system: System prompt for context
            context: Previous conversation context
        
        Returns:
            Response from Ollama with generated text
        """
        payload = {
            "model": model,
            "prompt": prompt,
            "stream": False
        }
        
        if system:
            payload["system"] = system
        if context:
            payload["context"] = context
        
        try:
            response = requests.post(
                f"{self.api_endpoint}/generate",
                json=payload,
                timeout=120
            )
            
            if response.status_code == 200:
                return response.json()
            else:
                return {"error": f"API error: {response.status_code}"}
        except Exception as e:
            return {"error": str(e)}
    
    def explain_flamelang_code(
        self, 
        code: str, 
        model: str = "codellama"
    ) -> str:
        """
        Get explanation of FlameLang code using Ollama
        
        Args:
            code: FlameLang code snippet
            model: Ollama model to use
        
        Returns:
            Explanation text
        """
        system_prompt = """You are an expert in FlameLang, a revolutionary 5-layer 
biological-quantum-physics programming language. FlameLang transforms code through:
1. Linguistic Layer: English → Hebrew mapping
2. Numeric Layer: Unicode → Decimal conversion  
3. Wave Layer: Frequency analysis (c=2πr → Hz)
4. DNA Layer: Biological codon encoding
5. LLVM Layer: Machine code generation

Explain the provided FlameLang code clearly and concisely."""
        
        prompt = f"Explain this FlameLang code:\n\n```flamelang\n{code}\n```"
        
        result = self.generate(prompt, model=model, system=system_prompt)
        
        if "response" in result:
            return result["response"]
        else:
            return result.get("error", "No response generated")
    
    def suggest_improvements(
        self, 
        code: str, 
        model: str = "codellama"
    ) -> str:
        """
        Get improvement suggestions for FlameLang code
        
        Args:
            code: FlameLang code snippet
            model: Ollama model to use
        
        Returns:
            Improvement suggestions
        """
        system_prompt = """You are a FlameLang code optimization expert. 
Analyze the code and suggest improvements for:
- Quantum state management
- DNA encoding efficiency
- Wave function optimization
- Memory and performance"""
        
        prompt = f"Suggest improvements for this FlameLang code:\n\n```flamelang\n{code}\n```"
        
        result = self.generate(prompt, model=model, system=system_prompt)
        
        if "response" in result:
            return result["response"]
        else:
            return result.get("error", "No response generated")
    
    def generate_test_cases(
        self, 
        code: str, 
        model: str = "codellama"
    ) -> List[Dict[str, Any]]:
        """
        Generate test cases for FlameLang code
        
        Args:
            code: FlameLang code snippet
            model: Ollama model to use
        
        Returns:
            List of test cases
        """
        system_prompt = """You are a FlameLang testing expert. 
Generate comprehensive test cases in JSON format with:
- input: test input
- expected_output: expected result
- description: what the test validates"""
        
        prompt = f"""Generate 3-5 test cases for this FlameLang code in JSON array format:

```flamelang
{code}
```

Return only valid JSON array."""
        
        result = self.generate(prompt, model=model, system=system_prompt)
        
        if "response" in result:
            try:
                # Try to parse JSON from response
                response_text = result["response"]
                # Find JSON array in response
                start = response_text.find('[')
                end = response_text.rfind(']') + 1
                if start >= 0 and end > start:
                    json_str = response_text[start:end]
                    return json.loads(json_str)
            except:
                pass
        
        # Return default test cases if parsing fails
        return [
            {
                "input": "default_input",
                "expected_output": "default_output",
                "description": "Basic functionality test"
            }
        ]
    
    def inline_completion(
        self, 
        partial_code: str, 
        cursor_position: int,
        model: str = "codellama"
    ) -> str:
        """
        Provide inline code completion suggestions
        
        Args:
            partial_code: Code up to cursor position
            cursor_position: Current cursor position
            model: Ollama model to use
        
        Returns:
            Completion suggestion
        """
        system_prompt = """You are a FlameLang code completion assistant.
Provide the most likely continuation of the code. Return only the completion,
no explanations."""
        
        prompt = f"Complete this FlameLang code:\n\n```flamelang\n{partial_code}"
        
        result = self.generate(prompt, model=model, system=system_prompt)
        
        if "response" in result:
            completion = result["response"].strip()
            # Remove markdown if present
            if completion.startswith('```'):
                lines = completion.split('\n')
                completion = '\n'.join(lines[1:-1] if len(lines) > 2 else lines)
            return completion
        
        return ""


class FlameLangCompilerWithOllama:
    """FlameLang compiler enhanced with Ollama AI assistance"""
    
    def __init__(self, ollama_url: str = "http://localhost:11434"):
        self.ollama = OllamaClient(ollama_url)
        self.compilation_context = []
    
    def compile_with_assistance(
        self, 
        source_file: str,
        model: str = "codellama",
        optimization_level: int = 2
    ) -> Dict[str, Any]:
        """
        Compile FlameLang code with AI assistance
        
        Args:
            source_file: Path to FlameLang source file
            model: Ollama model for assistance
            optimization_level: Optimization level (0-3)
        
        Returns:
            Compilation results with AI insights
        """
        try:
            with open(source_file, 'r') as f:
                source_code = f.read()
        except Exception as e:
            return {"error": f"Failed to read source file: {e}"}
        
        result = {
            "source_file": source_file,
            "optimization_level": optimization_level,
            "ai_model": model,
            "compilation_status": "pending",
            "ai_insights": {}
        }
        
        # Check if Ollama is available
        if not self.ollama.is_available():
            result["ai_insights"]["warning"] = "Ollama not available, compiling without AI assistance"
        else:
            # Get AI insights
            result["ai_insights"]["explanation"] = self.ollama.explain_flamelang_code(
                source_code, model
            )
            
            result["ai_insights"]["suggestions"] = self.ollama.suggest_improvements(
                source_code, model
            )
            
            result["ai_insights"]["test_cases"] = self.ollama.generate_test_cases(
                source_code, model
            )
        
        # TODO: Actual FlameLang compilation through 5-layer pipeline
        # This would integrate with the Rust compiler
        result["compilation_status"] = "ai_analysis_complete"
        result["next_steps"] = [
            "Layer 1: Linguistic transformation (English → Hebrew)",
            "Layer 2: Numeric conversion (Unicode → Decimal)",
            "Layer 3: Wave analysis (c=2πr → Hz)",
            "Layer 4: DNA encoding (Freq → Codon)",
            "Layer 5: LLVM IR generation"
        ]
        
        return result


def main():
    """Test Ollama integration"""
    client = OllamaClient()
    
    print("🔥 FlameLang Ollama Integration")
    print(f"Ollama available: {client.is_available()}")
    
    if client.is_available():
        models = client.list_models()
        print(f"Available models: {', '.join(models)}")
        
        # Test with sample FlameLang code
        sample_code = """
qubit q1;
qubit q2;
entangle q1 ~> q2;
H q1;
bell_phi+ q1 q2;
"""
        
        print("\n--- Code Explanation ---")
        explanation = client.explain_flamelang_code(sample_code)
        print(explanation)
        
        print("\n--- Improvement Suggestions ---")
        suggestions = client.suggest_improvements(sample_code)
        print(suggestions)


if __name__ == "__main__":
    main()
