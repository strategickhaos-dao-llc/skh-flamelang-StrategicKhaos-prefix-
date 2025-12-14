#!/usr/bin/env python3
"""
FlameLang Training Module: Bar Charts
Generates vector embeddings for visualization concepts across 5 layers
Includes Python/SeaBorn to FlameLang glyph transformations
"""

import os
import yaml
import json
import numpy as np
from pathlib import Path
from sentence_transformers import SentenceTransformer

def load_corpus():
    """Load the training corpus from YAML"""
    with open('/app/corpus.yml', 'r') as f:
        return yaml.safe_load(f)

def generate_embeddings(corpus, model):
    """Generate embeddings for all concepts in the corpus"""
    embeddings = {}
    
    # Extract concepts from English layer
    if 'layers' in corpus and 'english' in corpus['layers']:
        concepts = corpus['layers']['english'].get('concepts', [])
        for concept in concepts:
            name = concept.get('name', '')
            definition = concept.get('definition', '')
            text = f"{name}: {definition}"
            embedding = model.encode(text)
            embeddings[f"concept_{name}"] = embedding.tolist()
    
    # Embed FlameLang examples (Python to FlameLang transformations)
    if 'flamelang_examples' in corpus:
        for idx, example in enumerate(corpus['flamelang_examples']):
            title = example.get('title', '')
            python_code = example.get('python_code', '')
            flamelang_code = example.get('flamelang_code', '')
            
            # Embed Python code
            embeddings[f"example_{idx}_python"] = model.encode(python_code).tolist()
            
            # Embed FlameLang code
            embeddings[f"example_{idx}_flamelang"] = model.encode(flamelang_code).tolist()
            
            # Embed the title/description
            embeddings[f"example_{idx}_title"] = model.encode(title).tolist()
            
            # Embed layer mappings
            if 'layer_mapping' in example:
                for layer, description in example['layer_mapping'].items():
                    key = f"example_{idx}_layer_{layer}"
                    embeddings[key] = model.encode(description).tolist()
    
    # Embed visualization principles
    if 'visualization_theory' in corpus:
        theory = corpus['visualization_theory']
        if 'encoding_principles' in theory:
            for idx, principle in enumerate(theory['encoding_principles']):
                name = principle.get('name', '')
                desc = principle.get('description', '')
                text = f"{name}: {desc}"
                embeddings[f"principle_{name}"] = model.encode(text).tolist()
    
    return embeddings

def save_vectors(embeddings, output_dir):
    """Save embeddings to disk"""
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Save as numpy array
    vector_array = np.array(list(embeddings.values()))
    np.save(output_dir / 'mod2-bar-charts.npy', vector_array)
    
    # Save metadata
    metadata = {
        'keys': list(embeddings.keys()),
        'shape': vector_array.shape,
        'model': 'sentence-transformers/all-MiniLM-L6-v2',
        'dimensions': 384
    }
    with open(output_dir / 'mod2-bar-charts-meta.json', 'w') as f:
        json.dump(metadata, f, indent=2)
    
    print(f"✓ Saved {len(embeddings)} embeddings to {output_dir}")

def main():
    print("=" * 60)
    print("FlameLang Training: Bar Charts")
    print("=" * 60)
    
    # Load corpus
    print("\n[1/3] Loading corpus...")
    corpus = load_corpus()
    module_id = corpus['module']['id']
    print(f"      Module: {module_id}")
    print(f"      Version: {corpus['module']['version']}")
    
    # Initialize model
    print("\n[2/3] Initializing embedding model...")
    model_name = corpus['embedding']['model']
    model = SentenceTransformer(model_name)
    print(f"      Model: {model_name}")
    print(f"      Dimensions: {corpus['embedding']['dimensions']}")
    
    # Generate embeddings
    print("\n[3/3] Generating embeddings...")
    embeddings = generate_embeddings(corpus, model)
    print(f"      Generated {len(embeddings)} embeddings")
    print(f"      - Concepts: English layer")
    print(f"      - Examples: Python ↔ FlameLang")
    print(f"      - Layers: 5-layer transformations")
    print(f"      - Theory: Visualization principles")
    
    # Save outputs
    output_dir = os.environ.get('VECTOR_OUTPUT', '/app/vectors')
    save_vectors(embeddings, output_dir)
    
    print("\n" + "=" * 60)
    print("Training complete! ✓")
    print("=" * 60)

if __name__ == '__main__':
    main()
