# FlameLang Training Pipeline

**FlameLang v2.0.0** - 5-Layer Transformation Training Corpus
`English → Hebrew → Unicode → Wave → DNA → LLVM`

This directory contains the modular training infrastructure for FlameLang, with each module providing isolated vector generation for different educational concepts.

## 🏗️ Architecture

```
/flamelang-training/
├── docker-compose.yml          # Orchestrates all training modules
├── README.md                   # This file
└── modules/
    ├── mod1-descriptive-stats/ # Statistics fundamentals
    │   ├── Dockerfile          # Container definition
    │   ├── corpus.yml          # Training data (5-layer concepts)
    │   ├── train.py            # Vector generation script
    │   └── vectors/            # Output: embeddings & metadata
    └── mod2-bar-charts/        # Data visualization
        ├── Dockerfile          # Container definition
        ├── corpus.yml          # Training data (Python→FlameLang)
        ├── train.py            # Vector generation script
        └── vectors/            # Output: embeddings & metadata
```

## 📚 Modules

### Module 1: Descriptive Statistics
**Focus**: Core statistical concepts (mean, median, mode, variance, standard deviation)

**Layers**:
- **English**: Natural language definitions
- **Hebrew**: Mathematical symbols (μ, σ, σ²)
- **Unicode**: Operators (∑, √, ∈)
- **Wave**: Dimensional analysis
- **DNA**: Computational primitives

### Module 2: Bar Charts
**Focus**: Categorical data visualization using bar charts

**Special Features**:
- Python/SeaBorn to FlameLang glyph transformations
- Visual encoding principles
- Grouped and frequency bar charts
- Complete 5-layer mapping for visualization concepts

**Layers**:
- **English**: Visualization concepts
- **Hebrew**: Chart glyphs (📊, █, ▓, │, ─)
- **Unicode**: Visual operators (→, ⊕, ∀, ≡)
- **Wave**: Dimensional analysis ([length], [pixels/value_unit])
- **DNA**: Rendering primitives and functions

## 🚀 Quick Start

### Prerequisites
- Docker and Docker Compose
- 4GB+ available disk space
- Internet connection (for downloading models)

### Environment Selection

You can run this in three environments:

1. **GitHub Codespace** (Recommended for development)
   ```bash
   # Already in the correct directory
   cd flamelang-training
   docker-compose up --build
   ```

2. **Local Cluster** (Athena/Nova/Lyra)
   ```bash
   # Clone repository first
   git clone <repo-url>
   cd flamelang-training
   docker-compose up --build
   ```

3. **Claude's Environment** (For building/testing)
   ```bash
   # Files are already prepared
   docker-compose up --build
   ```

### Running Individual Modules

```bash
# Run all modules
docker-compose up --build

# Run specific module
docker-compose up --build mod1-descriptive-stats
docker-compose up --build mod2-bar-charts

# Run in background
docker-compose up -d

# View logs
docker-compose logs -f mod2-bar-charts

# Stop all modules
docker-compose down
```

## 📊 Output Format

Each module generates:

1. **Vector Embeddings** (`.npy` files)
   - NumPy arrays of embeddings
   - Dimensions: 384 (sentence-transformers/all-MiniLM-L6-v2)
   - L2 normalized

2. **Metadata** (`.json` files)
   - Embedding keys/labels
   - Model information
   - Array shapes and dimensions

### Example Output

```
modules/mod2-bar-charts/vectors/
├── mod2-bar-charts.npy          # Vector array
└── mod2-bar-charts-meta.json    # Metadata
```

**Metadata Structure**:
```json
{
  "keys": [
    "concept_bar_chart",
    "concept_categorical_variable",
    "example_0_python",
    "example_0_flamelang",
    "example_0_layer_english",
    "principle_position_accuracy"
  ],
  "shape": [50, 384],
  "model": "sentence-transformers/all-MiniLM-L6-v2",
  "dimensions": 384
}
```

## 🎯 Corpus Structure

Each `corpus.yml` contains:

### Required Fields
```yaml
module:
  id: "module-name"
  version: "1.0.0"
  title: "Human-readable title"
  description: "Module description"
  flamelang_version: "2.0.0"

layers:
  english:    # Natural language
  hebrew:     # Mathematical/symbolic
  unicode:    # Operators
  wave:       # Dimensional analysis
  dna:        # Computational primitives

embedding:
  model: "sentence-transformers/all-MiniLM-L6-v2"
  dimensions: 384
  normalization: "l2"
```

### Module 2 Extensions
```yaml
flamelang_examples:  # Python → FlameLang transformations
  - title: "Example name"
    python_code: |
      # Python/SeaBorn code
    flamelang_code: |
      # FlameLang with glyphs
    layer_mapping:
      english: "Natural description"
      hebrew: "Symbolic notation"
      # ... etc

visualization_theory:  # Domain-specific concepts
  encoding_principles:
  perceptual_constraints:
```

## 🔧 Customization

### Adding a New Module

1. **Create directory**:
   ```bash
   mkdir -p modules/mod3-your-module/vectors
   ```

2. **Create `Dockerfile`**:
   ```dockerfile
   FROM python:3.11-slim
   # ... (see existing modules for template)
   ```

3. **Create `corpus.yml`**:
   ```yaml
   module:
     id: mod3-your-module
     # ... (follow existing structure)
   ```

4. **Create `train.py`**:
   ```python
   # Use existing train.py as template
   ```

5. **Update `docker-compose.yml`**:
   ```yaml
   services:
     mod3-your-module:
       build:
         context: ./modules/mod3-your-module
       # ... (follow existing pattern)
   ```

### Modifying Training

Edit `train.py` in each module to:
- Change embedding model
- Add preprocessing steps
- Modify vector storage format
- Add validation logic

## 🧪 Testing

### Verify Module Build
```bash
cd modules/mod2-bar-charts
docker build -t test-mod2 .
docker run --rm test-mod2
```

### Check Vector Output
```bash
# After running docker-compose
ls -lh modules/mod2-bar-charts/vectors/
python3 -c "import numpy as np; arr = np.load('modules/mod2-bar-charts/vectors/mod2-bar-charts.npy'); print(f'Shape: {arr.shape}')"
```

### Validate Corpus YAML
```bash
python3 -c "import yaml; yaml.safe_load(open('modules/mod2-bar-charts/corpus.yml'))"
```

## 📖 FlameLang Glyph Reference

### Module 2 (Bar Charts) Glyphs

| Glyph | Unicode | Meaning | Usage |
|-------|---------|---------|-------|
| 📊 | U+1F4CA | Bar chart | Chart type indicator |
| █ | U+2588 | Bar element | Filled bar |
| ▓ | U+2593 | Bar pattern | Shaded bar |
| │ | U+2502 | Y-axis | Vertical axis |
| ─ | U+2500 | X-axis | Horizontal axis |
| ├ | U+251C | Axis tick | Tick mark |
| ℎ | U+210E | Height | Height variable |
| 𝑐 | U+1D450 | Category | Category variable |
| → | U+2192 | Maps to | Transformation |
| ⊕ | U+2295 | Concatenate | Combine categories |
| ∀ | U+2200 | For all | Iteration |
| ≡ | U+2261 | Equivalent | Visual equivalence |
| ⊢ | U+22A2 | Render | Produce visualization |
| ⟨⟩ | U+27E8/U+27E9 | Import | Module import |
| ⟦⟧ | U+27E6/U+27E7 | Variable | Variable declaration |
| ≔ | U+2254 | Define | Assignment |
| ⊳ | U+22B3 | Pipeline | Function chaining |

## 🔍 Troubleshooting

### Issue: Container fails to build
```bash
# Check Docker is running
docker info

# Rebuild without cache
docker-compose build --no-cache mod2-bar-charts
```

### Issue: Out of memory
```bash
# Reduce model size or run modules sequentially
docker-compose up mod1-descriptive-stats
docker-compose up mod2-bar-charts
```

### Issue: Vector files not created
```bash
# Check permissions
chmod -R 755 modules/*/vectors/

# Check logs
docker-compose logs mod2-bar-charts
```

## 📦 Dependencies

Each module container includes:
- Python 3.11
- NumPy >= 1.24.0
- pandas >= 2.0.0
- PyYAML >= 6.0
- sentence-transformers >= 2.2.0

Module 2 additionally includes:
- matplotlib >= 3.7.0
- seaborn >= 0.12.0

## 🤝 Contributing

To add educational content:

1. Create a new module following the structure
2. Define the 5-layer transformation in `corpus.yml`
3. Add training examples
4. Implement `train.py` for vector generation
5. Test with Docker Compose
6. Document glyphs and principles

## 📄 License

See LICENSE file in repository root.

## 🔗 Related Resources

- FlameLang Specification: See main repository README
- Zybooks Integration: For educational content sources
- Swarm Intelligence: Part of StrategicKhaos ecosystem

---

**StrategicKhaos DAO LLC** | FlameLang v2.0.0 | Biological Compilation
