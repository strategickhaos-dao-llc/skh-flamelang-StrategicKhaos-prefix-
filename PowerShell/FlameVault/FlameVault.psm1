<#
.SYNOPSIS
    FlameVault PowerShell Module — Honeypot + Encrypted Secrets Engine
    
.DESCRIPTION
    INV-080: Strategickhaos DAO LLC
    Machine-bound encryption with honeypot trap for exposed credentials.
    
.NOTES
    Author: Strategickhaos DAO LLC
    Version: 0.1.0
#>

$script:VaultPath = "$env:USERPROFILE\.flamevault"
$script:Honeypots = @{}
$script:MachineKey = $null

function Initialize-FlameVault {
    <#
    .SYNOPSIS
        Initialize FlameVault with machine-bound encryption
    #>
    
    # Create vault directory
    if (-not (Test-Path $script:VaultPath)) {
        New-Item -ItemType Directory -Path $script:VaultPath -Force | Out-Null
    }
    
    # Generate machine-bound key
    $deviceId = if ($env:STRAT_DEVICE_ID) { $env:STRAT_DEVICE_ID } else { $env:COMPUTERNAME }
    $hemisphere = if ($env:STRAT_HEMISPHERE) { $env:STRAT_HEMISPHERE } else { "Unknown" }
    $region = if ($env:STRAT_REGION) { $env:STRAT_REGION } else { "US" }
    
    $combined = "$deviceId|$hemisphere|$region|STRATEGICKHAOS-FLAMEVAULT-7%"
    
    $sha256 = [System.Security.Cryptography.SHA256]::Create()
    $keyBytes = $sha256.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($combined))
    $script:MachineKey = $keyBytes
    
    # Load honeypots
    $honeypotFile = Join-Path $script:VaultPath "honeypots.json"
    if (Test-Path $honeypotFile) {
        $script:Honeypots = Get-Content $honeypotFile | ConvertFrom-Json -AsHashtable
    }
    
    Write-Host @"

🔥 FLAMEVAULT INITIALIZED
═══════════════════════════════════════
Device ID:    $deviceId
Hemisphere:   $hemisphere
Region:       $region
Vault Path:   $($script:VaultPath)
Machine Key:  $([Convert]::ToBase64String($keyBytes[0..7]))...
═══════════════════════════════════════

"@ -ForegroundColor Cyan
    
    return @{
        DeviceId = $deviceId
        Hemisphere = $hemisphere
        Region = $region
        VaultPath = $script:VaultPath
        KeyHash = [Convert]::ToBase64String($keyBytes[0..7])
    }
}

function Set-FlameSecret {
    <#
    .SYNOPSIS
        Encrypt and store a secret (machine-bound)
    .PARAMETER Name
        Secret name
    .PARAMETER Value
        Secret value to encrypt
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Name,
        
        [Parameter(Mandatory)]
        [string]$Value
    )
    
    if (-not $script:MachineKey) {
        Initialize-FlameVault | Out-Null
    }
    
    # Encrypt using DPAPI (machine-bound)
    $secureString = ConvertTo-SecureString $Value -AsPlainText -Force
    $encrypted = ConvertFrom-SecureString $secureString -Key $script:MachineKey[0..31]
    
    # Create secret object
    $secret = @{
        Name = $Name
        Encrypted = $encrypted
        CreatedAt = (Get-Date).ToString("o")
        MachineHash = [Convert]::ToBase64String($script:MachineKey[0..7])
    }
    
    # Save to file
    $secretFile = Join-Path $script:VaultPath "$Name.enc.json"
    $secret | ConvertTo-Json | Out-File $secretFile -Force
    
    Write-Host "🔥 Encrypted: $Name → $secretFile" -ForegroundColor Green
}

function Get-FlameSecret {
    <#
    .SYNOPSIS
        Retrieve and decrypt a secret
    .PARAMETER Name
        Secret name
    .PARAMETER Raw
        Output raw value only (for scripts)
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Name,
        
        [switch]$Raw
    )
    
    if (-not $script:MachineKey) {
        Initialize-FlameVault | Out-Null
    }
    
    # Check if honeypot
    if ($script:Honeypots.ContainsKey($Name)) {
        $honeypot = $script:Honeypots[$Name]
        
        # Log alert
        $alert = @{
            Timestamp = (Get-Date).ToString("o")
            AlertType = "HONEYPOT_ACCESS"
            KeyName = $Name
            ProcessName = (Get-Process -Id $PID).ProcessName
            Username = $env:USERNAME
            DeviceId = $env:STRAT_DEVICE_ID
        } | ConvertTo-Json -Compress
        
        Add-Content -Path (Join-Path $script:VaultPath "alerts.log") -Value $alert
        
        Write-Host "🚨 HONEYPOT TRIGGERED: $Name" -ForegroundColor Red
        
        if ($Raw) {
            return $honeypot.Value
        } else {
            return @{
                Type = "Honeypot"
                Name = $Name
                Value = $honeypot.Value
            }
        }
    }
    
    # Try to load real secret
    $secretFile = Join-Path $script:VaultPath "$Name.enc.json"
    
    if (-not (Test-Path $secretFile)) {
        Write-Host "❌ Secret not found: $Name" -ForegroundColor Yellow
        return $null
    }
    
    try {
        $secret = Get-Content $secretFile | ConvertFrom-Json
        
        # Decrypt
        $secureString = ConvertTo-SecureString $secret.Encrypted -Key $script:MachineKey[0..31]
        $ptr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secureString)
        $value = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($ptr)
        [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($ptr)
        
        if ($Raw) {
            return $value
        } else {
            return @{
                Type = "Real"
                Name = $Name
                Value = $value
            }
        }
    }
    catch {
        Write-Host "❌ Decryption failed (wrong machine?): $_" -ForegroundColor Red
        return $null
    }
}

function Set-FlameHoneypot {
    <#
    .SYNOPSIS
        Deploy a honeypot (bait) key
    .PARAMETER Name
        Key name
    .PARAMETER Value
        Bait value (the fake/exposed key)
    #>
    param(
        [Parameter(Mandatory)]
        [string]$Name,
        
        [Parameter(Mandatory)]
        [string]$Value
    )
    
    if (-not $script:MachineKey) {
        Initialize-FlameVault | Out-Null
    }
    
    $script:Honeypots[$Name] = @{
        Name = $Name
        Value = $Value
        DeployedAt = (Get-Date).ToString("o")
        AccessCount = 0
    }
    
    # Save honeypots
    $honeypotFile = Join-Path $script:VaultPath "honeypots.json"
    $script:Honeypots | ConvertTo-Json | Out-File $honeypotFile -Force
    
    Write-Host "🍯 Honeypot deployed: $Name" -ForegroundColor Yellow
}

function Deploy-FlameTraps {
    <#
    .SYNOPSIS
        Deploy standard Strategickhaos honeypot traps
    .PARAMETER FromEnv
        Also capture currently exposed env vars as honeypots
    #>
    param(
        [switch]$FromEnv
    )
    
    Write-Host "🍯 Deploying Strategickhaos honeypot traps..." -ForegroundColor Yellow
    
    # Standard honeypots
    $traps = @{
        "OPENAI_API_KEY" = "sk-honeypot-strategickhaos-trap-do-not-use"
        "XAI_API_KEY" = "xai-honeypot-strategickhaos-trap-do-not-use"
        "ANTHROPIC_API_KEY" = "sk-ant-honeypot-strategickhaos-trap"
        "GITHUB_TOKEN" = "ghp_honeypotStrategickhaosDoNotUse000000"
        "AWS_SECRET_ACCESS_KEY" = "honeypot/strategickhaos/trap/key"
    }
    
    foreach ($trap in $traps.GetEnumerator()) {
        Set-FlameHoneypot -Name $trap.Key -Value $trap.Value
    }
    
    if ($FromEnv) {
        Write-Host "`n   Scanning environment for exposed keys..." -ForegroundColor Gray
        
        @("OPENAI_API_KEY", "XAI_API_KEY", "EMAIL_PASS", "GH_TOKEN") | ForEach-Object {
            $value = [Environment]::GetEnvironmentVariable($_)
            if ($value -and $value -notmatch "honeypot") {
                Write-Host "   🔍 Found: $_ (converting to honeypot)" -ForegroundColor Gray
                Set-FlameHoneypot -Name "ENV_$_" -Value $value
            }
        }
    }
    
    Write-Host "`n✅ Honeypot traps deployed!" -ForegroundColor Green
}

function Get-FlameAlerts {
    <#
    .SYNOPSIS
        View honeypot access alerts
    .PARAMETER Count
        Number of recent alerts to show
    #>
    param(
        [int]$Count = 10
    )
    
    $alertFile = Join-Path $script:VaultPath "alerts.log"
    
    if (-not (Test-Path $alertFile)) {
        Write-Host "📭 No alerts yet" -ForegroundColor Gray
        return
    }
    
    $alerts = Get-Content $alertFile | Select-Object -Last $Count
    
    Write-Host "🚨 RECENT HONEYPOT ALERTS:`n" -ForegroundColor Red
    
    foreach ($line in $alerts) {
        $alert = $line | ConvertFrom-Json
        Write-Host "  $($alert.Timestamp) | $($alert.KeyName) | $($alert.ProcessName)" -ForegroundColor Yellow
    }
}

function Export-FlameEnv {
    <#
    .SYNOPSIS
        Export environment with honeypots set as visible bait
    #>
    
    if (-not $script:MachineKey) {
        Initialize-FlameVault | Out-Null
    }
    
    Write-Host "# FLAMEVAULT ENVIRONMENT EXPORT" -ForegroundColor Cyan
    Write-Host "# Honeypots (bait):" -ForegroundColor Gray
    
    foreach ($pot in $script:Honeypots.GetEnumerator()) {
        Write-Host "`$env:$($pot.Key) = `"$($pot.Value.Value)`"  # 🍯 HONEYPOT"
    }
    
    Write-Host "`n# Real secrets (use Get-FlameSecret):" -ForegroundColor Gray
    
    Get-ChildItem (Join-Path $script:VaultPath "*.enc.json") | ForEach-Object {
        $name = $_.BaseName -replace "\.enc$", ""
        Write-Host "# `$env:$($name.ToUpper()) = `$(Get-FlameSecret -Name '$name' -Raw)"
    }
}

# Export module functions
Export-ModuleMember -Function @(
    'Initialize-FlameVault',
    'Set-FlameSecret',
    'Get-FlameSecret',
    'Set-FlameHoneypot',
    'Deploy-FlameTraps',
    'Get-FlameAlerts',
    'Export-FlameEnv'
)

Write-Host @"
🔥 FlameVault PowerShell Module Loaded

Commands:
  Initialize-FlameVault          # Initialize vault
  Set-FlameSecret -Name X -Value Y   # Store encrypted secret
  Get-FlameSecret -Name X            # Retrieve secret
  Set-FlameHoneypot -Name X -Value Y # Deploy bait key
  Deploy-FlameTraps                  # Deploy standard traps
  Get-FlameAlerts                    # View access alerts
  Export-FlameEnv                    # Export env script

"@ -ForegroundColor Cyan
