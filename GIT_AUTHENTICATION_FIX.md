# Git Authentication and Permission Fix Guide

## Problem Summary

Two critical issues have been identified:

1. **Permission Mismatch** — SSH authenticated as `strategickhaos-dao-llc` but push is being denied for `Me10101-01`. Windows credential manager is overriding SSH with a different account.

2. **Merge Conflicts** — 16 files have conflicts from a failed rebase, resulting in a detached HEAD state.

## Resolution Steps (Execute in Order)

### Step 1: Abort the Broken Rebase

If you're in the middle of a failed rebase with merge conflicts:

```powershell
git rebase --abort
```

This will return your repository to the state before the rebase attempt.

### Step 2: Check Credential Configuration

Identify which credential helper is interfering:

```powershell
git config --global credential.helper
```

Common outputs:
- `manager-core` or `manager` — Windows Credential Manager
- `osxkeychain` — macOS Keychain
- `cache` — Git's built-in credential cache

### Step 3: Clear Cached GitHub Credentials

Remove any stored credentials that might be causing conflicts:

```powershell
# Windows (PowerShell)
git credential reject https://github.com
```

After running this command, you'll need to provide input. Type the following and press Enter after each line:

```
protocol=https
host=github.com

```

(The last line should be blank — just press Enter twice)

**Alternative for Windows:** Manually remove credentials from Windows Credential Manager:
1. Open Control Panel → Credential Manager
2. Select "Windows Credentials"
3. Find and remove any GitHub-related credentials (look for `git:https://github.com`)

### Step 4: Force SSH Usage (Disable HTTPS Credential Override)

Configure Git to always use SSH instead of HTTPS:

```powershell
git config --global url."git@github.com:".insteadOf "https://github.com/"
```

This ensures that even if a URL is specified with `https://`, Git will automatically convert it to use SSH.

### Step 5: Verify SSH Authentication

Test your SSH connection to GitHub:

```powershell
ssh -T git@github.com
```

**Expected successful output:**
```
Hi strategickhaos-dao-llc! You've successfully authenticated, but GitHub does not provide shell access.
```

**If authentication fails:**
- Ensure your SSH key is added to your SSH agent
- Verify the SSH key is added to your GitHub account or as a deploy key

### Step 6: Verify Git User Configuration

Check your current Git identity:

```powershell
git config user.name
git config user.email
```

### Step 7: Set Organization Identity

If the user configuration doesn't match the organization account:

```powershell
# Set global configuration (applies to all repos)
git config --global user.name "strategickhaos-dao-llc"
git config --global user.email "security@strategickhaos.ai"

# OR set local configuration (only for this repo)
git config user.name "strategickhaos-dao-llc"
git config user.email "security@strategickhaos.ai"
```

### Step 8: Verify Remote URL Configuration

Ensure the remote is using SSH:

```powershell
git remote -v
```

**If using HTTPS (`https://github.com/...`)**, update to SSH:

```powershell
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
```

### Step 9: Push Changes

Now attempt to push:

```powershell
git checkout main
git push origin main
```

**If you need to force push** (⚠️ WARNING: This overwrites remote history):

```powershell
git push origin main --force
```

## Deploy Key Setup (If Still Denied)

If push access is still denied after following all steps above, the SSH key needs to be added as a **deploy key** with write access.

### Adding Deploy Key via GitHub Web UI

1. **Navigate to repository settings:**
   ```
   https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/settings/keys
   ```

2. **Click "Add deploy key"**

3. **Configure the deploy key:**
   - **Title:** `FlameLang-Athena-Write`
   - **Key:** 
     ```
     ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ8oRgDM4Tu1k0+c9fthOgvS8bQW5xiWIV9SN0sG2YkQ security@strategickhaos.ai
     ```
   - ✅ **Check "Allow write access"**

4. **Click "Add key"** to save

5. **Retry push operation**

### Verifying Deploy Key Access

After adding the deploy key:

```powershell
# Test SSH connection again
ssh -T git@github.com

# Attempt push
git push origin main
```

## Troubleshooting

### Issue: "Permission denied (publickey)"

**Causes:**
- SSH key not loaded in SSH agent
- SSH key not added to GitHub account/repo
- SSH agent not running

**Solutions:**

1. **Start SSH agent (Windows PowerShell):**
   ```powershell
   Start-Service ssh-agent
   Set-Service -Name ssh-agent -StartupType Automatic
   ```

2. **Add SSH key to agent:**
   ```powershell
   ssh-add ~/.ssh/id_ed25519
   # Or wherever your private key is located
   ```

3. **Verify key is loaded:**
   ```powershell
   ssh-add -l
   ```

### Issue: "Authentication failed" with HTTPS

**Solution:** This indicates credential manager is still interfering. Repeat Steps 3-4 above.

### Issue: "You do not have permission to push to this repository"

**Causes:**
- User doesn't have write access to the repository
- Using wrong account credentials

**Solutions:**
1. Verify you're a collaborator on the repository
2. Use deploy key with write access (see Deploy Key Setup section)
3. Ensure SSH key is associated with correct GitHub account

### Issue: Detached HEAD state

If you see "HEAD detached at [commit]":

```powershell
# Return to main branch
git checkout main

# If you had uncommitted changes, create a new branch first
git checkout -b recovery-branch
git checkout main
git merge recovery-branch
```

## Prevention

To avoid these issues in the future:

1. **Use SSH consistently** — Configure all Git operations to use SSH
2. **Avoid HTTPS cloning** — Always clone repositories using SSH URLs
3. **Clear credentials after account changes** — When switching GitHub accounts, clear credential manager
4. **Use deploy keys for automation** — For CI/CD and automated systems, use deploy keys instead of user credentials

## Additional Resources

- [GitHub SSH Key Documentation](https://docs.github.com/en/authentication/connecting-to-github-with-ssh)
- [GitHub Deploy Keys Documentation](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/managing-deploy-keys)
- [Git Credential Storage](https://git-scm.com/book/en/v2/Git-Tools-Credential-Storage)

## Quick Reference

```powershell
# Emergency reset sequence (use with caution)
git rebase --abort                                                    # Abort failed rebase
git config --global url."git@github.com:".insteadOf "https://github.com/"  # Force SSH
ssh -T git@github.com                                                 # Test SSH
git config user.name "strategickhaos-dao-llc"                        # Set username
git config user.email "security@strategickhaos.ai"                   # Set email
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git  # Set SSH remote
git push origin main --force                                          # Force push (if needed)
```
