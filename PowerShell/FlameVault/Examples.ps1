<#
.SYNOPSIS
    FlameVault Usage Examples
    
.DESCRIPTION
    Demonstrates various use cases and functionality of the FlameVault module
#>

# Import the module
Import-Module "$PSScriptRoot\FlameVault.psd1" -Force

Write-Host "`n=== FlameVault Examples ===" -ForegroundColor Cyan

# Example 1: Initialize Vault
Write-Host "`n--- Example 1: Initialize Vault ---" -ForegroundColor Yellow
$vaultInfo = Initialize-FlameVault
Write-Host "Vault initialized at: $($vaultInfo.VaultPath)"

# Example 2: Store Encrypted Secrets
Write-Host "`n--- Example 2: Store Encrypted Secrets ---" -ForegroundColor Yellow
Set-FlameSecret -Name "EXAMPLE_API_KEY" -Value "sk-example-12345-secret-key"
Set-FlameSecret -Name "DATABASE_PASSWORD" -Value "SuperSecretPassword123!"

# Example 3: Retrieve Secrets
Write-Host "`n--- Example 3: Retrieve Secrets ---" -ForegroundColor Yellow
$secret = Get-FlameSecret -Name "EXAMPLE_API_KEY"
Write-Host "Secret Type: $($secret.Type)"
Write-Host "Secret Name: $($secret.Name)"
Write-Host "Secret Value: $($secret.Value)"

# Example 4: Deploy Honeypot Traps
Write-Host "`n--- Example 4: Deploy Honeypot Traps ---" -ForegroundColor Yellow
Deploy-FlameTraps

# Example 5: Create Custom Honeypot
Write-Host "`n--- Example 5: Create Custom Honeypot ---" -ForegroundColor Yellow
Set-FlameHoneypot -Name "PROD_SSH_KEY" -Value "fake-ssh-key-honeypot-trap"

# Example 6: Simulate Honeypot Access (for demo)
Write-Host "`n--- Example 6: Simulate Honeypot Access ---" -ForegroundColor Yellow
$honeypotValue = Get-FlameSecret -Name "OPENAI_API_KEY" -Raw
Write-Host "Accessed honeypot, got value: $honeypotValue"

# Example 7: View Alerts
Write-Host "`n--- Example 7: View Security Alerts ---" -ForegroundColor Yellow
Get-FlameAlerts

# Example 8: Export Environment
Write-Host "`n--- Example 8: Export Environment Variables ---" -ForegroundColor Yellow
Export-FlameEnv

# Example 9: Using in Scripts
Write-Host "`n--- Example 9: Using Secrets in Scripts ---" -ForegroundColor Yellow
Write-Host @"
# In your scripts, retrieve secrets like this:
`$apiKey = Get-FlameSecret -Name 'EXAMPLE_API_KEY' -Raw
`$dbPassword = Get-FlameSecret -Name 'DATABASE_PASSWORD' -Raw

# Use the secrets
Write-Host "Using API Key: `$(`$apiKey.Substring(0, 10))..."
"@

# Example 10: Clean Up (optional)
Write-Host "`n--- Example 10: Vault Location ---" -ForegroundColor Yellow
Write-Host "Your vault is located at: $env:USERPROFILE\.flamevault"
Write-Host "Secrets are machine-bound and cannot be decrypted on other machines."

Write-Host "`n=== Examples Complete ===" -ForegroundColor Green
Write-Host "Check $env:USERPROFILE\.flamevault for vault files and alerts.log" -ForegroundColor Gray
