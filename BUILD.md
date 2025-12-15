# Build Instructions

## Prerequisites

### LLVM Installation

FlameLang requires LLVM with static libraries for the codegen crate. The recommended version is LLVM 17.

#### Ubuntu/Debian

```bash
# Install LLVM 17 with all components
sudo apt-get install llvm-17 llvm-17-dev libpolly-17-dev

# Set environment variable
export LLVM_SYS_170_PREFIX=/usr/lib/llvm-17
```

#### macOS

```bash
brew install llvm@17
export LLVM_SYS_170_PREFIX=$(brew --prefix llvm@17)
```

#### Windows

Download and install LLVM 17 from the official LLVM releases page.

## Building

### Full Build (requires LLVM)

```bash
cargo build --release
```

### Partial Build (without codegen)

If you don't have LLVM installed or want to work on the frontend only:

```bash
# Build and test frontend crates only
cargo build -p flamelang-lexer -p flamelang-parser -p flamelang-ast -p flamelang-hir -p flamelang-mir
cargo test -p flamelang-lexer -p flamelang-parser -p flamelang-ast -p flamelang-hir -p flamelang-mir
```

## Testing

```bash
# Test all crates (requires LLVM)
cargo test

# Test frontend only (no LLVM required)
cargo test -p flamelang-lexer -p flamelang-parser -p flamelang-ast -p flamelang-hir -p flamelang-mir
```

## Common Issues

### Missing Polly Library

If you encounter an error like:
```
error: could not find native static library `Polly`
```

This means LLVM was built without the Polly static library. Solutions:

1. Install LLVM from your package manager with all development packages
2. Build LLVM from source with static libraries enabled
3. Work with frontend crates only (see Partial Build above)

### LLVM Version Mismatch

Ensure your LLVM version matches the one specified in `Cargo.toml`. Currently using LLVM 17.

Set the appropriate environment variable:
```bash
export LLVM_SYS_170_PREFIX=/path/to/llvm-17
```
