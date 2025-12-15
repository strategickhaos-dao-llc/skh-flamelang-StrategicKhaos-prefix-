# FlameLang Training Pipeline - Quick Start

## 🎯 Answers to Your Questions

**From the problem statement:**

> Are you working in:
> 1. Your GitHub Codespace (`studious-adventure-7g74pq4r6493x9x4`)
> 2. Local cluster (Athena/Nova/Lyra)
> 3. Claude's computer environment (I build it here, you pull it)

**✅ Answer: ALL THREE!**

This infrastructure works in all environments. Choose yours below:

---

## 🚀 Environment 1: GitHub Codespace

You're already in the right place!

```bash
cd flamelang-training
docker compose up --build
```

**What happens:**
1. Builds Docker containers for each module
2. Generates vector embeddings from corpus.yml files
3. Outputs to `modules/*/vectors/` directories

**Time:** ~5-10 minutes (first build, faster subsequent runs)

---

## 🖥️ Environment 2: Local Cluster (Athena/Nova/Lyra)

```bash
# On your local machine
git clone https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
cd skh-flamelang-StrategicKhaos-prefix-/flamelang-training

# Run the pipeline
docker compose up --build

# Distribute results
rsync -av modules/*/vectors/ athena:/shared/flamelang/vectors/
```

---

## 🌐 Environment 3: Claude Built It (You Pull)

```bash
# Pull this repository
git clone <repo-url>
cd skh-flamelang-StrategicKhaos-prefix-/flamelang-training

# Everything is ready - just run
docker compose up --build
```

---

## 📊 What You Get

### Module 1: Descriptive Statistics
**Content:** Mean, median, mode, variance, standard deviation  
**Output:** `modules/mod1-descriptive-stats/vectors/embeddings.npy`  
**Size:** ~10-15 concept embeddings (384 dimensions each)

### Module 2: Bar Charts (1.5 from Zybooks)
**Content:** 
- Bar chart visualization concepts
- Python/SeaBorn → FlameLang transformations
- Complete 5-layer mappings
- Glyph-annotated syntax examples

**Output:** `modules/mod2-bar-charts/vectors/mod2-bar-charts.npy`  
**Size:** ~50+ embeddings including:
- Visual concepts (bar_chart, categorical_variable, frequency)
- Code transformations (Python ↔ FlameLang)
- Layer mappings (English, Hebrew, Unicode, Wave, DNA)
- Visualization principles

---

## 🎓 The Bar Charts Module Delivers Everything You Asked For

### ✅ corpus.yml with Bar Chart Concepts

All concepts mapped to FlameLang's 5 layers:

```yaml
layers:
  english:    # "Visual representation using rectangular bars"
  hebrew:     # Glyphs: 📊 █ ▓ │ ─
  unicode:    # Operators: → ⊕ ∀ ≡ ⊢
  wave:       # Dimensions: [length], [pixels/value_unit]
  dna:        # Functions: create_bar_chart, render_bar
```

### ✅ Dockerfile for Module

Container with all dependencies:
- Python 3.11 + numpy, pandas
- matplotlib + seaborn (for visualization context)
- sentence-transformers (for embeddings)

### ✅ Python/SeaBorn Code → Glyph-Annotated FlameLang

Three complete examples:

1. **Basic Bar Chart**
   ```python
   # Python/SeaBorn
   sns.barplot(x=['A','B','C'], y=[23,45,12])
   ```
   ↓
   ```flamelang
   # FlameLang with glyphs
   ⟦data⟧ ≔ {'A'→23, 'B'→45, 'C'→12}
   ⟦chart⟧ ≔ 📊.create_bar_chart(⟦data⟧)
   ⊢ render(⟦chart⟧)
   ```

2. **Frequency Bar Chart**
3. **Grouped Bar Chart**

### ✅ Vector Embedding Schema

```yaml
embedding:
  model: "sentence-transformers/all-MiniLM-L6-v2"
  dimensions: 384
  normalization: "l2"
  storage:
    format: "numpy"
    path: "/app/vectors/"
```

---

## 🔍 Verify It Worked

```bash
# Check if vectors were generated
ls -lh modules/mod2-bar-charts/vectors/

# Should see:
# mod2-bar-charts.npy (~78KB for 50 embeddings)
# mod2-bar-charts-meta.json

# Inspect metadata
cat modules/mod2-bar-charts/vectors/mod2-bar-charts-meta.json

# Load vectors in Python
python3 -c "
import numpy as np
import json

vectors = np.load('modules/mod2-bar-charts/vectors/mod2-bar-charts.npy')
meta = json.load(open('modules/mod2-bar-charts/vectors/mod2-bar-charts-meta.json'))

print(f'Shape: {vectors.shape}')
print(f'Keys: {len(meta[\"keys\"])} embeddings')
print(f'Sample keys: {meta[\"keys\"][:5]}')
"
```

---

## 📖 Documentation

- **[README.md](README.md)** - Complete documentation
- **[INTEGRATION.md](INTEGRATION.md)** - Detailed integration guide
- **[corpus.yml](modules/mod2-bar-charts/corpus.yml)** - Full example

---

## 🔧 Customize

### Run Just One Module

```bash
docker compose up --build mod2-bar-charts
```

### Change Embedding Model

Edit `corpus.yml`:
```yaml
embedding:
  model: "sentence-transformers/all-mpnet-base-v2"
```

### Add Module 3

```bash
cp -r modules/mod2-bar-charts modules/mod3-your-topic
# Edit corpus.yml, train.py
# Add to docker-compose.yml
```

---

## 🎉 Summary

**You asked for:**
- ✅ Modular Docker structure (`/flamelang-training/modules/`)
- ✅ corpus.yml with bar chart concepts
- ✅ 5-layer transformation mapping
- ✅ Python/SeaBorn → FlameLang glyphs
- ✅ Vector embedding schema
- ✅ Works in any environment (Codespace/Local/Claude)

**You got all of it!**

Run `docker compose up --build` and watch the vectors generate. 🚀
