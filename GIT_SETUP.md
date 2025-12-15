# Git Remote Setup Guide

## Repository Name Important Note

⚠️ **The repository name includes a trailing hyphen:** `skh-flamelang-StrategicKhaos-prefix-`

This is intentional and must be included in all Git remote URLs.

## Correct Remote URLs

### SSH (Recommended)
```bash
git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
```

### HTTPS
```bash
https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
```

## Verifying Your Setup

### 1. Check GitHub SSH Authentication
```bash
ssh -T git@github.com
```

Expected output:
```
Hi strategickhaos-dao-llc! You've successfully authenticated, but GitHub does not provide shell access.
```

### 2. Check Current Remote Configuration
```bash
git remote -v
```

### 3. Verify Remote Connection
```bash
git ls-remote
```

This should list all branches and pull requests without errors.

## Troubleshooting

### Issue: Missing Trailing Hyphen

If your remote URL is missing the trailing hyphen, you'll encounter connection issues.

**Symptoms:**
- Repository not found errors
- Push/pull failures
- Authentication issues (even when SSH works)

**Fix:**
```bash
# Update the remote URL to include the trailing hyphen
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git

# Verify the change
git remote -v

# Test the connection
git ls-remote
```

### Issue: HTTPS vs SSH

If you prefer SSH over HTTPS (recommended for frequent contributors):

```bash
# Switch from HTTPS to SSH
git remote set-url origin git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
```

If you prefer HTTPS over SSH:

```bash
# Switch from SSH to HTTPS
git remote set-url origin https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
```

## Clone Command

When cloning this repository, use one of these commands:

### SSH (Recommended)
```bash
git clone git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
```

### HTTPS
```bash
git clone https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-
```

## Quick Reference

| Component | Value |
|-----------|-------|
| Organization | `strategickhaos-dao-llc` |
| Repository | `skh-flamelang-StrategicKhaos-prefix-` ⚠️ (note trailing hyphen) |
| SSH URL | `git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git` |
| HTTPS URL | `https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-` |

---

**Remember:** The trailing hyphen in the repository name is **required** and must be included in all Git operations.
