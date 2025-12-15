# Deploy Key Setup for Write Access

## Overview

This guide provides step-by-step instructions for adding the FlameLang SSH key as a deploy key with write access to the repository. This is necessary when SSH authentication succeeds but push operations are still denied.

## When to Use This Guide

Use this guide if you experience:
- ✅ SSH authentication to GitHub succeeds (`ssh -T git@github.com` works)
- ❌ Push operations are denied with permission errors
- ❌ Error message: "You do not have permission to push to this repository"

## Prerequisites

- You must be an administrator or have appropriate permissions on the repository
- The SSH key must be available (see Public Key section below)

## Deploy Key Information

**Repository:** `strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-`

**Key Title:** `FlameLang-Athena-Write`

**Key Type:** `ssh-ed25519` (ED25519 - modern, secure, fast)

**Public Key:**
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ8oRgDM4Tu1k0+c9fthOgvS8bQW5xiWIV9SN0sG2YkQ security@strategickhaos.ai
```

**Email:** `security@strategickhaos.ai`

## Setup Instructions

### Method 1: Via GitHub Web Interface (Recommended)

#### Step 1: Navigate to Repository Settings

Open your web browser and go to:
```
https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/settings/keys
```

Or navigate manually:
1. Go to the repository: `https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-`
2. Click on **Settings** (gear icon) in the top navigation bar
3. In the left sidebar, click **Deploy keys** under "Security"

#### Step 2: Add New Deploy Key

1. Click the **"Add deploy key"** button (green button in the top right)

2. Fill in the form:
   - **Title:** `FlameLang-Athena-Write`
   - **Key:** Paste the public key:
     ```
     ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ8oRgDM4Tu1k0+c9fthOgvS8bQW5xiWIV9SN0sG2YkQ security@strategickhaos.ai
     ```
   - **✅ Allow write access:** Check this box (IMPORTANT!)

3. Click **"Add key"** to save

#### Step 3: Verify Deploy Key

After adding the key, you should see it listed in the deploy keys section with:
- Title: `FlameLang-Athena-Write`
- Fingerprint: A unique fingerprint of the key
- Write access: ✅ (green checkmark indicating write access is enabled)

### Method 2: Via GitHub CLI (gh)

If you have GitHub CLI installed and authenticated:

```bash
# Add deploy key with write access
gh repo deploy-key add ~/.ssh/id_ed25519.pub \
  --title "FlameLang-Athena-Write" \
  --allow-write \
  --repo strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
```

## Verification

### Step 1: Test SSH Connection

```bash
ssh -T git@github.com
```

**Expected Output:**
```
Hi strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-! You've successfully authenticated, but GitHub does not provide shell access.
```

### Step 2: Test Push Operation

```bash
# Ensure you're on the correct branch
git checkout main

# Test push (will fail if there are no changes, but won't show permission error)
git push origin main
```

**Expected Output (if no changes):**
```
Everything up-to-date
```

**Expected Output (if there are changes):**
```
Enumerating objects: X, done.
Counting objects: 100% (X/X), done.
...
To github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
   abc1234..def5678  main -> main
```

**NOT Expected (indicates permission still denied):**
```
ERROR: Permission to strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git denied
fatal: Could not read from remote repository.
```

## Troubleshooting

### Issue: Cannot Add Deploy Key - "Key is already in use"

**Cause:** The SSH key is already added to a GitHub user account or another repository.

**Solutions:**

1. **Remove from user account** (if it's added there):
   - Go to: https://github.com/settings/keys
   - Find and delete the key from your user account
   - Then add it as a deploy key to the repository

2. **Use a different key** for this repository:
   ```bash
   # Generate a new SSH key specifically for this repo
   ssh-keygen -t ed25519 -C "security@strategickhaos.ai" -f ~/.ssh/flamelang_deploy_key
   
   # Use the new public key
   cat ~/.ssh/flamelang_deploy_key.pub
   ```
   
   Then configure SSH to use this key for this repo:
   ```bash
   # Add to ~/.ssh/config
   Host github.com-flamelang
       HostName github.com
       User git
       IdentityFile ~/.ssh/flamelang_deploy_key
       IdentitiesOnly yes
   
   # Update remote URL to use the custom host
   git remote set-url origin git@github.com-flamelang:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
   ```

### Issue: Deploy Key Added But Push Still Denied

**Possible Causes:**

1. **Write access not enabled:**
   - Go back to the deploy keys settings
   - Verify the checkbox "Allow write access" is checked (✅)
   - If not, you'll need to delete and re-add the key with write access enabled

2. **SSH agent not using the correct key:**
   ```bash
   # Check loaded keys
   ssh-add -l
   
   # If your key isn't listed, add it
   ssh-add ~/.ssh/id_ed25519  # or path to your private key
   
   # Test which key is being used
   ssh -vT git@github.com 2>&1 | grep "Offering public key"
   ```

3. **HTTPS still being used instead of SSH:**
   ```bash
   # Check remote URL
   git remote -v
   
   # Should show: git@github.com:strategickhaos-dao-llc/...
   # Not:         https://github.com/strategickhaos-dao-llc/...
   
   # If using HTTPS, change to SSH
   git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
   ```

### Issue: "Permission denied (publickey)"

**Cause:** Private key not available or not loaded in SSH agent.

**Solution:**

```bash
# Start SSH agent (if not running)
eval "$(ssh-agent -s)"

# Add private key
ssh-add ~/.ssh/id_ed25519  # Adjust path to your private key

# Verify key is loaded
ssh-add -l

# Test connection
ssh -T git@github.com
```

## Security Considerations

### Why Deploy Keys?

Deploy keys are preferred over user SSH keys for repositories because:

1. **Repository-specific:** Limited to a single repository
2. **Revocable:** Can be removed without affecting user access
3. **Auditable:** Clear tracking of which keys have access
4. **Automation-friendly:** Perfect for CI/CD pipelines

### Write Access Implications

Enabling write access on a deploy key allows:

- ✅ Pushing commits
- ✅ Creating/deleting branches
- ✅ Creating/deleting tags
- ❌ Does NOT allow administrative actions (changing settings, managing other keys, etc.)

### Best Practices

1. **Use separate keys for different purposes:**
   - Personal development: User SSH key
   - CI/CD automation: Deploy key
   - Server deployments: Deploy key (read-only if possible)

2. **Rotate keys periodically:**
   - Generate new keys every 6-12 months
   - Remove old keys after rotation

3. **Limit write access:**
   - Only enable write access when absolutely necessary
   - Consider read-only deploy keys for deployments that only need to pull code

4. **Protect private keys:**
   - Never commit private keys to repositories
   - Use appropriate file permissions (chmod 600)
   - Store securely (encrypted volumes, password-protected)

## Additional Resources

- [GitHub Deploy Keys Documentation](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/managing-deploy-keys)
- [GitHub SSH Key Types](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent)
- [SSH Key Best Practices](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/reviewing-your-ssh-keys)

## Quick Reference

```bash
# Direct link to add deploy key
https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/settings/keys/new

# Public key to add
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ8oRgDM4Tu1k0+c9fthOgvS8bQW5xiWIV9SN0sG2YkQ security@strategickhaos.ai

# Title
FlameLang-Athena-Write

# Required settings
✅ Allow write access

# Verification commands
ssh -T git@github.com
git push origin main
```
