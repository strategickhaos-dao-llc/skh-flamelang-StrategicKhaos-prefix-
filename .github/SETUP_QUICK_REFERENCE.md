# Quick Setup Reference

## Git Commit Signing - Quick Fix

If you're experiencing GPG signing issues blocking commits, use one of these solutions:

### Option A: SSH Signing (Recommended)

```bash
# Configure SSH signing
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519_flamelang
git config --global commit.gpgsign true

# Add allowed signers file
echo "$(git config --global user.email) namespaces=\"git\" $(cat ~/.ssh/id_ed25519_flamelang.pub)" >> ~/.ssh/allowed_signers
git config --global gpg.ssh.allowedSignersFile ~/.ssh/allowed_signers
```

**Note:** Replace `id_ed25519_flamelang` with your actual SSH key name.

### Option B: Disable GPG Signing (Temporary)

```bash
# Disable globally
git config --global commit.gpgsign false

# Or disable for this repository only
git config commit.gpgsign false
```

### Verify Configuration

```bash
git config --list | grep -E "(user\.|commit\.|gpg\.)"
```

---

## Claude CLI Installation

**Note:** These are example commands. Please refer to official Claude AI documentation for actual installation instructions.

### Rider IDE (PowerShell)

```powershell
# Example - verify from official documentation
irm https://claude.ai/install.ps1 | iex
```

### macOS/Linux (Bash/Zsh)

```bash
# Example - verify from official documentation
curl -fsSL https://claude.ai/install.sh | sh
```

### Usage

```bash
claude "explain this codebase"
claude commit
claude review
```

---

## Rust Build & Test

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Build
cargo build --verbose

# Test
cargo test --verbose
```

---

For detailed documentation, see [DEVELOPMENT.md](../DEVELOPMENT.md)
