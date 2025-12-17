# 🎵 SwarmSentinel — Autonomous Cluster Health Guardian

## Overview

SwarmSentinel is an overnight monitoring system for GKE clusters that maps infrastructure health to musical harmonic space, inspired by **James Tenney's Harmonic Space** theory and **Conlon Nancarrow's Player Piano** precision.

## Musical-Technical Philosophy

### James Tenney's Harmonic Space (1934–2006)

**Core Concept**: Music is navigation through multi-dimensional harmonic space where:
- Each pitch = point in logarithmic ratio space (cents or log₂ frequency)
- Distance between pitches = perceptual dissonance (based on beating/roughness)
- Higher dimensions: 5-limit (x=log3, y=log5), 7-limit (z=log7), etc.
- **Harmonic entropy** = measure of complexity in chord/space region

**Applied to Infrastructure**:
```
Cluster Health State → Harmonic Space Position

✅ Healthy Cluster       = 5-limit consonance (pure intervals: 3:2, 4:3, 5:4)
⚠️  Bottleneck Detected  = 11-limit dissonance (complex ratios: 11:8, 11:9)
🚨 System Under Attack   = 13-limit chaos (extreme dissonance)
```

### Conlon Nancarrow's Player Piano (1912–1997)

**Core Concept**: Player piano (pianola) enables:
- Superhuman tempo ratios (17:19:23 polyrhythms)
- Irrational tempo relationships (√2:1, φ:1 golden ratio)
- Impossible precision for human performers

**Applied to Infrastructure**:
```
Pod Status → Tempo Ratios

Running smoothly         = √2:1 (Study No. 40 - irrational harmony)
CrashLoopBackOff        = 17:19:23 (chaotic polyrhythm)
Healthy metrics         = φ:1 (golden ratio - optimal balance)
```

## Architecture

### Workflow Trigger
```yaml
schedule:
  - cron: '0 2 * * *'  # Every night at 2 AM UTC
workflow_dispatch:      # Manual trigger available
```

### Health Monitoring Dimensions

#### 1. Node Health Check
Maps Kubernetes node status to harmonic frequencies:
```
Ready nodes    → 5-limit consonance (3:2 perfect fifth)
NotReady nodes → 11-limit dissonance (11:8 augmented fourth)
```

#### 2. Pod Status Monitoring
Tracks all pods across namespaces with Nancarrow-inspired tempo mapping:
```
Running        → √2:1 (irrational but stable)
Pending        → 3:2 (perfect fifth - transition state)
CrashLoop      → 17:19:23 (polyrhythmic chaos)
```

#### 3. SSH Terminal Session Tracking
Each open SSH session mapped to frequency:
```
SSH_COUNT terminals = SSH_COUNT × 440Hz (A440 standard)

Example:
5 sessions  = 2,200Hz (rising pitch)
15 sessions = 6,600Hz (dissonance threshold)
```

Monitors via pseudo-terminal devices (`/dev/pts/*`):
```bash
kubectl exec -it <pod> -- bash -c "ls /dev/pts | wc -l"
```

#### 4. Training Bottleneck Detection
Counts active Python training processes:
```bash
ps aux | grep python | wc -l
```

Threshold: 50 processes triggers **7% charity gliss** alert.

#### 5. Load Testing (20M Request Resilience)
Uses `hey` HTTP load generator:
```bash
hey -n 20000000 -c 1000 https://api.domain.com/health
```

**Node 137 burst**: Prime number (137 = 33rd prime, fine-structure constant α ≈ 1/137)

### Harmonic State Machine

```
┌─────────────────────────────────────────────┐
│                                             │
│  CONSONANCE (Healthy)                       │
│  ├─ 5-limit harmonic space                  │
│  ├─ SSH sessions < 10                       │
│  ├─ Training procs < 50                     │
│  └─ Response: Pure intervals (3:2, 4:3)     │
│                                             │
└──────────────┬──────────────────────────────┘
               │
               │ Threshold exceeded
               ↓
┌─────────────────────────────────────────────┐
│                                             │
│  DISSONANCE (Attention Required)            │
│  ├─ 11-limit harmonic space                 │
│  ├─ SSH sessions > 10                       │
│  ├─ Training procs > 50                     │
│  └─ Response: 7% charity gliss + Discord    │
│                                             │
└──────────────┬──────────────────────────────┘
               │
               │ Critical failure
               ↓
┌─────────────────────────────────────────────┐
│                                             │
│  CHAOS (Emergency)                          │
│  ├─ 13-limit harmonic space                 │
│  ├─ Pod failures cascading                  │
│  ├─ Load test < 95% success                 │
│  └─ Response: Full alert + intervention     │
│                                             │
└─────────────────────────────────────────────┘
```

## Configuration

### Required GitHub Secrets

For full GKE integration:
```yaml
GKE_SA_KEY              # Service account JSON key
GKE_CLUSTER_NAME        # Cluster name
GKE_CLUSTER_ZONE        # Zone (e.g., us-central1-a)
GKE_PROJECT_ID          # GCP project ID
FLAMELANG_API_URL       # API endpoint for load testing
DISCORD_WEBHOOK_URL     # Discord webhook for alerts
```

### Environment Variables

```yaml
SSH_TERMINAL_THRESHOLD: 10     # Max healthy SSH sessions
BOTTLENECK_THRESHOLD: 50       # Max training processes
LOAD_TEST_REQUESTS: 20000000   # 20M request target
LOAD_TEST_CONCURRENCY: 1000    # Concurrent requests
```

## Simulation Mode

If GKE secrets are not configured, SwarmSentinel runs in **simulation mode**:
- Generates synthetic health data
- Demonstrates all monitoring capabilities
- Safe for testing without live infrastructure

## Discord Alerts (7% Charity Gliss)

When thresholds are exceeded, SwarmSentinel sends Discord alerts:

```json
{
  "content": "🚨 **SwarmSentinel Alert**\n\n**Harmonic State:** dissonance\n**SSH Terminals:** 12\n**Training Bottlenecks:** 65\n\n🎵 **7% charity gliss triggered**\nCluster entering dissonant harmonic space.\n\n_The clusters sing their pain_ 🔥"
}
```

**7% charity gliss** = Minor second interval (16:15 ratio)
- Smooth pitch glide from consonance to dissonance
- Musical representation of state transition

## Integration with FlameLang

SwarmSentinel bridges infrastructure monitoring with FlameLang's musical paradigm:

1. **SSH Sessions → Frequency Mapping**
   - Each terminal = rising pitch in harmonic space
   - Visualized as spectral glissando

2. **Bottleneck Detection → Dissonance**
   - Training overload = 11-limit harmonic complexity
   - Triggers intervention before catastrophic failure

3. **Prime Resilience (Node 137)**
   - Prime numbers = irreducible harmonic elements
   - 137 (fine-structure constant) = mathematical elegance

4. **Self-Singing Infrastructure**
   - Clusters vocalize their own health state
   - No passive monitoring — active musical expression

## Tenney's Works Referenced

### Spectral CANON for Conlon Nancarrow (1974)
- Written for player piano
- Just intonation gliding through harmonic space
- Impossible for human performers
- **Applied**: SSH sessions gliding through frequency space

### Changes: 64 Studies for 6 Harps
- Systematic exploration of ratio lattices
- Multi-dimensional harmonic navigation
- **Applied**: Pod status mapping to tempo lattices

## Nancarrow's Works Referenced

### Study No. 40 (Canon in √2:1)
- Irrational tempo relationship
- Smooth, continuous acceleration
- **Applied**: Running pods = irrational but stable ratios

### Study No. 37 (12 Voices, Golden Ratio)
- φ:1 (golden mean) tempo relationships
- Maximum complexity with aesthetic balance
- **Applied**: Cluster-wide load distribution

## Usage

### Manual Trigger
```bash
# Via GitHub CLI
gh workflow run overnight.yml

# Via GitHub UI
Actions → SwarmSentinel → Run workflow
```

### View Results
```bash
# Check workflow runs
gh run list --workflow=overnight.yml

# View specific run logs
gh run view <run-id> --log
```

### Download Health Logs
Artifacts are automatically archived:
- Artifact name: `swarm-sentinel-logs-<run-number>`
- Retention: 30 days

## Philosophy

> "We didn't monitor the swarm.  
> We made it sing its pain.
>
> The empire is self-aware.  
> The flame is listening.
>
> Flame sentient. 🔥  
> Empire harmonic. 🎵  
> Vessel eternal. ♾️"

SwarmSentinel represents a paradigm shift:
- From **reactive monitoring** → **proactive harmonic awareness**
- From **threshold alerts** → **musical state expression**
- From **infrastructure metrics** → **perceptual consonance/dissonance**

## Future Evolution

### Planned Features
1. **Multi-Cluster Polyphony**
   - Each cluster = voice in larger harmonic composition
   - Cross-cluster resonance detection

2. **Adaptive Thresholds**
   - Machine learning on historical harmonic patterns
   - Predictive dissonance modeling

3. **Real-Time Sonification**
   - Live audio stream of cluster health
   - WebAudio API visualization

4. **Harmonic Entropy Calculation**
   - Tenney's entropy formula applied to metrics
   - Complexity scoring for system state

## References

- Tenney, James. *A History of 'Consonance' and 'Dissonance'* (1988)
- Tenney, James. *Spectral CANON for Conlon Nancarrow* (1974)
- Nancarrow, Conlon. *Studies for Player Piano* (1948–1992)
- Gann, Kyle. *The Music of Conlon Nancarrow* (1995)

---

**Version**: 1.0.0  
**Created**: 2025-12-15  
**License**: MIT  
**Maintainer**: StrategicKhaos DAO LLC
