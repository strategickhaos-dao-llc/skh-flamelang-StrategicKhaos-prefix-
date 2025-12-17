# Git Authentication Fix Script for StrategicKhaos FlameLang Repository
# This script automates the resolution of Git authentication and permission issues

[CmdletBinding()]
param(
    [Parameter(Mandatory=$false)]
    [switch]$ForcePush,
    
    [Parameter(Mandatory=$false)]
    [switch]$AbortRebase,
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipCredentialClear,
    
    [Parameter(Mandatory=$false)]
    [string]$TargetBranch = "main"
)

# Color output functions
function Write-Step {
    param([string]$Message)
    Write-Host "`n==> $Message" -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "    ✓ $Message" -ForegroundColor Green
}

function Write-Warning-Custom {
    param([string]$Message)
    Write-Host "    ⚠ $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "    ✗ $Message" -ForegroundColor Red
}

# Main script
Write-Host @"

╔════════════════════════════════════════════════════════════════╗
║  StrategicKhaos FlameLang - Git Authentication Fix Script     ║
║  Repository: skh-flamelang-StrategicKhaos-prefix-             ║
╚════════════════════════════════════════════════════════════════╝

"@ -ForegroundColor Magenta

# Step 1: Abort rebase if needed
if ($AbortRebase -or (Test-Path ".git/rebase-merge") -or (Test-Path ".git/rebase-apply")) {
    Write-Step "Step 1: Aborting broken rebase"
    try {
        git rebase --abort 2>&1 | Out-Null
        Write-Success "Rebase aborted successfully"
    }
    catch {
        Write-Warning-Custom "No rebase in progress or already aborted"
    }
}
else {
    Write-Step "Step 1: Checking rebase state"
    Write-Success "No active rebase detected"
}

# Step 2: Check credential helper
Write-Step "Step 2: Checking credential helper configuration"
$credHelper = git config --global credential.helper
if ($credHelper) {
    Write-Host "    Current credential helper: $credHelper" -ForegroundColor Yellow
}
else {
    Write-Success "No global credential helper configured"
}

# Step 3: Clear cached credentials
if (-not $SkipCredentialClear) {
    Write-Step "Step 3: Clearing cached GitHub credentials"
    Write-Warning-Custom "This will remove stored GitHub credentials"
    
    $confirmation = Read-Host "    Continue? (y/n)"
    if ($confirmation -eq 'y') {
        try {
            # Create input for git credential reject
            # Format: protocol=https\nhost=github.com\n\n (two newlines at end)
            $input = "protocol=https`nhost=github.com`n`n"
            $input | git credential reject
            Write-Success "Cached credentials cleared"
        }
        catch {
            Write-Warning-Custom "Could not clear credentials automatically. Please clear manually via Credential Manager."
        }
    }
    else {
        Write-Warning-Custom "Skipped credential clearing"
    }
}
else {
    Write-Step "Step 3: Skipping credential clearing (--SkipCredentialClear flag)"
}

# Step 4: Force SSH usage
Write-Step "Step 4: Configuring Git to use SSH instead of HTTPS"
try {
    git config --global url."git@github.com:".insteadOf "https://github.com/"
    Write-Success "SSH override configured"
}
catch {
    Write-Error-Custom "Failed to configure SSH override"
}

# Step 5: Verify SSH authentication
Write-Step "Step 5: Verifying SSH authentication to GitHub"
try {
    $sshTest = ssh -T git@github.com 2>&1
    if ($sshTest -match "successfully authenticated") {
        Write-Success "SSH authentication successful!"
        Write-Host "    $sshTest" -ForegroundColor Gray
    }
    else {
        Write-Error-Custom "SSH authentication failed"
        Write-Host "    $sshTest" -ForegroundColor Gray
        Write-Warning-Custom "You may need to add your SSH key to GitHub or as a deploy key"
    }
}
catch {
    Write-Error-Custom "Could not test SSH connection"
}

# Step 6: Check Git user configuration
Write-Step "Step 6: Checking Git user configuration"
$currentName = git config user.name
$currentEmail = git config user.email
Write-Host "    Current user.name:  $currentName" -ForegroundColor Gray
Write-Host "    Current user.email: $currentEmail" -ForegroundColor Gray

# Step 7: Set organization identity
Write-Step "Step 7: Setting organization identity"
$correctName = "strategickhaos-dao-llc"
$correctEmail = "security@strategickhaos.ai"

if ($currentName -ne $correctName -or $currentEmail -ne $correctEmail) {
    Write-Warning-Custom "Current configuration doesn't match organization identity"
    $setIdentity = Read-Host "    Set to organization identity? (y/n)"
    
    if ($setIdentity -eq 'y') {
        git config user.name $correctName
        git config user.email $correctEmail
        Write-Success "User identity updated to organization account"
    }
}
else {
    Write-Success "User identity already matches organization account"
}

# Step 8: Verify and update remote URL
Write-Step "Step 8: Verifying remote URL configuration"
$remoteUrl = git remote get-url origin
Write-Host "    Current remote URL: $remoteUrl" -ForegroundColor Gray

if ($remoteUrl -match "^https://") {
    Write-Warning-Custom "Remote is using HTTPS, updating to SSH"
    try {
        git remote set-url origin "git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git"
        Write-Success "Remote URL updated to SSH"
    }
    catch {
        Write-Error-Custom "Failed to update remote URL"
    }
}
else {
    Write-Success "Remote is already using SSH"
}

# Step 9: Checkout target branch and push
Write-Step "Step 9: Preparing to push changes"

# Check current branch
$currentBranch = git rev-parse --abbrev-ref HEAD
Write-Host "    Current branch: $currentBranch" -ForegroundColor Gray

if ($currentBranch -ne $TargetBranch) {
    Write-Warning-Custom "Not on $TargetBranch branch"
    $checkout = Read-Host "    Checkout $TargetBranch? (y/n)"
    
    if ($checkout -eq 'y') {
        try {
            git checkout $TargetBranch
            Write-Success "Switched to $TargetBranch branch"
        }
        catch {
            Write-Error-Custom "Failed to checkout $TargetBranch branch"
            exit 1
        }
    }
    else {
        Write-Warning-Custom "Staying on current branch"
    }
}

# Push
Write-Step "Step 10: Pushing changes to remote"

if ($ForcePush) {
    Write-Warning-Custom "FORCE PUSH requested - this will overwrite remote history!"
    $confirmForce = Read-Host "    Are you absolutely sure? Type 'force' to continue"
    
    if ($confirmForce -eq 'force') {
        try {
            git push origin $TargetBranch --force
            Write-Success "Force push completed"
        }
        catch {
            Write-Error-Custom "Force push failed"
            Write-Host "`nIf permission was denied, you may need to add a deploy key with write access." -ForegroundColor Yellow
            Write-Host "See GIT_AUTHENTICATION_FIX.md for instructions." -ForegroundColor Yellow
        }
    }
    else {
        Write-Warning-Custom "Force push cancelled"
    }
}
else {
    $doPush = Read-Host "    Push to origin/$TargetBranch? (y/n)"
    
    if ($doPush -eq 'y') {
        try {
            git push origin $TargetBranch
            Write-Success "Push completed successfully"
        }
        catch {
            Write-Error-Custom "Push failed"
            Write-Host "`nIf permission was denied, you may need to:" -ForegroundColor Yellow
            Write-Host "  1. Add your SSH key as a deploy key with write access" -ForegroundColor Yellow
            Write-Host "  2. Use --ForcePush flag if you need to overwrite remote" -ForegroundColor Yellow
            Write-Host "`nSee GIT_AUTHENTICATION_FIX.md for detailed instructions." -ForegroundColor Yellow
        }
    }
    else {
        Write-Warning-Custom "Push cancelled by user"
    }
}

# Summary
Write-Host @"

╔════════════════════════════════════════════════════════════════╗
║  Script Execution Complete                                     ║
╚════════════════════════════════════════════════════════════════╝

Next Steps:
  • If push was denied, add deploy key via GitHub web UI
  • Repository: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/settings/keys
  • See GIT_AUTHENTICATION_FIX.md for complete instructions

Usage Examples:
  .\fix-git-auth.ps1                           # Interactive mode
  .\fix-git-auth.ps1 -AbortRebase              # Abort rebase first
  .\fix-git-auth.ps1 -ForcePush                # Enable force push option
  .\fix-git-auth.ps1 -SkipCredentialClear      # Skip credential clearing
  .\fix-git-auth.ps1 -TargetBranch develop     # Target different branch

"@ -ForegroundColor Cyan
