#!/usr/bin/env python3
"""
FlameLang Training Module: Descriptive Statistics
Generates vector embeddings for statistical concepts across 5 layers
"""

import os
import yaml
import json
import numpy as np
from pathlib import Path
from sentence_transformers import SentenceTransformer

def load_corpus():
    """Load the training corpus from YAML"""
    corpus_path = os.environ.get('CORPUS_PATH', '/app/corpus.yml')
    with open(corpus_path, 'r') as f:
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
    
    # Embed training examples
    if 'examples' in corpus:
        for idx, example in enumerate(corpus['examples']):
            input_text = example.get('input', '')
            embedding = model.encode(input_text)
            embeddings[f"example_{idx}"] = embedding.tolist()
    
    return embeddings

def save_vectors(embeddings, output_dir):
    """Save embeddings to disk"""
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Save as numpy array
    vector_array = np.array(list(embeddings.values()))
    np.save(output_dir / 'embeddings.npy', vector_array)
    
    # Save metadata
    metadata = {
        'keys': list(embeddings.keys()),
        'shape': vector_array.shape,
        'model': 'sentence-transformers/all-MiniLM-L6-v2'
    }
    with open(output_dir / 'metadata.json', 'w') as f:
        json.dump(metadata, f, indent=2)
    
    print(f"✓ Saved {len(embeddings)} embeddings to {output_dir}")

def main():
    print("=" * 60)
    print("FlameLang Training: Descriptive Statistics")
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
    
    # Generate embeddings
    print("\n[3/3] Generating embeddings...")
    embeddings = generate_embeddings(corpus, model)
    print(f"      Generated {len(embeddings)} embeddings")
    
    # Save outputs
    output_dir = os.environ.get('VECTOR_OUTPUT', '/app/vectors')
    save_vectors(embeddings, output_dir)
    
    print("\n" + "=" * 60)
    print("Training complete! ✓")
    print("=" * 60)

if __name__ == '__main__':
    main()
