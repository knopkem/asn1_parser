# ASN.1 Web Decoder

A WebAssembly-powered web application for decoding and visualizing PEM-formatted ASN.1 structures in your browser.

## Features

- **Browser-based**: Runs entirely in your browser using WebAssembly
- **Interactive Tree View**: Visualize ASN.1 structures as an expandable/collapsible tree
- **Real-time Decoding**: Instant decoding of PEM-formatted data
- **No Server Required**: Static website that can be hosted anywhere
- **Detailed Information**: Shows tag types, classes, lengths, and decoded values
- **Beautiful UI**: Modern, responsive design with color-coded badges

## Building

### Prerequisites

- Rust toolchain (install from https://rustup.rs)
- wasm-pack (will be installed by build script if needed)

### Build Steps

1. Run the build script:
```bash
./build.sh
```

This will:
- Install wasm-pack if needed
- Compile the Rust library to WebAssembly
- Generate JavaScript bindings
- Output everything to `www/pkg/`

2. Serve the website locally:
```bash
cd www
python3 -m http.server 8080
```

3. Open your browser to http://localhost:8080

## Usage

1. Paste PEM-formatted ASN.1 data into the text area (e.g., certificates, keys)
2. Click "Decode" or press Ctrl+Enter
3. View the decoded structure in the tree view
4. Click on tree nodes to expand/collapse them

### Buttons

- **Decode**: Decode the input PEM data
- **Clear**: Clear both input and output
- **Load Sample**: Load a sample certificate for testing

## Supported ASN.1 Types

- BOOLEAN
- INTEGER
- BIT STRING
- OCTET STRING
- NULL
- OBJECT IDENTIFIER (OID)
- UTF8String, PrintableString, IA5String, TeletexString
- UTCTime, GeneralizedTime
- SEQUENCE (constructed)
- SET (constructed)
- Context-specific and Application tags

## Architecture

### Rust Library (`src/lib.rs`)

The core decoder is written in Rust and compiled to WebAssembly:
- Parses PEM format
- Decodes DER-encoded ASN.1 structures
- Outputs JSON tree structure
- Uses `wasm-bindgen` for JavaScript interop

### Web Interface (`www/`)

- `index.html`: Main page structure
- `styles.css`: Modern, responsive styling
- `app.js`: JavaScript application logic and tree rendering
- `pkg/`: Generated WebAssembly module and bindings (created during build)

## Deployment

The `www/` directory (after building) contains a complete static website that can be deployed to:
- GitHub Pages
- Netlify
- Vercel
- Any static hosting service
- Local HTTP server

Simply copy the entire `www/` directory to your hosting service.

### GitHub Pages (Automated)

This repository includes a GitHub Actions workflow that automatically builds and deploys the web application to GitHub Pages:

1. The workflow is triggered on pushes to the `main` branch
2. It builds the WebAssembly module from Rust source
3. Deploys the `www/` directory to GitHub Pages

To enable GitHub Pages for your fork:
1. Go to your repository Settings → Pages
2. Under "Build and deployment", select "GitHub Actions" as the source
3. Push to the `main` branch or manually trigger the workflow
4. Your site will be available at `https://<username>.github.io/<repository>/`

See `.github/workflows/deploy-pages.yml` for the workflow configuration.

## Development

To modify the Rust decoder:
1. Edit `src/lib.rs`
2. Run `./build.sh`
3. Refresh your browser

To modify the UI:
1. Edit files in `www/` (except `www/pkg/`)
2. Refresh your browser (no rebuild needed)

## Example Input

```
-----BEGIN CERTIFICATE-----
MIIDXTCCAkWgAwIBAgIJAKVpbF9KLG5cMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV
BAYTAkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEwHwYDVQQKDBhJbnRlcm5ldCBX
aWRnaXRzIFB0eSBMdGQwHhcNMjMwMTAxMDAwMDAwWhcNMjQwMTAxMDAwMDAwWjBF
...
-----END CERTIFICATE-----
```

## Dependencies

### Rust
- `wasm-bindgen`: WebAssembly bindings
- `pem`: PEM format parser
- `serde`: Serialization framework
- `serde_json`: JSON serialization
- `web-sys`: Web API bindings

## License

MIT
