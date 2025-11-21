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
- Node.js v16+ and npm

### Build Steps

1. Build the WebAssembly module:
```bash
./build.sh
```

This will:
- Install wasm-pack if needed
- Compile the Rust library to WebAssembly
- Generate JavaScript bindings
- Output everything to `pkg/` (symlinked from `www/src/wasm`)

2. Install dependencies and start dev server:
```bash
cd www
npm install
npm run dev
```

3. Open your browser to http://localhost:8080

### Production Build

```bash
cd www
npm run build
```

The optimized production build will be in `www/dist/` directory.

## Usage

1. Click the **floating action button** (bottom-right corner) to open the input dialog
2. Paste PEM-formatted ASN.1 data into the text area (or click "Load Sample")
3. Click "Decode" to parse the structure
4. View the decoded tree structure in the left panel
5. View the color-coded hex dump in the right panel
6. Click on tree nodes to expand/collapse them
7. Hover over hex bytes to highlight corresponding tree nodes

### Features

- **Full-Screen Layout**: Optimized two-panel interface
- **Modal Input**: Floating action button for quick access
- **Tree View**: Interactive, collapsible tree structure
- **Hex View**: Color-coded hex dump with byte offsets
- **Highlighting**: Hover interaction between tree and hex views
- **Material Design**: Professional UI components
- **Responsive**: Works on desktop and mobile

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
- Outputs JSON tree structure with byte offsets
- Uses `wasm-bindgen` for JavaScript interop
- Provides `pem_to_hex()` function for hex view

### React Application (`www/`)

Modern React 18 application with Material-UI:
- **Components**:
  - `App.jsx`: Main application with state management
  - `InputDialog.jsx`: Modal dialog for PEM input with floating action button
  - `OutputSection.jsx`: Tree view panel with collapsible nodes
  - `HexViewSection.jsx`: Hex dump panel with color coding
  - `TreeNode.jsx`: Recursive tree node component
  - `HexView.jsx`: Color-coded hex display with hover highlighting
- **Build System**: Vite for fast development and optimized production builds
- **Styling**: Material-UI components with Emotion CSS-in-JS
- **State Management**: React hooks (useState, useEffect, useRef)
- **WASM Integration**: Direct import from generated bindings via symlink

### Build Output

After building:
- `pkg/`: WebAssembly module (~95KB) and JavaScript bindings
- `www/dist/`: Optimized production build
  - JavaScript bundle (~379KB, ~120KB gzipped)
  - HTML and assets
  - Ready for static hosting

## Deployment

The `www/dist/` directory (after `npm run build`) contains a complete static website ready for deployment.

### GitHub Pages (Automated)

This repository includes a GitHub Actions workflow that automatically builds and deploys to GitHub Pages:

1. The workflow triggers on pushes to the `main` branch (or manually via workflow_dispatch)
2. Builds the WebAssembly module from Rust source
3. Installs Node.js dependencies and builds the React app with Vite
4. Deploys the `www/dist/` directory to GitHub Pages

**To enable GitHub Pages:**
1. Go to repository Settings → Pages
2. Under "Build and deployment", select "GitHub Actions" as the source
3. Push to `main` branch or manually trigger the workflow
4. Your site will be available at `https://<username>.github.io/<repository>/`

See `.github/workflows/deploy-pages.yml` for the workflow configuration.

### Other Platforms

The static build in `www/dist/` can be deployed to:
### Other Platforms

The static build in `www/dist/` can be deployed to:
- Netlify
- Vercel
- Cloudflare Pages
- AWS S3 + CloudFront
- Firebase Hosting
- Any static hosting service

See [DEPLOYMENT.md](./DEPLOYMENT.md) for detailed platform-specific instructions.

## Development

### Project Structure

```
asn1_web_decoder/
├── src/
│   └── lib.rs                  # Rust WASM library
├── www/
│   ├── src/
│   │   ├── App.jsx             # Main React component
│   │   ├── main.jsx            # React entry point
│   │   ├── index.css           # Global styles
│   │   ├── components/         # React components
│   │   │   ├── InputDialog.jsx
│   │   │   ├── OutputSection.jsx
│   │   │   ├── HexViewSection.jsx
│   │   │   ├── TreeNode.jsx
│   │   │   └── HexView.jsx
│   │   └── wasm/               # Symlink to ../../pkg
│   ├── dist/                   # Production build output
│   ├── index.html              # HTML entry point
│   ├── package.json
│   └── vite.config.js          # Vite configuration
├── pkg/                        # Generated WASM (after build)
├── Cargo.toml
└── build.sh                    # Build script
```

### Modifying the Rust Decoder

1. Edit `src/lib.rs`
2. Rebuild WASM: `./build.sh`
3. Restart dev server: `cd www && npm run dev`

The WASM bindings are automatically available via symlink at `www/src/wasm`.

### Modifying the React UI

1. Edit files in `www/src/`
2. Changes auto-reload with Vite Hot Module Replacement (HMR)
3. No rebuild needed during development

### Testing Production Build

```bash
cd www
npm run build    # Build for production
npm run preview  # Preview production build locally
```

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

### JavaScript/React
- `react` & `react-dom`: UI framework
- `@mui/material`: Material Design components
- `@mui/icons-material`: Material Design icons
- `vite`: Build tool and dev server

## Documentation

Comprehensive documentation is available in the `www/` directory:

- `README.md`: Quick start guide
- `QUICKSTART.md`: Step-by-step tutorial
- `USAGE_GUIDE.md`: Detailed usage instructions
- `HEX_VIEW_FEATURE.md`: Hex view documentation
- `HEX_COLOR_CODING.md`: Color coding and auto-scroll
- `COMPACT_VIEW.md`: Compact tree view details
- `FULLSCREEN_UI.md`: Full-screen layout design
- `FINAL_OPTIMIZATIONS.md`: Latest optimizations
- `UI_OVERVIEW.md`: Complete UI reference
- `MUI_INTEGRATION.md`: Material-UI integration
- `WHATS_NEW.md`: Feature changelog

## License

MIT
