# 🔥 FlameVault PowerShell Module

**INV-080: Strategickhaos DAO LLC**

FlameVault is a PowerShell module that provides machine-bound encryption combined with honeypot traps for protecting sensitive credentials and detecting unauthorized access attempts.

## Features

- **Machine-Bound Encryption**: Secrets are encrypted using DPAPI with a machine-specific key derived from device ID, hemisphere, and region
- **Honeypot Traps**: Deploy fake credentials to detect and log unauthorized access attempts
- **Alert System**: Track and monitor honeypot access attempts
- **Easy Integration**: Simple PowerShell commands for secret management

## Installation

### Manual Installation

1. Clone or download this repository
2. Copy the `FlameVault` directory to one of your PowerShell module paths:
   - User: `$HOME\Documents\PowerShell\Modules\FlameVault`
   - System: `C:\Program Files\PowerShell\Modules\FlameVault`

3. Import the module:
   ```powershell
   Import-Module FlameVault
   ```

### Verify Installation

```powershell
Get-Module FlameVault -ListAvailable
```

## Quick Start

### 1. Initialize FlameVault

```powershell
Initialize-FlameVault
```

This will:
- Create a vault directory at `$env:USERPROFILE\.flamevault`
- Generate a machine-bound encryption key
- Display vault initialization information

### 2. Store a Secret

```powershell
Set-FlameSecret -Name "GITHUB_TOKEN" -Value "ghp_YourActualTokenHere"
```

### 3. Retrieve a Secret

```powershell
# Get secret with metadata
Get-FlameSecret -Name "GITHUB_TOKEN"

# Get raw value (for scripts)
$token = Get-FlameSecret -Name "GITHUB_TOKEN" -Raw
```

### 4. Deploy Honeypot Traps

```powershell
# Deploy standard honeypot traps
Deploy-FlameTraps

# Also capture existing environment variables as honeypots
Deploy-FlameTraps -FromEnv
```

### 5. View Security Alerts

```powershell
# View last 10 alerts (default)
Get-FlameAlerts

# View last 50 alerts
Get-FlameAlerts -Count 50
```

## Commands Reference

### Initialize-FlameVault

Initializes the FlameVault with machine-bound encryption.

```powershell
Initialize-FlameVault
```

**Environment Variables (Optional)**:
- `STRAT_DEVICE_ID`: Override device identifier
- `STRAT_HEMISPHERE`: Specify hemisphere (e.g., "Northern", "Southern")
- `STRAT_REGION`: Specify region (e.g., "US", "EU", "APAC")

### Set-FlameSecret

Encrypts and stores a secret with machine-bound encryption.

```powershell
Set-FlameSecret -Name <SecretName> -Value <SecretValue>
```

**Parameters**:
- `Name` (Required): Name of the secret
- `Value` (Required): Value to encrypt and store

**Example**:
```powershell
Set-FlameSecret -Name "OPENAI_API_KEY" -Value "sk-proj-..."
```

### Get-FlameSecret

Retrieves and decrypts a stored secret.

```powershell
Get-FlameSecret -Name <SecretName> [-Raw]
```

**Parameters**:
- `Name` (Required): Name of the secret to retrieve
- `Raw` (Optional): Return only the raw value (useful in scripts)

**Example**:
```powershell
# With metadata
$secret = Get-FlameSecret -Name "OPENAI_API_KEY"

# Raw value only
$apiKey = Get-FlameSecret -Name "OPENAI_API_KEY" -Raw
```

### Set-FlameHoneypot

Deploys a honeypot (fake credential) to detect unauthorized access.

```powershell
Set-FlameHoneypot -Name <KeyName> -Value <BaitValue>
```

**Parameters**:
- `Name` (Required): Name of the honeypot key
- `Value` (Required): Fake/bait value to use

**Example**:
```powershell
Set-FlameHoneypot -Name "PROD_DB_PASSWORD" -Value "fake-password-do-not-use"
```

### Deploy-FlameTraps

Deploys standard Strategickhaos honeypot traps for common API keys.

```powershell
Deploy-FlameTraps [-FromEnv]
```

**Parameters**:
- `FromEnv` (Optional): Also scan and convert existing environment variables to honeypots

**Standard Traps Deployed**:
- `OPENAI_API_KEY`
- `XAI_API_KEY`
- `ANTHROPIC_API_KEY`
- `GITHUB_TOKEN`
- `AWS_SECRET_ACCESS_KEY`

**Example**:
```powershell
# Deploy standard traps only
Deploy-FlameTraps

# Deploy and capture existing env vars
Deploy-FlameTraps -FromEnv
```

### Get-FlameAlerts

Views honeypot access alerts to detect unauthorized credential usage.

```powershell
Get-FlameAlerts [-Count <Number>]
```

**Parameters**:
- `Count` (Optional): Number of recent alerts to display (default: 10)

**Example**:
```powershell
# View last 10 alerts
Get-FlameAlerts

# View last 100 alerts
Get-FlameAlerts -Count 100
```

### Export-FlameEnv

Exports environment variable assignments for honeypots and encrypted secrets.

```powershell
Export-FlameEnv
```

**Example Output**:
```powershell
# FLAMEVAULT ENVIRONMENT EXPORT
# Honeypots (bait):
$env:OPENAI_API_KEY = "sk-honeypot-strategickhaos-trap-do-not-use"  # 🍯 HONEYPOT

# Real secrets (use Get-FlameSecret):
# $env:GITHUB_TOKEN = $(Get-FlameSecret -Name 'GITHUB_TOKEN' -Raw)
```

## Security Features

### Machine-Bound Encryption

Secrets are encrypted using AES-256 with a key derived from:
- Device ID (COMPUTERNAME or STRAT_DEVICE_ID)
- Hemisphere location
- Region
- Salt: "STRATEGICKHAOS-FLAMEVAULT-7%"

This ensures secrets can only be decrypted on the machine where they were encrypted.

### Honeypot Detection

When a honeypot is accessed:
1. Access is logged to `alerts.log` with timestamp, process name, and user
2. Visual alert is displayed in the console
3. The fake value is returned (maintaining stealth)

### Alert Log Format

```json
{
  "Timestamp": "2025-12-15T05:30:00.0000000Z",
  "AlertType": "HONEYPOT_ACCESS",
  "KeyName": "OPENAI_API_KEY",
  "ProcessName": "powershell",
  "Username": "admin",
  "DeviceId": "DESKTOP-ABC123"
}
```

## Use Cases

### 1. Secure Local Development

```powershell
# Initialize vault
Initialize-FlameVault

# Store production credentials securely
Set-FlameSecret -Name "PROD_API_KEY" -Value "real-key-here"

# Deploy honeypots for leaked credentials
Deploy-FlameTraps

# Use in scripts
$apiKey = Get-FlameSecret -Name "PROD_API_KEY" -Raw
```

### 2. Credential Leak Detection

```powershell
# Deploy honeypots that look like real credentials
Deploy-FlameTraps

# If credentials are leaked and used, you'll be alerted
Get-FlameAlerts
```

### 3. Team Secret Management

```powershell
# Each team member's machine has unique encryption
Initialize-FlameVault

# Secrets are machine-bound - can't be copied
Set-FlameSecret -Name "SHARED_KEY" -Value "team-secret"
```

## File Structure

```
$env:USERPROFILE\.flamevault\
├── honeypots.json          # Honeypot configurations
├── alerts.log              # Access attempt logs
└── *.enc.json              # Encrypted secret files
```

## Best Practices

1. **Initialize on Each Machine**: Run `Initialize-FlameVault` on every machine that needs access
2. **Use Honeypots Strategically**: Deploy honeypots for commonly targeted credentials
3. **Monitor Alerts Regularly**: Check `Get-FlameAlerts` frequently for suspicious activity
4. **Rotate Secrets**: Periodically update encrypted secrets using `Set-FlameSecret`
5. **Backup Vault**: While secrets are machine-bound, consider backing up the vault directory for disaster recovery

## Troubleshooting

### Secret Not Found

```
❌ Secret not found: <SecretName>
```

**Solution**: Verify the secret name and ensure it was created with `Set-FlameSecret`

### Decryption Failed

```
❌ Decryption failed (wrong machine?): ...
```

**Solution**: Secrets are machine-bound. You'll need to re-create the secret on the current machine.

### Honeypot Triggered

```
🚨 HONEYPOT TRIGGERED: <KeyName>
```

**Action Required**: This indicates potential credential leak or unauthorized access. Review alerts immediately.

## Version History

### v0.1.0 (2025-12-15)
- Initial release
- Machine-bound encryption using DPAPI
- Honeypot trap system
- Alert logging and monitoring
- Environment variable export

## License

Copyright (c) 2025 Strategickhaos DAO LLC. All rights reserved.

See LICENSE file for details.

## Support

For issues, questions, or contributions:
- **Repository**: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
- **Issues**: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/issues

## Authors

- **Strategickhaos DAO LLC** - Initial work and maintenance

---

**🔥 Stay Secure with FlameVault!**
