# VesselMirror

**Sovereign Page Cloning & Synthesis Engine - INV-079**

VesselMirror is a specialized tool for cloning web pages with cryptographic proof generation capabilities. It's designed for creating verifiable, sovereign copies of web content with optional asset inlining.

## Features

- 🔮 **Page Cloning**: Clone any web page to local storage
- 🖼️ **Asset Inlining**: Optionally inline all assets (images, CSS) as data URLs
- 🔐 **Cryptographic Proof**: Generate SHA256-based proofs of cloned content
- 📦 **JSON Export**: Export proofs in JSON format for on-chain or archival use
- 🛡️ **Secure**: Built with Rust for memory safety and reliability

## Installation

From the repository root:

```bash
cargo build --release -p vesselmirror
```

The binary will be available at `target/release/vessel`.

## Usage

### Basic page cloning

```bash
vessel --url https://example.com
```

### Clone with cryptographic proof

```bash
vessel --url https://example.com --proof
```

### Clone with asset inlining

```bash
vessel --url https://example.com --inline --proof
```

### Specify output directory

```bash
vessel --url https://example.com --output ./my-archive
```

## Command-Line Options

- `-u, --url <URL>`: URL to clone (required)
- `-o, --output <OUTPUT>`: Output directory (default: "output")
- `-p, --proof`: Generate cryptographic proof
- `-i, --inline`: Inline all assets (images, CSS, JS)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Output

VesselMirror creates the following files:

- `index.html`: The cloned web page
- `proof.json`: Cryptographic proof (when `--proof` flag is used)

## Proof Format

The proof.json file contains:

```json
{
  "url": "https://example.com",
  "timestamp": "2025-12-15T04:30:00Z",
  "content_hash": "sha256_hash_of_content",
  "proof_type": "vesselmirror_sovereign",
  "assets": [
    {
      "url": "https://example.com/image.png",
      "hash": "sha256_hash_of_asset",
      "mime_type": "image/png"
    }
  ]
}
```

## Security Considerations

- All network requests use TLS/HTTPS when available
- Content hashes use SHA256 for cryptographic verification
- No external code execution or command injection
- Path traversal protection through Rust's PathBuf

## License

MIT License - See repository root for details

## Author

Strategickhaos DAO LLC <security@strategickhaos.ai>
