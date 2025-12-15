# FlameVault: Honeypot + Encrypted Secrets Engine

**INV-080: Strategickhaos DAO LLC**

FlameVault is a dual-layer security system that combines honeypot monitoring with machine-bound encrypted secret storage.

## Overview

FlameVault provides two complementary security layers:

1. **Honeypot Layer**: Visible fake keys that trigger alerts when accessed
2. **Encrypted Layer**: Machine-bound AES-256-GCM encrypted storage for real secrets

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     FlameVault System                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌───────────────────────┐    ┌────────────────────────┐   │
│  │   Honeypot Layer      │    │   Encrypted Storage    │   │
│  │   (Monitored Bait)    │    │   (Real Secrets)       │   │
│  ├───────────────────────┤    ├────────────────────────┤   │
│  │ - OPENAI_API_KEY      │    │ AES-256-GCM Encrypted  │   │
│  │ - XAI_API_KEY         │    │ Machine-bound key      │   │
│  │ - EMAIL_PASS          │    │ ~/.flamevault/*.enc    │   │
│  │                       │    │                        │   │
│  │ Logs all access       │    │ Requires:              │   │
│  │ attempts to           │    │ - Hostname             │   │
│  │ alerts.log            │    │ - STRAT_HEMISPHERE     │   │
│  └───────────────────────┘    │ - STRAT_DEVICE_ID      │   │
│                                └────────────────────────┘   │
│                                                               │
│  Smart Router: Checks honeypot first, then encrypted store  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Installation

### Rust Implementation

```bash
# Build FlameVault CLI
cargo build --release --bin flamevault

# Install globally
cargo install --path . --bin flamevault
```

### PowerShell Implementation

```powershell
# Load the FlameVault PowerShell module
. .\flamevault-honeypot.ps1
```

## Usage

### Rust CLI

#### Initialize FlameVault

```bash
flamevault init
```

This creates the `~/.flamevault/` directory with machine-bound encryption.

#### Store Real Secrets

```bash
# Store a real API key (encrypted)
flamevault set REAL_OPENAI_API_KEY "sk-your-actual-key"
flamevault set REAL_XAI_API_KEY "xai-your-actual-key"
flamevault set REAL_GH_TOKEN "ghp_your-real-token"
```

#### Deploy Honeypot

```bash
# Deploy fake keys as bait
flamevault deploy-honeypot
```

This sets environment variables with fake keys that will trigger alerts if accessed.

#### Retrieve Secrets

```bash
# Direct retrieval (no honeypot check)
flamevault get REAL_OPENAI_API_KEY

# Smart retrieval (checks honeypot first)
flamevault get-real OPENAI_API_KEY
```

#### List Secrets

```bash
flamevault list
```

#### Check Honeypot Alerts

```bash
# Show last 10 alerts
flamevault alerts

# Show last 5 alerts
flamevault alerts 5
```

#### Remove Secrets

```bash
flamevault remove REAL_OLD_API_KEY
```

#### Interactive Mode

```bash
flamevault interactive
# or
flamevault -i
```

### PowerShell

#### Load Module

```powershell
. .\flamevault-honeypot.ps1
```

#### Store Real Secrets

```powershell
Set-FlameSecret -Name "REAL_OPENAI_API_KEY" -Value "sk-your-actual-key"
Set-FlameSecret -Name "REAL_XAI_API_KEY" -Value "xai-your-actual-key"
```

#### Deploy Honeypot

```powershell
Set-HoneypotTrap
```

#### Retrieve Secrets

```powershell
# Direct retrieval
$key = Get-FlameSecret -Name "REAL_OPENAI_API_KEY"

# Smart retrieval (checks honeypot first)
$key = Get-RealSecret -Name "OPENAI_API_KEY"
```

#### View Alerts

```powershell
Get-HoneypotAlerts -Last 10
```

#### List Secrets

```powershell
Get-FlameSecretList
```

#### Remove Secrets

```powershell
Remove-FlameSecret -Name "REAL_OLD_KEY"
```

## Security Model

### Honeypot Keys

The following keys are monitored as honeypots:

- `OPENAI_API_KEY`
- `XAI_API_KEY`
- `EMAIL_PASS`

Any access to these keys is logged with:
- Timestamp
- Key name
- Process name
- Username
- Computer/hostname
- Current path (PowerShell only)

### Machine-Bound Encryption

Real secrets are encrypted using AES-256-GCM with a machine-bound key derived from:

1. **Hostname/Computer name** - Ties the key to the physical machine
2. **STRAT_HEMISPHERE** - Environment variable for geographical isolation
3. **STRAT_DEVICE_ID** - Device-specific identifier
4. **Constant salt** - "STRATEGICKHAOS"

The key is derived using SHA-256:
```
Key = SHA256(hostname|hemisphere|device_id|STRATEGICKHAOS)
```

This ensures secrets cannot be decrypted if:
- Copied to a different machine
- The environment variables change
- The machine is renamed

### Storage Location

- **Rust**: `~/.flamevault/`
- **PowerShell**: `$env:USERPROFILE\.flamevault\`

Files:
- `*.enc` - Encrypted secrets (base64-encoded AES-256-GCM ciphertext)
- `alerts.log` - Honeypot access attempts (JSON lines)
- `honeypot.log` - Honeypot deployment history (JSON lines)

## Integration Examples

### In Application Code (Rust)

```rust
use flamelang::flamevault::FlameVault;

fn main() {
    let vault = FlameVault::new().expect("Failed to initialize FlameVault");
    
    // Get a real secret (with honeypot check)
    let api_key = vault.get_real_secret("OPENAI_API_KEY")
        .expect("Failed to get API key");
    
    // Use the API key...
}
```

### In Scripts (Bash)

```bash
#!/bin/bash

# Get API key
API_KEY=$(flamevault get-real OPENAI_API_KEY)

# Use it
curl -H "Authorization: Bearer $API_KEY" https://api.openai.com/v1/models
```

### In Scripts (PowerShell)

```powershell
# Load FlameVault
. .\flamevault-honeypot.ps1

# Get API key
$apiKey = Get-RealSecret -Name "OPENAI_API_KEY"

# Use it
$headers = @{
    "Authorization" = "Bearer $apiKey"
}
Invoke-RestMethod -Uri "https://api.openai.com/v1/models" -Headers $headers
```

## Monitoring

### Check for Intrusions

Regularly check honeypot alerts:

```bash
# Rust
flamevault alerts 20

# PowerShell
Get-HoneypotAlerts -Last 20
```

### Alert Format

```json
{
  "timestamp": "2025-12-15T05:00:00Z",
  "alert": "HONEYPOT_ACCESS_ATTEMPT",
  "key": "OPENAI_API_KEY",
  "process": "python3",
  "user": "username",
  "computer": "hostname"
}
```

### Automated Monitoring

You can monitor the alerts.log file in real-time:

```bash
# Linux/Mac
tail -f ~/.flamevault/alerts.log

# PowerShell
Get-Content "$env:USERPROFILE\.flamevault\alerts.log" -Wait
```

## Best Practices

1. **Always use the REAL_ prefix** for actual secrets:
   - ✅ `REAL_OPENAI_API_KEY`
   - ❌ `OPENAI_API_KEY` (this is a honeypot!)

2. **Deploy honeypot early** to maximize monitoring coverage

3. **Regularly check alerts** to detect unauthorized access attempts

4. **Set environment variables** for machine binding:
   ```bash
   export STRAT_HEMISPHERE="NORTH"
   export STRAT_DEVICE_ID="node-001"
   ```

5. **Rotate real keys** if honeypot is triggered:
   ```bash
   # If alert detected, rotate the real key
   flamevault remove REAL_OPENAI_API_KEY
   flamevault set REAL_OPENAI_API_KEY "sk-new-rotated-key"
   ```

6. **Backup encrypted secrets** (they're useless without the machine key):
   ```bash
   tar -czf flamevault-backup.tar.gz ~/.flamevault/*.enc
   ```

## Security Considerations

### Strengths

- ✅ Machine-bound encryption prevents secret theft via file copy
- ✅ Honeypot layer detects unauthorized access attempts
- ✅ AES-256-GCM provides authenticated encryption
- ✅ Nonces prevent replay attacks
- ✅ Separation of honeypot and real secrets

### Limitations

- ⚠️ Secrets are decrypted in memory when accessed (use secure memory practices)
- ⚠️ Machine-bound key can be recovered with physical access and environment variables
- ⚠️ Honeypot only detects access, doesn't prevent it
- ⚠️ Root/admin users can access all files
- ⚠️ Process memory dumps could expose decrypted secrets

### Mitigations

1. **Use secure memory** practices when handling decrypted secrets
2. **Limit access** to the `.flamevault` directory (chmod 700)
3. **Monitor system logs** for unauthorized access to the vault directory
4. **Use hardware security modules (HSM)** for additional key protection
5. **Implement rate limiting** on secret access
6. **Set up alerting** on honeypot triggers

## Troubleshooting

### Decryption fails after machine change

**Cause**: Machine-bound key has changed

**Solution**: Restore from backup or re-encrypt secrets on the new machine

### Honeypot not triggering

**Cause**: Application using direct file access instead of FlameVault API

**Solution**: Ensure applications use FlameVault functions to access secrets

### Cannot read secrets on different machine

**Expected behavior**: Secrets are machine-bound by design

**Solution**: Transfer encrypted files and re-encrypt with the new machine's key (requires the original secrets)

## License

© 2025 Strategickhaos DAO LLC

Part of the FlameLang sovereign compiler toolchain.

## Support

For issues and questions:
- GitHub Issues: https://github.com/Strategickhaos-Swarm-Intelligence/skh-flamelang
- Security: security@strategickhaos.ai
