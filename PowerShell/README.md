# PowerShell Modules

This directory contains PowerShell modules developed by Strategickhaos DAO LLC as part of the FlameLang project ecosystem.

## Available Modules

### 🔥 FlameVault

**Version**: 0.1.0  
**INV-080**: Honeypot + Encrypted Secrets Engine

A security-focused PowerShell module that combines machine-bound encryption with honeypot traps for protecting sensitive credentials and detecting unauthorized access attempts.

**Key Features**:
- Machine-bound DPAPI encryption
- Honeypot trap deployment
- Access alert monitoring
- Secure secret storage and retrieval

**Documentation**: [FlameVault/README.md](FlameVault/README.md)

**Quick Start**:
```powershell
# Import module
Import-Module .\FlameVault\FlameVault.psd1

# Initialize
Initialize-FlameVault

# Store a secret
Set-FlameSecret -Name "API_KEY" -Value "your-secret-here"

# Deploy honeypots
Deploy-FlameTraps
```

## Installation

### Option 1: Direct Usage

```powershell
# Navigate to the module directory
cd PowerShell/FlameVault

# Import the module
Import-Module .\FlameVault.psd1
```

### Option 2: Install to PowerShell Modules Path

**For Current User**:
```powershell
# Create modules directory if it doesn't exist
$modulePath = "$HOME\Documents\PowerShell\Modules\FlameVault"
New-Item -ItemType Directory -Path $modulePath -Force

# Copy module files
Copy-Item -Path ".\FlameVault\*" -Destination $modulePath -Recurse

# Import module
Import-Module FlameVault
```

**For All Users** (requires admin):
```powershell
# Create modules directory if it doesn't exist
$modulePath = "C:\Program Files\PowerShell\Modules\FlameVault"
New-Item -ItemType Directory -Path $modulePath -Force

# Copy module files
Copy-Item -Path ".\FlameVault\*" -Destination $modulePath -Recurse

# Import module
Import-Module FlameVault
```

## System Requirements

- **PowerShell**: 5.1 or higher (Windows PowerShell or PowerShell Core)
- **Operating System**: Windows 10/11, Windows Server 2016+
- **.NET Framework**: 4.5 or higher

## Usage Examples

Each module includes an `Examples.ps1` file demonstrating common usage patterns.

```powershell
# Run FlameVault examples
.\FlameVault\Examples.ps1
```

## Security Considerations

### FlameVault

- Secrets are encrypted with machine-bound keys and cannot be transferred between machines
- Honeypots generate alerts when accessed, stored in `$env:USERPROFILE\.flamevault\alerts.log`
- Vault data is stored locally at `$env:USERPROFILE\.flamevault`
- No secrets are transmitted over the network

## Development

### Module Structure

Each PowerShell module follows this standard structure:

```
ModuleName/
├── ModuleName.psm1    # Main module script
├── ModuleName.psd1    # Module manifest
├── README.md          # Module documentation
└── Examples.ps1       # Usage examples
```

### Contributing

1. Create a new branch for your module
2. Follow the standard module structure
3. Include comprehensive documentation
4. Add usage examples
5. Test on PowerShell 5.1 and PowerShell 7+

## License

All PowerShell modules are licensed under the same license as the parent FlameLang project. See [LICENSE](../LICENSE) for details.

Copyright (c) 2025 Strategickhaos DAO LLC. All rights reserved.

## Support

- **Repository**: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
- **Issues**: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/issues

## Version History

- **v0.1.0** (2025-12-15): Initial release with FlameVault module

---

**Part of the FlameLang v2.0.0 Ecosystem**
