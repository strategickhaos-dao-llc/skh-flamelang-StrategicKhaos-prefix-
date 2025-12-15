# FlameLang Development Setup Guide

This guide covers the development environment setup for FlameLang v2.0.0, including IDE integration, commit signing, and workflow tools.

## Table of Contents

- [Quick Start](#quick-start)
- [IDE Setup](#ide-setup)
  - [Rider IDE](#rider-ide)
  - [VS Code](#vs-code)
  - [GitHub Codespaces](#github-codespaces)
- [Claude CLI Integration](#claude-cli-integration)
  - [Rider IDE Installation](#rider-ide-installation)
  - [Usage Examples](#usage-examples)
- [Git Commit Signing](#git-commit-signing)
  - [SSH Signing (Recommended)](#ssh-signing-recommended)
  - [GPG Signing](#gpg-signing)
  - [Disabling Signing](#disabling-signing)
- [Workflow Assessment](#workflow-assessment)
- [Development Workflow](#development-workflow)

---

## Quick Start

### Prerequisites

- **Rust** (edition 2021 or later) - see [rust-toolchain.toml](rust-toolchain.toml)
- **Git** configured with your GitHub credentials
- **IDE**: Rider, VS Code, or GitHub Codespaces

### Clone and Build

```bash
git clone https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git
cd skh-flamelang-StrategicKhaos-prefix-
cargo build --verbose
cargo test --verbose
```

---

## IDE Setup

### Rider IDE

**JetBrains Rider** provides excellent Rust support and terminal integration.

1. **Install Rider** from [jetbrains.com/rider](https://www.jetbrains.com/rider/)
2. **Install Rust Plugin**: Settings → Plugins → Search "Rust" → Install
3. **Configure Rust Toolchain**: Settings → Languages & Frameworks → Rust → Set toolchain location

**Terminal Access:**
- Open integrated terminal: `Alt+F12` or View → Tool Windows → Terminal
- Rider's terminal is a full PowerShell instance on Windows or Bash/Zsh on macOS/Linux

### VS Code

The repository includes `.devcontainer/devcontainer.json` with pre-configured VS Code settings:

- **Extensions**: rust-analyzer, LLDB debugger, Crates, Error Lens, GitLens
- **Format on Save**: Enabled
- **Clippy**: Integrated linting

### GitHub Codespaces

Click "Code" → "Codespaces" → "Create codespace on main" for a cloud-based development environment with all dependencies pre-installed.

---

## Claude CLI Integration

The Claude CLI provides AI-powered code assistance directly from your terminal.

### Rider IDE Installation

**Note:** The Claude CLI installation method varies based on the actual tool you're using. The commands below are examples based on the problem statement. Please refer to the official Claude AI documentation for accurate installation instructions.

Open Rider's integrated terminal (`Alt+F12`) and run the appropriate command:

**PowerShell (Windows):**
```powershell
# Example command - verify actual command from official documentation
irm https://claude.ai/install.ps1 | iex
```

**Bash/Zsh (macOS/Linux):**
```bash
# Example command - verify actual command from official documentation
curl -fsSL https://claude.ai/install.sh | sh
```

Visit the official Claude AI website for the most up-to-date installation instructions.

### Usage Examples

After installation, invoke Claude directly from Rider's terminal:

```bash
# Get codebase explanation
claude "explain this codebase"

# Generate AI-powered commit messages
claude commit

# Request code review
claude review

# Ask specific questions
claude "how does the 5-layer pipeline work?"
```

### Rider External Tools Integration

For hotkey access, add Claude as an External Tool:

1. **Settings** → **Tools** → **External Tools** → **Add**
2. **Name**: Claude Commit
3. **Program**: `claude` (or use the full path from `which claude` on Unix or `where claude` on Windows)
4. **Arguments**: `commit`
5. **Working Directory**: `$ProjectFileDir$`
6. Assign a hotkey (e.g., `Ctrl+Shift+C`)

Repeat for other commands like `review`, `explain`, etc.

---

## Git Commit Signing

Commit signing verifies the authenticity of your commits. FlameLang development recommends signed commits.

### SSH Signing (Recommended)

**Modern, streamlined approach using your existing SSH key.**

#### Step 1: Check for SSH Keys

```bash
ls -la ~/.ssh/
```

Look for keys like:
- `id_ed25519` / `id_ed25519.pub` (recommended)
- `id_rsa` / `id_rsa.pub` (legacy)
- `id_ed25519_flamelang` / `id_ed25519_flamelang.pub` (project-specific)

#### Step 2: Generate SSH Key (if needed)

```bash
ssh-keygen -t ed25519 -C "your_email@example.com" -f ~/.ssh/id_ed25519_flamelang
```

#### Step 3: Add SSH Key to GitHub

```bash
# Copy public key to clipboard
cat ~/.ssh/id_ed25519_flamelang.pub
```

Go to [GitHub SSH Keys Settings](https://github.com/settings/keys):
1. Click "New SSH Key"
2. **Title**: `FlameLang Dev - <Machine Name>`
3. **Key Type**: "Signing Key" (for commit signing) or "Authentication Key"
4. Paste the public key
5. Click "Add SSH Key"

#### Step 4: Configure Git for SSH Signing

```bash
# Use SSH format for signing
git config --global gpg.format ssh

# Set signing key (replace path if using different key)
git config --global user.signingkey ~/.ssh/id_ed25519_flamelang

# Enable commit signing
git config --global commit.gpgsign true

# Verify configuration
git config --list | grep -E "(user\.|commit\.|gpg\.)"
```

#### Step 5: Configure Git to Show SSH Key

Create or edit `~/.ssh/allowed_signers`:

```bash
echo "$(git config --global user.email) namespaces=\"git\" $(cat ~/.ssh/id_ed25519_flamelang.pub)" >> ~/.ssh/allowed_signers
```

Tell Git about this file:

```bash
git config --global gpg.ssh.allowedSignersFile ~/.ssh/allowed_signers
```

#### Step 6: Test Signing

```bash
# Create a test commit
git commit --allow-empty -m "Test signed commit"

# Verify signature
git log --show-signature -1
```

You should see "Good signature" in the output.

---

### GPG Signing

**Traditional approach using GPG keys.**

#### Step 1: Install GPG

**macOS:**
```bash
brew install gnupg
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get install gnupg
```

**Windows:**
Download from [gnupg.org](https://gnupg.org/download/)

#### Step 2: Generate GPG Key

```bash
gpg --full-generate-key
```

- Key type: **(1) RSA and RSA**
- Key size: **4096 bits**
- Expiration: **2y** (2 years)
- Real name: Your name
- Email: **Must match your GitHub email**
- Passphrase: Use a strong passphrase (store it securely!)

#### Step 3: List and Copy GPG Key

```bash
# List keys
gpg --list-secret-keys --keyid-format=long

# Output example:
# sec   rsa4096/ABC123DEF456 2025-01-01 [SC] [expires: 2027-01-01]

# Export public key (replace ABC123DEF456 with your key ID)
gpg --armor --export ABC123DEF456
```

#### Step 4: Add GPG Key to GitHub

Copy the output from the previous step (including `-----BEGIN PGP PUBLIC KEY BLOCK-----` and `-----END PGP PUBLIC KEY BLOCK-----`).

Go to [GitHub GPG Keys Settings](https://github.com/settings/keys):
1. Click "New GPG Key"
2. Paste the public key
3. Click "Add GPG Key"

#### Step 5: Configure Git for GPG Signing

```bash
# Set GPG signing key (replace with your key ID)
git config --global user.signingkey ABC123DEF456

# Enable commit signing
git config --global commit.gpgsign true

# Tell Git which GPG program to use
git config --global gpg.program gpg
```

#### Step 6: Troubleshooting GPG Passphrase

If you encounter passphrase prompt issues:

**Option A: Use GPG Agent (Recommended)**
```bash
# Start gpg-agent
eval $(gpg-agent --daemon)

# Cache passphrase (enter when prompted)
echo "test" | gpg --sign --armor > /dev/null
```

**Option B: Configure Passphrase Caching**

Edit `~/.gnupg/gpg-agent.conf`:
```
default-cache-ttl 3600
max-cache-ttl 7200
```

Restart gpg-agent:
```bash
gpgconf --kill gpg-agent
gpg-agent --daemon
```

---

### Disabling Signing

If commit signing is causing issues and you want to disable it temporarily:

```bash
# Disable globally
git config --global commit.gpgsign false

# Or disable for this repository only
git config commit.gpgsign false
```

**Note:** Some projects require signed commits. Check with project maintainers before disabling.

---

## Workflow Assessment

### FlameLang Pipeline Status

| Phase | Status | Evidence |
|-------|--------|----------|
| **Auth** | ✅ | SSH key generated, `gh auth login` complete, logged in as `strategickhaos-dao-llc` |
| **Signing** | ⚠️ | GPG signing may fail (passphrase issue) — use SSH signing or disable for now |
| **Compilation** | ✅ | FlameLang 5-layer pipeline executing (`English→Hebrew→Unicode→Wave→DNA→LLVM`) |
| **CI/CD** | ✅ | GitHub Actions workflows present, Codespaces active |
| **Multi-entity** | ✅ | DAO LLC + ValorYield PBC orgs on GitHub |

### Current Workflow

1. **Development** happens in feature branches (`copilot/*`, `feature/*`)
2. **CI/CD** runs via GitHub Actions (`.github/workflows/flamelang-ci.yml`)
3. **Testing** includes unit tests, integration tests, and benchmarks
4. **Commit signing** is recommended for all commits (SSH preferred)
5. **Code review** via GitHub Pull Requests

---

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

```rust
// Edit source files in src/
// Add tests in tests/
```

### 3. Run Tests Locally

```bash
# Format code
cargo fmt --all

# Lint with Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build
cargo build --verbose

# Run tests
cargo test --verbose
```

### 4. Commit Changes

```bash
git add .
git commit -m "feat: add new layer transformation logic"
```

**Commit Message Format:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Go to GitHub and create a Pull Request from your branch to `main`.

### 6. CI/CD Validation

GitHub Actions will automatically:
- Check code formatting
- Run Clippy linting
- Build the project
- Execute all tests

---

## Troubleshooting

### Issue: GPG Signing Fails with "gpg: signing failed: Inappropriate ioctl for device"

**Solution:**
```bash
export GPG_TTY=$(tty)
echo 'export GPG_TTY=$(tty)' >> ~/.bashrc  # or ~/.zshrc
```

### Issue: SSH Signing Shows "Invalid Signature"

**Solution:** Ensure you've added the SSH key as a **Signing Key** on GitHub, not just an Authentication Key.

### Issue: Cargo Build Fails

**Solution:**
```bash
# Update Rust toolchain
rustup update

# Clean build artifacts
cargo clean

# Rebuild
cargo build
```

### Issue: Claude CLI Not Found

**Solution:** First, verify the actual installation location of the Claude CLI binary. Then ensure it's in your PATH:

**Find the Claude binary location:**
```bash
# On macOS/Linux
which claude

# On Windows (PowerShell)
where.exe claude
```

**Add to PATH (if needed):**

**Windows (PowerShell):**
```powershell
# Replace <install-path> with the actual directory containing claude.exe
$env:PATH += ";<install-path>"
```

**macOS/Linux (Bash):**
```bash
# Replace <install-path> with the actual directory containing the claude binary
export PATH="<install-path>:$PATH"
echo 'export PATH="<install-path>:$PATH"' >> ~/.bashrc
```

---

## Additional Resources

- **FlameLang Spec**: [docs/spec/](docs/spec/) (coming soon)
- **API Reference**: [docs/api/](docs/api/) (coming soon)
- **Tutorials**: [docs/tutorials/](docs/tutorials/) (coming soon)
- **GitHub Issues**: [Issues](https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/issues)
- **Discussions**: [Discussions](https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/discussions)

---

## Contact

- **Email**: security@strategickhaos.ai
- **Organization**: Strategickhaos DAO LLC
- **License**: MIT (see [LICENSE](LICENSE))

---

🔥 **FlameLang v2.0.0** | Ratio Ex Nihilo | 5-Layer Biological Compilation Pipeline
