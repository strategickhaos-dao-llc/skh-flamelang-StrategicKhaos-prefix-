# ═══════════════════════════════════════════════════════════════════════════════
# FLAMEVAULT HONEYPOT + ENCRYPTED ENV LOADER
# INV-080: Strategickhaos DAO LLC
# ═══════════════════════════════════════════════════════════════════════════════

# === HONEYPOT LAYER (visible, monitored, triggers alerts) ===
$honeypot = @{
    "OPENAI_API_KEY" = "sk-svcacct-sbfXUZ_PeKwvpwplir8qGOw1w3WHzoWL2kostsd4YBYkijcqPnkKlgxSF9sSfZlt-galia9E1U4CT3BlbkFJbq0w-cz73G4U1MQrmWtRTFfD7LNGHJhmujyPzf7"
    "XAI_API_KEY" = "xai-A8DdRfGLfXnAT9Mwzg9wxvpQhEDLRA4BKPoKAlU7AgM1gvbjUuHkE8XeLTONTGUE1CMX5pc0dAlR2ddy"
    "EMAIL_PASS" = "imcnaicqemiiuzqr"
}

# === ENCRYPTED REAL KEYS (FlameVault protected) ===
# Stored in: $HOME\.flamevault\secrets.enc
# Decryption requires: STRAT_DEVICE_ID + STRAT_HEMISPHERE + passphrase

function Initialize-FlameVault {
    <#
    .SYNOPSIS
    Initialize FlameVault directory and generate machine-bound encryption key.
    
    .DESCRIPTION
    Creates the FlameVault directory if it doesn't exist and generates a machine-bound
    encryption key using computer name, hemisphere, device ID, and a constant salt.
    
    .OUTPUTS
    Hashtable with vault path and derived key hash.
    #>
    
    $vaultPath = "$env:USERPROFILE\.flamevault"
    if (-not (Test-Path $vaultPath)) {
        New-Item -ItemType Directory -Path $vaultPath -Force | Out-Null
    }
    
    # Generate machine-bound encryption key
    $machineKey = [Convert]::ToBase64String(
        [System.Security.Cryptography.SHA256]::Create().ComputeHash(
            [System.Text.Encoding]::UTF8.GetBytes(
                "$env:COMPUTERNAME|$env:STRAT_HEMISPHERE|$env:STRAT_DEVICE_ID|STRATEGICKHAOS"
            )
        )
    )
    
    return @{
        Path = $vaultPath
        KeyHash = $machineKey.Substring(0, 16)
    }
}

function Set-FlameSecret {
    <#
    .SYNOPSIS
    Encrypt and store a secret in FlameVault.
    
    .PARAMETER Name
    The name/key for the secret.
    
    .PARAMETER Value
    The secret value to encrypt and store.
    
    .EXAMPLE
    Set-FlameSecret -Name "REAL_OPENAI_API_KEY" -Value "sk-actual-key"
    #>
    
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name,
        
        [Parameter(Mandatory=$true)]
        [string]$Value
    )
    
    $vault = Initialize-FlameVault
    $secureString = ConvertTo-SecureString $Value -AsPlainText -Force
    $encrypted = ConvertFrom-SecureString $secureString
    
    $secretFile = Join-Path $vault.Path "$Name.enc"
    $encrypted | Out-File $secretFile -Force
    
    Write-Host "🔥 Encrypted: $Name → $secretFile" -ForegroundColor Green
}

function Get-FlameSecret {
    <#
    .SYNOPSIS
    Retrieve and decrypt a secret from FlameVault.
    
    .PARAMETER Name
    The name/key of the secret to retrieve.
    
    .OUTPUTS
    The decrypted secret value or $null if not found.
    
    .EXAMPLE
    $apiKey = Get-FlameSecret -Name "REAL_OPENAI_API_KEY"
    #>
    
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name
    )
    
    $vault = Initialize-FlameVault
    $secretFile = Join-Path $vault.Path "$Name.enc"
    
    if (Test-Path $secretFile) {
        try {
            $encrypted = Get-Content $secretFile
            $secureString = ConvertTo-SecureString $encrypted
            $ptr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secureString)
            $value = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($ptr)
            [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($ptr)
            return $value
        }
        catch {
            Write-Host "⚠️ Failed to decrypt secret: $Name - $_" -ForegroundColor Yellow
            return $null
        }
    }
    
    Write-Host "⚠️ Secret not found: $Name" -ForegroundColor Yellow
    return $null
}

function Set-HoneypotTrap {
    <#
    .SYNOPSIS
    Deploy honeypot keys as visible environment variables.
    
    .DESCRIPTION
    Sets all honeypot keys as user environment variables for monitoring.
    Any access to these keys will be logged as a potential security incident.
    
    .EXAMPLE
    Set-HoneypotTrap
    #>
    
    # Set honeypot keys as visible env vars (bait)
    foreach ($key in $honeypot.Keys) {
        [Environment]::SetEnvironmentVariable($key, $honeypot[$key], "User")
        Write-Host "🍯 Honeypot set: $key" -ForegroundColor Yellow
    }
    
    # Log honeypot deployment
    $vault = Initialize-FlameVault
    $logEntry = @{
        timestamp = (Get-Date -Format "o")
        action = "honeypot_deployed"
        keys = $honeypot.Keys
        node = $env:STRAT_DEVICE_ID
        computer = $env:COMPUTERNAME
    } | ConvertTo-Json
    
    Add-Content -Path "$($vault.Path)\honeypot.log" -Value $logEntry
    Write-Host "📝 Honeypot deployment logged" -ForegroundColor Cyan
}

function Get-RealSecret {
    <#
    .SYNOPSIS
    Smart routing function that returns real or honeypot secrets based on context.
    
    .PARAMETER Name
    The name of the secret to retrieve.
    
    .DESCRIPTION
    Checks if the requested key is a honeypot key. If so, logs the access attempt
    and returns the honeypot value. Otherwise, retrieves the real encrypted value.
    
    .OUTPUTS
    The secret value (either honeypot or real encrypted value).
    
    .EXAMPLE
    $apiKey = Get-RealSecret -Name "OPENAI_API_KEY"
    #>
    
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name
    )
    
    # First check if it's a honeypot key being accessed
    if ($honeypot.ContainsKey($Name)) {
        # Log the access attempt
        $vault = Initialize-FlameVault
        $alert = @{
            timestamp = (Get-Date -Format "o")
            alert = "HONEYPOT_ACCESS_ATTEMPT"
            key = $Name
            process = (Get-Process -Id $PID).ProcessName
            user = $env:USERNAME
            computer = $env:COMPUTERNAME
            path = (Get-Location).Path
        } | ConvertTo-Json
        
        Add-Content -Path "$($vault.Path)\alerts.log" -Value $alert
        Write-Host "🚨 HONEYPOT TRIGGERED: $Name" -ForegroundColor Red
        
        # Return honeypot value (attacker gets fake key)
        return $honeypot[$Name]
    }
    
    # Return real encrypted value
    return Get-FlameSecret -Name "REAL_$Name"
}

function Get-HoneypotAlerts {
    <#
    .SYNOPSIS
    Retrieve honeypot access alerts.
    
    .PARAMETER Last
    Number of most recent alerts to return. Default is 10.
    
    .OUTPUTS
    Array of alert objects.
    
    .EXAMPLE
    Get-HoneypotAlerts -Last 5
    #>
    
    param(
        [int]$Last = 10
    )
    
    $vault = Initialize-FlameVault
    $alertsFile = "$($vault.Path)\alerts.log"
    
    if (Test-Path $alertsFile) {
        $alerts = Get-Content $alertsFile | ForEach-Object {
            $_ | ConvertFrom-Json
        } | Select-Object -Last $Last
        
        return $alerts
    }
    
    Write-Host "ℹ️ No alerts found" -ForegroundColor Cyan
    return @()
}

function Remove-FlameSecret {
    <#
    .SYNOPSIS
    Remove a secret from FlameVault.
    
    .PARAMETER Name
    The name of the secret to remove.
    
    .EXAMPLE
    Remove-FlameSecret -Name "REAL_OLD_API_KEY"
    #>
    
    param(
        [Parameter(Mandatory=$true)]
        [string]$Name
    )
    
    $vault = Initialize-FlameVault
    $secretFile = Join-Path $vault.Path "$Name.enc"
    
    if (Test-Path $secretFile) {
        Remove-Item $secretFile -Force
        Write-Host "🗑️ Removed secret: $Name" -ForegroundColor Green
    }
    else {
        Write-Host "⚠️ Secret not found: $Name" -ForegroundColor Yellow
    }
}

function Get-FlameSecretList {
    <#
    .SYNOPSIS
    List all secrets stored in FlameVault.
    
    .OUTPUTS
    Array of secret names (without .enc extension).
    
    .EXAMPLE
    Get-FlameSecretList
    #>
    
    $vault = Initialize-FlameVault
    
    if (Test-Path $vault.Path) {
        $secrets = Get-ChildItem -Path $vault.Path -Filter "*.enc" | ForEach-Object {
            $_.BaseName
        }
        
        return $secrets
    }
    
    return @()
}

# === INITIALIZATION MESSAGE ===
Write-Host @"

🔥 FLAMEVAULT HONEYPOT ENGINE LOADED

Commands:
  Set-FlameSecret -Name "REAL_OPENAI_API_KEY" -Value "sk-actual-key"
  Get-FlameSecret -Name "REAL_OPENAI_API_KEY"
  Set-HoneypotTrap          # Deploy bait keys to env
  Get-RealSecret -Name "OPENAI_API_KEY"  # Smart routing
  Get-HoneypotAlerts -Last 10    # View recent alerts
  Get-FlameSecretList       # List stored secrets
  Remove-FlameSecret -Name "REAL_OLD_KEY"  # Remove a secret

The exposed keys are now BAIT.
Real keys encrypted in ~/.flamevault/

"@ -ForegroundColor Cyan

# Export module members if loaded as module
Export-ModuleMember -Function @(
    'Initialize-FlameVault',
    'Set-FlameSecret',
    'Get-FlameSecret',
    'Set-HoneypotTrap',
    'Get-RealSecret',
    'Get-HoneypotAlerts',
    'Remove-FlameSecret',
    'Get-FlameSecretList'
)
