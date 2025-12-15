# 🔐 SSH Setup Guide for FlameLang Repository

This guide helps you set up SSH authentication for the FlameLang repository to avoid HTTPS authentication issues.

## ⚠️ Important: Correct Repository Name

The repository name includes a **trailing hyphen**: `skh-flamelang-StrategicKhaos-prefix-`

**Correct SSH URL:**
```
git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
```

**Incorrect URL (missing trailing hyphen):**
```
git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix.git  ❌
```

---

## Windows Setup (PowerShell)

### 1. Generate SSH Key

```powershell
# Generate a new ED25519 SSH key
# Replace "your-email@example.com" with your actual email address
ssh-keygen -t ed25519 -C "your-email@example.com" -f "$env:USERPROFILE\.ssh\id_ed25519_flamelang"

# Enter a secure passphrase (recommended) or press Enter for no passphrase (less secure)
# Using a passphrase adds an extra layer of security
```

### 2. Start SSH Agent and Add Key

```powershell
# Set Git to use SSH
$env:GIT_SSH_COMMAND="ssh"

# Start the SSH agent service
Start-Service ssh-agent

# Add your SSH key to the agent
ssh-add "$env:USERPROFILE\.ssh\id_ed25519_flamelang"
```

### 3. Copy Your Public Key

```powershell
# Display your public key
Write-Host "`n🔥 COPY THE BLOCK BELOW THIS LINE 🔥`n" -ForegroundColor Red
Get-Content "$env:USERPROFILE\.ssh\id_ed25519_flamelang.pub"
Write-Host "`n🔥 END OF KEY 🔥`n" -ForegroundColor Red
```

### 4. Add Key to GitHub

1. Go to GitHub → Settings → SSH and GPG keys
2. Click "New SSH key"
3. Title: `FlameLang Development Key`
4. Paste your public key
5. Click "Add SSH key"

### 5. Test SSH Connection

```powershell
# Test GitHub SSH connection
ssh -T git@github.com

# Expected output:
# Hi strategickhaos-dao-llc! You've successfully authenticated...
```

### 6. Configure Repository Remote (IMPORTANT!)

```powershell
# Navigate to your repository
cd path\to\skh-flamelang-StrategicKhaos-prefix-

# Update remote URL with CORRECT repository name (note the trailing hyphen!)
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git

# Verify the remote URL
git remote -v
```

### 7. Test Push

```powershell
# Create a test commit
git add .
git commit -m "Test SSH authentication"
git push origin main
```

---

## Linux/macOS Setup

### 1. Generate SSH Key

```bash
# Generate a new ED25519 SSH key
# Replace "your-email@example.com" with your actual email address
ssh-keygen -t ed25519 -C "your-email@example.com" -f ~/.ssh/id_ed25519_flamelang

# Enter a secure passphrase (recommended) or press Enter for no passphrase (less secure)
# Using a passphrase adds an extra layer of security
```

### 2. Start SSH Agent and Add Key

```bash
# Start the SSH agent
eval "$(ssh-agent -s)"

# Add your SSH key to the agent
ssh-add ~/.ssh/id_ed25519_flamelang
```

### 3. Copy Your Public Key

```bash
# Display your public key
echo -e "\n🔥 COPY THE BLOCK BELOW THIS LINE 🔥\n"
cat ~/.ssh/id_ed25519_flamelang.pub
echo -e "\n🔥 END OF KEY 🔥\n"
```

### 4. Add Key to GitHub

1. Go to GitHub → Settings → SSH and GPG keys
2. Click "New SSH key"
3. Title: `FlameLang Development Key`
4. Paste your public key
5. Click "Add SSH key"

### 5. Test SSH Connection

```bash
# Test GitHub SSH connection
ssh -T git@github.com

# Expected output:
# Hi strategickhaos-dao-llc! You've successfully authenticated...
```

### 6. Configure Repository Remote (IMPORTANT!)

```bash
# Navigate to your repository
cd /path/to/skh-flamelang-StrategicKhaos-prefix-

# Update remote URL with CORRECT repository name (note the trailing hyphen!)
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git

# Verify the remote URL
git remote -v
```

### 7. Test Push

```bash
# Create a test commit
git add .
git commit -m "Test SSH authentication"
git push origin main
```

---

## 🔧 Troubleshooting

### "ERROR: Repository not found"

This error typically occurs when the repository URL is incorrect.

**Most Common Cause:** Missing the trailing hyphen in the repository name!

```bash
# Check your current remote URL
git remote -v

# If it shows:
# origin  git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix.git
#                                                                                    ^ MISSING HYPHEN!

# Fix it with the correct URL (note the trailing hyphen):
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
#                                                                                                      ^ ADDED HYPHEN!

# Verify the fix
git remote -v
```

### "Permission denied (publickey)"

1. Ensure your SSH key is added to the ssh-agent:
   ```bash
   ssh-add -l  # List loaded keys
   ```

2. Verify SSH connection to GitHub:
   ```bash
   ssh -T git@github.com
   ```

3. Check that you added the public key (`.pub` file) to GitHub, not the private key

### "Could not read from remote repository"

1. Verify you have access to the repository
2. Ensure the SSH key is added to your GitHub account
3. Check that the repository URL is correct (with trailing hyphen!)

---

## 📋 Quick Reference

**Repository Name:** `skh-flamelang-StrategicKhaos-prefix-` ⚠️ **Note the trailing hyphen!**

**Correct SSH URL:**
```
git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
```

**Correct HTTPS URL:**
```
https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
```

---

## 🔒 Security Best Practices

1. **Use a passphrase** for your SSH key in production environments
2. **Never share** your private key (`id_ed25519_flamelang` without `.pub`)
3. **Only share** your public key (`id_ed25519_flamelang.pub`)
4. **Rotate keys** periodically (every 6-12 months)
5. **Delete unused keys** from GitHub Settings

---

For more information, see [GitHub's SSH Documentation](https://docs.github.com/en/authentication/connecting-to-github-with-ssh).
