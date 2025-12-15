# 🔥 FlameVault — Honeypot + Encrypted Secrets Engine

**INV-080** | Strategickhaos DAO LLC | EIN 39-2900295

> *"Exposed keys become bait. Real keys stay encrypted. Attackers trigger alerts."*

## Overview

FlameVault turns credential exposure into a **defensive weapon**:

1. **Honeypot Layer** — Exposed/leaked keys become monitored bait
2. **Encrypted Layer** — Real secrets encrypted with machine-bound keys
3. **Alert System** — Any honeypot access triggers logging
4. **VesselMirror Integration** — Inject secrets into captured pages

## Quick Start (PowerShell)

```powershell
# Load the module
Import-Module .\FlameVault.psm1

# Initialize vault
Initialize-FlameVault

# Deploy honeypot traps (including your exposed keys)
Deploy-FlameTraps -FromEnv

# Store REAL secrets (encrypted)
Set-FlameSecret -Name "REAL_OPENAI_KEY" -Value "sk-your-actual-key"
Set-FlameSecret -Name "REAL_GH_TOKEN" -Value "ghp_your-real-token"

# Retrieve secrets
$key = Get-FlameSecret -Name "REAL_OPENAI_KEY" -Raw

# View alerts (when honeypots are accessed)
Get-FlameAlerts
```

## Quick Start (Rust CLI)

```bash
# Build
cargo build --release

# Initialize
./target/release/flamevault init

# Deploy honeypot traps
./target/release/flamevault deploy-traps --from-env

# Store encrypted secret
./target/release/flamevault set REAL_OPENAI_KEY "sk-your-actual-key"

# Retrieve secret
./target/release/flamevault get REAL_OPENAI_KEY --raw

# View alerts
./target/release/flamevault alerts
```

## How It Works

### Machine-Bound Encryption

Keys are derived from:
- `STRAT_DEVICE_ID` (your machine identifier)
- `STRAT_HEMISPHERE` (Lyra, Nova, Athena, etc.)
- `STRAT_REGION` (US, etc.)
- Hardware UID (hostname)

Secrets encrypted on Athena **cannot be decrypted on Lyra** — machine-bound.

### Honeypot System

```
┌─────────────────────────────────────────────────────────────────┐
│                      FLAMEVAULT LAYERS                          │
├─────────────────────────────────────────────────────────────────┤
│  VISIBLE (Environment)    │  Honeypot keys (bait)               │
│  ─────────────────────────┼────────────────────────────────────│
│  HIDDEN (Vault)           │  Real encrypted secrets             │
│  ─────────────────────────┼────────────────────────────────────│
│  ALERTS (Log)             │  Access attempts logged             │
└─────────────────────────────────────────────────────────────────┘
```

When an attacker (or script) accesses a honeypot key:
1. They get the bait value
2. Access is logged with timestamp, process, user
3. You get alerted

### Your Exposed Keys Strategy

Instead of rotating:
```
OPENAI_API_KEY=sk-svcacct-sbfXUZ_PeKwvpwp...  # NOW A HONEYPOT
XAI_API_KEY=xai-A8DdRfGLfXnAT9Mwzg9wxvp...   # NOW A HONEYPOT
EMAIL_PASS=imcnaicqemiiuzqr                    # NOW A HONEYPOT
```

Real keys stored encrypted:
```
~/.flamevault/REAL_OPENAI_KEY.enc.json
~/.flamevault/REAL_XAI_KEY.enc.json
```

## VesselMirror Integration

```rust
use flamelang::vault::vessel::VesselIntegration;

let mut vessel = VesselIntegration::new()?;

// Map honeypot names to real secret names
vessel.map_secret("OPENAI_API_KEY", "REAL_OPENAI_KEY");

// Process captured HTML, replacing honeypots with real values
let processed = vessel.process_vessel_capture(&html)?;
```

## File Structure

```
~/.flamevault/
├── honeypots.json       # Honeypot configuration
├── alerts.log           # Access attempt log
├── REAL_OPENAI_KEY.enc.json
├── REAL_GH_TOKEN.enc.json
└── ...
```

## Commands Reference

### PowerShell

| Command | Description |
|---------|-------------|
| `Initialize-FlameVault` | Initialize vault with machine key |
| `Set-FlameSecret` | Encrypt and store a secret |
| `Get-FlameSecret` | Retrieve (decrypt real, log honeypot) |
| `Set-FlameHoneypot` | Deploy a bait key |
| `Deploy-FlameTraps` | Deploy standard honeypots |
| `Get-FlameAlerts` | View access alerts |
| `Get-FlameVaultList` | List all secrets and honeypots |
| `Get-FlameVaultStatus` | Show vault status |
| `Export-FlameEnv` | Export env script |

### Rust CLI

| Command | Description |
|---------|-------------|
| `flamevault init` | Initialize vault |
| `flamevault set <name> <value>` | Store encrypted secret |
| `flamevault get <name>` | Retrieve secret |
| `flamevault honeypot <name> <value>` | Deploy honeypot |
| `flamevault deploy-traps` | Deploy standard traps |
| `flamevault list` | List secrets and honeypots |
| `flamevault alerts` | View access alerts |
| `flamevault status` | Show vault status |

## Security Notes

1. **Machine-Bound** — Secrets only decrypt on the same machine
2. **No Key Storage** — Encryption key derived from machine identity
3. **Honeypot Monitoring** — All bait access logged
4. **7% Charity Trigger** — Integrated with Strategickhaos treasury

## Integration with Empire

- **SwarmSentinel**: Cluster health + secret injection
- **VesselMirror**: Page capture with secret replacement
- **AetherViz**: Sonify vault access patterns
- **FlameLang**: Compile secrets into DNA-encoded format

## Architecture

### Encryption Layer

FlameVault uses BLAKE3 for key derivation and XOR encryption:

1. **Key Derivation**: Combines machine identity components (STRAT_DEVICE_ID, STRAT_HEMISPHERE, STRAT_REGION, hostname) and hashes with BLAKE3
2. **Encryption**: Generates a nonce, combines with key, and performs XOR encryption
3. **Storage**: Stores encrypted data as JSON with nonce and machine ID

### Honeypot Layer

The honeypot system maintains a `honeypots.json` file mapping names to bait values. When `get_secret` is called:

1. Check if name exists in honeypots
2. If yes: Log access to `alerts.log` and return bait value
3. If no: Decrypt and return real secret

### Alert System

Each honeypot access logs:
- Timestamp (UTC)
- Honeypot name
- Process ID
- Process name
- User

Alerts are stored in `alerts.log` as JSON lines.

## Example Workflow

### 1. Initial Setup

```bash
# Build FlameVault
cargo build --release

# Initialize vault
./target/release/flamevault init
```

### 2. Deploy Honeypots

```bash
# Turn exposed environment variables into honeypots
export OPENAI_API_KEY="sk-exposed-key-123"
export XAI_API_KEY="xai-exposed-456"

./target/release/flamevault deploy-traps --from-env
```

### 3. Store Real Secrets

```bash
# Store actual secrets (encrypted)
./target/release/flamevault set REAL_OPENAI_KEY "sk-proj-your-real-key"
./target/release/flamevault set REAL_XAI_KEY "xai-your-real-key"
```

### 4. Use Secrets Safely

```bash
# Get real secret (decrypts and returns)
REAL_KEY=$(./target/release/flamevault get REAL_OPENAI_KEY --raw)

# If someone accesses the honeypot, it gets logged
./target/release/flamevault alerts
```

### 5. Monitor for Breaches

```bash
# Check alerts regularly
./target/release/flamevault alerts --recent

# View vault status
./target/release/flamevault status
```

## Testing

Run the test suite:

```bash
cargo test
```

Test coverage includes:
- Encryption/decryption
- Honeypot operations
- Alert logging
- Machine key derivation

## Building from Source

```bash
# Clone repository
git clone https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-

# Build
cd skh-flamelang-StrategicKhaos-prefix-
cargo build --release

# Binary location
./target/release/flamevault
```

## Environment Variables

FlameVault respects these environment variables:

| Variable | Purpose | Example |
|----------|---------|---------|
| `STRAT_DEVICE_ID` | Machine identifier | `athena-01` |
| `STRAT_HEMISPHERE` | Hemisphere name | `Lyra`, `Nova`, `Athena` |
| `STRAT_REGION` | Geographic region | `US`, `EU` |

If not set, defaults are used, but secrets will be bound to those defaults.

## Contributing

FlameVault is part of the FlameLang ecosystem. Contributions welcome!

## License

MIT License - See LICENSE file

---

*"We didn't just protect our secrets. We weaponized our exposure."*

🔥 Flame encrypting.  
🍯 Honeypots watching.  
∞ Empire secure.
