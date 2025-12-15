# Git Authentication Quick Reference

## Common Scenarios

### Scenario 1: Permission Denied When Pushing

**Symptoms:**
```
ERROR: Permission to strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git denied
fatal: Could not read from remote repository.
```

**Quick Fix:**
```bash
# Verify SSH works
ssh -T git@github.com

# Configure SSH over HTTPS
git config --global url."git@github.com:".insteadOf "https://github.com/"

# Update remote URL
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git

# Try push again
git push origin main
```

**If still denied:** Add deploy key → See [DEPLOY_KEY_SETUP.md](DEPLOY_KEY_SETUP.md)

---

### Scenario 2: Wrong User Authentication

**Symptoms:**
```
remote: Permission denied to Me10101-01.
```

**Quick Fix:**
```bash
# Clear cached credentials
echo -e "protocol=https\nhost=github.com\n" | git credential reject

# Set correct user identity
git config user.name "strategickhaos-dao-llc"
git config user.email "security@strategickhaos.ai"

# Verify SSH identity
ssh -T git@github.com
```

---

### Scenario 3: Merge Conflicts from Failed Rebase

**Symptoms:**
```
You are currently rebasing branch 'main' on 'abc1234'.
(fix conflicts and run "git rebase --continue")
```

**Quick Fix:**
```bash
# Abort the rebase
git rebase --abort

# Verify clean state
git status

# If needed, force push (CAUTION)
git push origin main --force
```

---

### Scenario 4: Detached HEAD State

**Symptoms:**
```
HEAD detached at abc1234
```

**Quick Fix:**
```bash
# Create a recovery branch (if you have uncommitted changes)
git checkout -b recovery-branch

# Or just return to main
git checkout main
```

---

### Scenario 5: Windows Credential Manager Interference

**Symptoms:**
- SSH test succeeds but push fails
- Different username shown in error messages

**Quick Fix (Windows):**
```powershell
# Option 1: Clear Git credentials
git credential reject https://github.com
# Then press Enter twice

# Option 2: Manual removal
# 1. Open Control Panel → Credential Manager
# 2. Windows Credentials → Find "git:https://github.com"
# 3. Remove/Edit the credential

# Force SSH usage
git config --global url."git@github.com:".insteadOf "https://github.com/"
```

---

## Automated Solutions

### Linux/macOS
```bash
# Run automated fix script
chmod +x fix-git-auth.sh
./fix-git-auth.sh

# With options
./fix-git-auth.sh --abort-rebase --force-push
```

### Windows PowerShell
```powershell
# Run automated fix script
.\fix-git-auth.ps1

# With options
.\fix-git-auth.ps1 -AbortRebase -ForcePush
```

---

## Verification Commands

```bash
# Check SSH authentication
ssh -T git@github.com
# Expected: "Hi strategickhaos-dao-llc! You've successfully authenticated..."

# Check Git user config
git config user.name
git config user.email
# Expected: strategickhaos-dao-llc, security@strategickhaos.ai

# Check remote URL
git remote -v
# Expected: git@github.com:strategickhaos-dao-llc/...

# Check current branch and status
git status
# Expected: On branch main, nothing to commit, working tree clean

# Check credential helper
git config --global credential.helper
# For SSH-only: should be empty or not interfering
```

---

## Emergency Commands

```bash
# Nuclear option: Force overwrite remote with local
git checkout main
git push origin main --force

# Completely reset to remote state
git fetch origin
git reset --hard origin/main

# Abandon all local changes
git checkout -- .
git clean -fd
```

⚠️ **WARNING:** Force operations can cause data loss. Use with caution!

---

## Deploy Key Quick Add

1. **Get the public key:**
   ```
   ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ8oRgDM4Tu1k0+c9fthOgvS8bQW5xiWIV9SN0sG2YkQ security@strategickhaos.ai
   ```

2. **Add via GitHub:**
   - URL: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/settings/keys
   - Click "Add deploy key"
   - Title: `FlameLang-Athena-Write`
   - Paste key
   - ✅ Check "Allow write access"
   - Click "Add key"

3. **Verify:**
   ```bash
   ssh -T git@github.com
   git push origin main
   ```

---

## Decision Tree

```
Can't push to repository?
│
├─ SSH test fails?
│  ├─ Yes → Add SSH key or deploy key
│  └─ No → Continue
│
├─ Using HTTPS URL?
│  ├─ Yes → Change to SSH URL
│  └─ No → Continue
│
├─ Wrong user in error?
│  ├─ Yes → Clear credentials + Set correct user
│  └─ No → Continue
│
└─ Still denied?
   └─ Add deploy key with write access
```

---

## Detailed Documentation

- **[GIT_AUTHENTICATION_FIX.md](GIT_AUTHENTICATION_FIX.md)** - Complete troubleshooting guide
- **[DEPLOY_KEY_SETUP.md](DEPLOY_KEY_SETUP.md)** - Deploy key setup instructions
- **[README.md](README.md)** - Project overview and links

---

## Support

For additional assistance:
- Review the detailed guides in the documentation
- Run the automated fix scripts
- Check GitHub's SSH documentation: https://docs.github.com/en/authentication/connecting-to-github-with-ssh

---

**Repository:** `strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-`  
**Organization:** StrategicKhaos DAO LLC  
**Contact:** security@strategickhaos.ai
