# FlameVault PowerShell Module
# Honeypot + Encrypted Secrets Engine
# © 2025 Strategickhaos DAO LLC

$Script:VaultDir = Join-Path $env:USERPROFILE ".flamevault"
$Script:HoneypotsFile = Join-Path $Script:VaultDir "honeypots.json"
$Script:AlertsFile = Join-Path $Script:VaultDir "alerts.log"

# Get the flamevault binary path
function Get-FlameVaultBinary {
    $binary = "flamevault"
    if ($IsWindows) {
        $binary = "flamevault.exe"
    }
    
    # Check if in PATH
    $cmd = Get-Command $binary -ErrorAction SilentlyContinue
    if ($cmd) {
        return $cmd.Source
    }
    
    # Check in cargo target directory
    $scriptDir = Split-Path -Parent $PSCommandPath
    $targetPath = Join-Path $scriptDir "target/release/$binary"
    if (Test-Path $targetPath) {
        return $targetPath
    }
    
    return $null
}

<#
.SYNOPSIS
Initialize FlameVault with machine-bound encryption

.DESCRIPTION
Creates the vault directory and initializes machine-bound encryption keys
derived from STRAT_DEVICE_ID, STRAT_HEMISPHERE, STRAT_REGION, and hardware UID.

.EXAMPLE
Initialize-FlameVault
#>
function Initialize-FlameVault {
    [CmdletBinding()]
    param()
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    & $binary init
}

<#
.SYNOPSIS
Store an encrypted secret

.DESCRIPTION
Encrypts and stores a secret value using machine-bound encryption.
The secret can only be decrypted on the same machine.

.PARAMETER Name
The name of the secret

.PARAMETER Value
The secret value to encrypt and store

.EXAMPLE
Set-FlameSecret -Name "REAL_OPENAI_KEY" -Value "sk-your-actual-key"
#>
function Set-FlameSecret {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        
        [Parameter(Mandatory = $true)]
        [string]$Value
    )
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    & $binary set $Name $Value
}

<#
.SYNOPSIS
Retrieve a secret (decrypt real secrets, log honeypot access)

.DESCRIPTION
Retrieves a secret from FlameVault. If the secret is a honeypot,
it logs the access and returns the bait value. If it's a real secret,
it decrypts and returns the actual value.

.PARAMETER Name
The name of the secret to retrieve

.PARAMETER Raw
Output raw value without formatting

.EXAMPLE
$key = Get-FlameSecret -Name "REAL_OPENAI_KEY" -Raw

.EXAMPLE
Get-FlameSecret -Name "OPENAI_API_KEY"
#>
function Get-FlameSecret {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        
        [switch]$Raw
    )
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    if ($Raw) {
        & $binary get $Name --raw
    } else {
        & $binary get $Name
    }
}

<#
.SYNOPSIS
Deploy a honeypot trap

.DESCRIPTION
Sets up a honeypot trap with the specified name and bait value.
When accessed, the honeypot logs the access attempt.

.PARAMETER Name
The name of the honeypot

.PARAMETER Value
The bait value

.EXAMPLE
Set-FlameHoneypot -Name "FAKE_API_KEY" -Value "sk-fake-key-123"
#>
function Set-FlameHoneypot {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        
        [Parameter(Mandatory = $true)]
        [string]$Value
    )
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    & $binary honeypot $Name $Value
}

<#
.SYNOPSIS
Deploy standard honeypot traps from environment variables

.DESCRIPTION
Scans environment variables for potentially exposed secrets and
converts them into honeypot traps. Supports OPENAI_API_KEY, XAI_API_KEY,
EMAIL_PASS, GITHUB_TOKEN, AWS keys, and more.

.PARAMETER FromEnv
Load honeypots from current environment variables

.EXAMPLE
Deploy-FlameTraps -FromEnv
#>
function Deploy-FlameTraps {
    [CmdletBinding()]
    param(
        [switch]$FromEnv
    )
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    if ($FromEnv) {
        & $binary deploy-traps --from-env
    } else {
        & $binary deploy-traps
    }
}

<#
.SYNOPSIS
View access alerts for honeypots

.DESCRIPTION
Displays all logged honeypot access attempts, including timestamp,
process information, and user details.

.PARAMETER Recent
Show only recent alerts (last 10)

.EXAMPLE
Get-FlameAlerts

.EXAMPLE
Get-FlameAlerts -Recent
#>
function Get-FlameAlerts {
    [CmdletBinding()]
    param(
        [switch]$Recent
    )
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    if ($Recent) {
        & $binary alerts --recent
    } else {
        & $binary alerts
    }
}

<#
.SYNOPSIS
List all secrets and honeypots

.DESCRIPTION
Lists all encrypted secrets and honeypot traps configured in FlameVault.

.EXAMPLE
Get-FlameVaultList
#>
function Get-FlameVaultList {
    [CmdletBinding()]
    param()
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    & $binary list
}

<#
.SYNOPSIS
Show FlameVault status

.DESCRIPTION
Displays vault status including directory location, machine ID,
secret count, honeypot count, and alert count.

.EXAMPLE
Get-FlameVaultStatus
#>
function Get-FlameVaultStatus {
    [CmdletBinding()]
    param()
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    & $binary status
}

<#
.SYNOPSIS
Export environment variables script

.DESCRIPTION
Exports a script that sets environment variables with real secret values.
Use this to inject secrets into your environment safely.

.PARAMETER Format
The script format (powershell, bash, or cmd)

.EXAMPLE
Export-FlameEnv -Format powershell | Out-File secrets.ps1
#>
function Export-FlameEnv {
    [CmdletBinding()]
    param(
        [ValidateSet("powershell", "bash", "cmd")]
        [string]$Format = "powershell"
    )
    
    $binary = Get-FlameVaultBinary
    if (-not $binary) {
        Write-Error "flamevault binary not found. Please build the project first: cargo build --release"
        return
    }
    
    # Get list of secrets
    $listOutput = & $binary list 2>&1
    
    $secrets = @()
    foreach ($line in $listOutput) {
        if ($line -match '^\s*•\s*(\S+)\s+\(encrypted\)') {
            $secrets += $Matches[1]
        }
    }
    
    # Generate export script
    foreach ($secretName in $secrets) {
        $value = & $binary get $secretName --raw 2>&1
        
        switch ($Format) {
            "powershell" {
                Write-Output "`$env:$secretName = `"$value`""
            }
            "bash" {
                Write-Output "export $secretName=`"$value`""
            }
            "cmd" {
                Write-Output "set $secretName=$value"
            }
        }
    }
}

# Export module members
Export-ModuleMember -Function @(
    'Initialize-FlameVault',
    'Set-FlameSecret',
    'Get-FlameSecret',
    'Set-FlameHoneypot',
    'Deploy-FlameTraps',
    'Get-FlameAlerts',
    'Get-FlameVaultList',
    'Get-FlameVaultStatus',
    'Export-FlameEnv'
)

Write-Host "🔥 FlameVault PowerShell Module loaded" -ForegroundColor Yellow
Write-Host "   Use Initialize-FlameVault to get started" -ForegroundColor Gray
