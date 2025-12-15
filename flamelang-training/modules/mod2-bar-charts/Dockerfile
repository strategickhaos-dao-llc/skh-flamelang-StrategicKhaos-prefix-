# FlameLang Training Module: Bar Charts Visualization
# 5-layer transformation pipeline: English→Hebrew→Unicode→Wave→DNA→LLVM

FROM python:3.11-slim

LABEL maintainer="StrategicKhaos DAO LLC"
LABEL module="mod2-bar-charts"
LABEL flamelang.version="2.0.0"
LABEL flamelang.layer="5-layer-transform"

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Install Python dependencies for visualization
RUN pip install --no-cache-dir \
    numpy>=1.24.0 \
    pandas>=2.0.0 \
    matplotlib>=3.7.0 \
    seaborn>=0.12.0 \
    pyyaml>=6.0 \
    sentence-transformers>=2.2.0

# Copy corpus configuration
COPY corpus.yml /app/corpus.yml

# Training script
COPY train.py /app/train.py

# Create vectors output directory
RUN mkdir -p /app/vectors

# Environment variables
ENV MODULE_NAME=mod2-bar-charts
ENV FLAMELANG_LAYER=5-layer-transform
ENV PYTHONUNBUFFERED=1
ENV MPLBACKEND=Agg

# Volume for vector embeddings
VOLUME ["/app/vectors"]

# Entry point
CMD ["python", "/app/train.py"]
