# ASN.1 Decoder Project

A Rust-based WebAssembly web application for decoding and analyzing ASN.1 (Abstract Syntax Notation One) structures in PEM format.

Try the online version: [ASN1 PEM Decoder](https://knopkem.github.io/asn1_parser/)

## ASN.1 Web Decoder

A modern React + Material-UI web application powered by WebAssembly for decoding ASN.1 structures with an interactive tree view and hex viewer.

**Features:**
- 🚀 **Browser-based**: No server required, runs entirely in your browser
- 🎨 **Modern React UI**: Built with React 18 and Material-UI components
- 🔍 **Interactive Tree View**: Expand/collapse nodes to explore ASN.1 structures
- 📊 **Hex Viewer**: Color-coded hex dump with byte offset tracking
- ⚡ **Real-time Decoding**: Instant results using WebAssembly
- 🎯 **Full-Screen Layout**: Optimized split-panel interface
- 💾 **Sample Data**: Built-in example certificates for testing
- 🌐 **Automatically Deployed**: GitHub Pages deployment via GitHub Actions
- 🔒 **Privacy-First**: All processing happens in your browser

**Quick Start:**
```bash
# Build the WebAssembly module
cd asn1_web_decoder
./build.sh

# Install dependencies and start dev server
cd www
npm install
npm run dev
# Open http://localhost:8080
```

**Production Build:**
```bash
cd asn1_web_decoder/www
npm run build
# Output in www/dist/
```

**GitHub Pages:**
The web application is automatically built and deployed to GitHub Pages via GitHub Actions when you push to the main branch. See `.github/workflows/deploy-pages.yml` for details.

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
- **React 18**: Modern component-based UI framework
- **Material-UI (MUI) 5**: Professional design system
- **Vite 5**: Fast build tool and dev server
- **WebAssembly**: Fast, secure browser execution
- **Emotion**: CSS-in-JS styling

## Use Cases

### Certificate Analysis
Analyze SSL/TLS certificates to understand their structure, validity periods, subject/issuer information, and extensions.

### Security Auditing
Examine cryptographic keys and certificates for security compliance, key sizes, algorithms, and proper encoding.

### Development & Debugging
Debug certificate-related issues in applications by understanding the exact structure of certificates and keys.

### Education
Learn about X.509 certificates, ASN.1 encoding, and cryptographic structures through interactive visualization with real-time hex view.

## Supported ASN.1 Types

The web application supports decoding:

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

### Using Web App
1. Open the deployed site or run locally: `cd asn1_web_decoder/www && npm run dev`
2. Click the **floating action button** (bottom-right) to open input dialog
3. Paste your certificate content (or click "Load Sample")
4. Click "Decode"
5. Explore the interactive tree view on the left panel
6. View the color-coded hex dump on the right panel
7. Hover over hex bytes to highlight corresponding tree nodes

### Generate Test Certificate
```bash
openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem -days 365 -nodes \
  -subj "/C=US/ST=CA/L=SF/O=Test/CN=example.com"
```

## Development

### Web Application
```bash
cd asn1_web_decoder

# Build WebAssembly module
./build.sh

# Start development server
cd www
npm install
npm run dev
# Open http://localhost:8080

# Build for production
npm run build
# Output in www/dist/

# Preview production build
npm run preview
```

### Modifying Rust Decoder
```bash
cd asn1_web_decoder
# Edit src/lib.rs
./build.sh
# Restart dev server to see changes
```

### Modifying React UI
```bash
cd asn1_web_decoder/www
# Edit files in src/
# Changes auto-reload with Vite HMR
```

## Project Structure

```
asn1_decoder/
├── asn1_web_decoder/       # Web application
│   ├── src/
│   │   └── lib.rs          # Rust/WASM library
│   ├── www/
│   │   ├── src/
│   │   │   ├── App.jsx             # Main React component
│   │   │   ├── components/         # React components
│   │   │   │   ├── InputDialog.jsx
│   │   │   │   ├── OutputSection.jsx
│   │   │   │   ├── HexViewSection.jsx
│   │   │   │   ├── TreeNode.jsx
│   │   │   │   └── HexView.jsx
│   │   │   └── wasm/               # Symlink to ../../pkg
│   │   ├── dist/                   # Production build output
│   │   ├── index.html              # HTML entry
│   │   ├── package.json
│   │   └── vite.config.js
│   ├── pkg/                # Generated WASM (after build)
│   ├── Cargo.toml
│   ├── build.sh            # Build script
│   ├── README.md
│   └── DEPLOYMENT.md
│
├── .github/
│   └── workflows/
│       └── deploy-pages.yml    # GitHub Pages deployment
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

- **WASM Module**: ~95 KB (optimized)
- **JavaScript Bundle**: ~379 KB (~120 KB gzipped)
- **WASM Load**: ~100-200ms (first time)
- **Decode Time**: <10ms (typical certificate)
- **Render Time**: <50ms (typical tree)
- **Total First Load**: <300ms

## Contributing

The project welcomes contributions:
- Bug fixes
- New ASN.1 type support
- UI/UX improvements
- Documentation enhancements
- Performance optimizations
- New features (export, validation, etc.)

## Deployment

The `www/dist/` directory (after building) contains a complete static website that can be deployed to:
- **GitHub Pages** (automated via GitHub Actions)
- Netlify
- Vercel
- Cloudflare Pages
- AWS S3 + CloudFront
- Any static hosting service

See [DEPLOYMENT.md](./asn1_web_decoder/DEPLOYMENT.md) for detailed instructions.

## License

MIT License - See individual project directories for details.

## Resources

- [ASN.1 Specification](https://www.itu.int/rec/T-REC-X.680)
- [X.509 Certificate Format](https://tools.ietf.org/html/rfc5280)
- [DER Encoding Rules](https://www.itu.int/rec/T-REC-X.690)
- [OpenSSL Documentation](https://www.openssl.org/docs/)
- [WebAssembly](https://webassembly.org/)

## Acknowledgments

- **pem crate**: PEM parsing library
- **wasm-bindgen**: WebAssembly tooling
- **React**: UI framework
- **Material-UI**: Component library
- **Vite**: Build tool
- **OpenSSL**: For test data generation
