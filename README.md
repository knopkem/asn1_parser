# ASN.1 Decoder Project

A collection of Rust-based tools for decoding and analyzing ASN.1 (Abstract Syntax Notation One) structures in PEM format.

Try the online version: [asn1_parser](https://knopkem.github.io/asn1_parser/)

## Projects

### 1. asn1_decoder (CLI Tool)
A command-line utility for decoding PEM-formatted ASN.1 structures.

**Features:**
- Decode certificates, keys, CSRs from PEM files
- Read from files or stdin
- Detailed hierarchical output to terminal
- Shows tags, lengths, types, and decoded values

**Usage:**
```bash
cd asn1_decoder
cargo build --release
./target/release/asn1_decoder certificate.pem
```

[View CLI Documentation →](./asn1_decoder)

### 2. asn1_web_decoder (Web Application)
A WebAssembly-powered web interface for decoding ASN.1 structures with an interactive tree view.

**Features:**
- Browser-based, no server required
- Interactive tree widget with expand/collapse
- Real-time decoding using WebAssembly
- Beautiful, responsive UI
- Color-coded badges for tag classes and types
- Sample data included for testing
- **Automatically deployed to GitHub Pages**

**Quick Start:**
```bash
cd asn1_web_decoder
./build.sh
cd www
./serve.sh
# Open http://localhost:8080
```

**GitHub Pages:**
The web application is automatically built and deployed to GitHub Pages via GitHub Actions. See `.github/workflows/deploy-pages.yml` for details.

[View Web App Documentation →](./asn1_web_decoder)

## What is ASN.1?

ASN.1 (Abstract Syntax Notation One) is a standard interface description language for defining data structures that can be serialized and deserialized in a cross-platform way. It's widely used in:

- **X.509 Certificates**: SSL/TLS certificates
- **Cryptographic Keys**: RSA, EC, DSA keys
- **Certificate Signing Requests (CSRs)**
- **Telecommunications**: Network protocols
- **Financial Systems**: EMV, payment cards
- **Aviation**: Air traffic control systems

### PEM Format

PEM (Privacy-Enhanced Mail) is a base64-encoded format for ASN.1 data, typically looking like:
```
-----BEGIN CERTIFICATE-----
MIIDXTCCAkWgAwIBAgIJAKVpbF9KLG5cMA0GCSqGSIb3DQEBCwUA...
-----END CERTIFICATE-----
```

### DER Encoding

DER (Distinguished Encoding Rules) is the binary encoding of ASN.1 structures. PEM is simply base64-encoded DER with header/footer markers.

## Technology Stack

### Rust Libraries
- **rasn**: ASN.1 codec (not directly used in current implementation)
- **pem**: PEM format parsing
- **wasm-bindgen**: Rust-to-JavaScript bindings
- **serde**: Serialization framework

### Web Technologies
- **WebAssembly**: Fast, secure browser execution
- **Vanilla JavaScript**: No frameworks, pure ES6+
- **CSS Grid/Flexbox**: Modern responsive layout

## Use Cases

### Certificate Analysis
Analyze SSL/TLS certificates to understand their structure, validity periods, subject/issuer information, and extensions.

### Security Auditing
Examine cryptographic keys and certificates for security compliance, key sizes, algorithms, and proper encoding.

### Development & Debugging
Debug certificate-related issues in applications by understanding the exact structure of certificates and keys.

### Education
Learn about X.509 certificates, ASN.1 encoding, and cryptographic structures through interactive visualization.

## Supported ASN.1 Types

Both tools support decoding:

- **BOOLEAN**: True/false values
- **INTEGER**: Numbers of arbitrary size
- **BIT STRING**: Binary data with padding
- **OCTET STRING**: Raw byte sequences
- **NULL**: Null values
- **OBJECT IDENTIFIER (OID)**: Hierarchical identifiers
- **UTF8String, PrintableString, IA5String**: Text data
- **UTCTime, GeneralizedTime**: Timestamps
- **SEQUENCE**: Ordered collections (constructed)
- **SET**: Unordered collections (constructed)
- **Context-specific tags**: Tagged elements
- **Application tags**: Application-defined types

## Example: Decoding a Certificate

### Using CLI Tool
```bash
# Generate a test certificate
openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem -days 365 -nodes \
  -subj "/C=US/ST=CA/L=SF/O=Test/CN=example.com"

# Decode it
./asn1_decoder/target/release/asn1_decoder cert.pem
```

### Using Web App
1. Open http://localhost:8080
2. Paste the certificate content
3. Click "Decode"
4. Explore the interactive tree

## Development

### CLI Tool
```bash
cd asn1_decoder
cargo build
cargo test
cargo run -- path/to/file.pem
```

### Web Application
```bash
cd asn1_web_decoder

# Build
./build.sh

# Develop (edit files, rebuild as needed)
cargo check
wasm-pack build --target web --out-dir www/pkg

# Test locally
cd www
./serve.sh
```

## Project Structure

```
asn1_decoder/
├── asn1_decoder/           # CLI tool
│   ├── src/
│   │   └── main.rs         # CLI implementation
│   ├── Cargo.toml
│   └── README.md
│
├── asn1_web_decoder/       # Web application
│   ├── src/
│   │   └── lib.rs          # Rust/WASM library
│   ├── www/
│   │   ├── index.html      # Web interface
│   │   ├── styles.css      # Styling
│   │   ├── app.js          # JavaScript app
│   │   └── pkg/            # Generated WASM (after build)
│   ├── Cargo.toml
│   ├── build.sh            # Build script
│   ├── README.md
│   ├── DEMO.md
│   └── DEPLOYMENT.md
│
└── README.md               # This file
```

## Common ASN.1 OIDs

Understanding Object Identifiers helps interpret certificates:

- `2.5.4.3` - Common Name (CN)
- `2.5.4.6` - Country (C)
- `2.5.4.7` - Locality (L)
- `2.5.4.8` - State/Province (ST)
- `2.5.4.10` - Organization (O)
- `2.5.4.11` - Organizational Unit (OU)
- `1.2.840.113549.1.1.1` - RSA Encryption
- `1.2.840.113549.1.1.11` - SHA256 with RSA
- `1.2.840.10045.2.1` - EC Public Key
- `2.5.29.15` - Key Usage extension
- `2.5.29.19` - Basic Constraints extension

## Browser Compatibility

The web application requires WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Performance

- **CLI Tool**: <10ms for typical certificates
- **Web App**: 
  - WASM module: ~62KB (optimized)
  - Decode time: <10ms
  - First load: ~100-200ms (WASM initialization)

## Contributing

Both projects welcome contributions:
- Bug fixes
- New ASN.1 type support
- UI improvements
- Documentation enhancements
- Performance optimizations

## License

MIT License - See individual project directories for details.

## Resources

- [ASN.1 Specification](https://www.itu.int/rec/T-REC-X.680)
- [X.509 Certificate Format](https://tools.ietf.org/html/rfc5280)
- [DER Encoding Rules](https://www.itu.int/rec/T-REC-X.690)
- [OpenSSL Documentation](https://www.openssl.org/docs/)
- [WebAssembly](https://webassembly.org/)

## Acknowledgments

- **rasn**: Rust ASN.1 library
- **pem**: PEM parsing library
- **wasm-bindgen**: WebAssembly tooling
- **OpenSSL**: For test data generation
