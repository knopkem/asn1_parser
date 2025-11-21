# ASN.1 PEM Decoder - React Application

This is the React-based web interface for the ASN.1 PEM Decoder.

## Features

- Modern React 18 application with functional components and hooks
- **Material-UI (MUI) 5** integration for professional UI
- **Full-screen split-panel layout** with tree view and hex viewer
- **Modal input dialog** with floating action button
- **Interactive hex viewer** with hover highlighting and color coding
- **Collapsible tree nodes** for exploring ASN.1 structures
- Built with Vite for fast development and optimized builds
- Component-based architecture for maintainability
- WebAssembly integration for high-performance ASN.1 decoding
- Responsive design with Material Design principles
- Real-time byte offset tracking and highlighting
- Independent scrolling for tree and hex panels

## Prerequisites

- Node.js (v16 or higher recommended)
- npm or yarn

## Development

1. Install dependencies:
```bash
npm install
```

2. Start the development server:
```bash
npm run dev
```

The application will be available at `http://localhost:8080`

3. Make changes to files in `src/` and they will auto-reload with Hot Module Replacement (HMR)

## Building for Production

Build the application for production:
```bash
npm run build
```

The optimized production build will be in the `dist` directory.

## Preview Production Build

Preview the production build locally:
```bash
npm run preview
```

## Project Structure

```
www/
├── src/
│   ├── main.jsx            # React entry point
│   ├── App.jsx             # Main application component
│   ├── index.css           # Global styles
│   ├── components/         # React components
│   │   ├── InputDialog.jsx         # Modal input with FAB
│   │   ├── OutputSection.jsx       # Tree view panel
│   │   ├── HexViewSection.jsx      # Hex dump panel
│   │   ├── TreeNode.jsx            # Recursive tree component
│   │   └── HexView.jsx             # Hex viewer component
│   └── wasm/               # Symlink to ../../pkg (WASM bindings)
├── dist/                   # Production build output (after npm run build)
├── index.html              # HTML entry point
├── package.json            # Dependencies and scripts
├── vite.config.js          # Vite configuration
└── node_modules/           # Installed dependencies
```

## Components

- **App.jsx**: Main application component managing state, WASM initialization, and layout
- **InputDialog.jsx**: Modal dialog for PEM input with floating action button trigger
- **OutputSection.jsx**: Left panel displaying the decoded ASN.1 tree structure
- **HexViewSection.jsx**: Right panel displaying the color-coded hex dump
- **TreeNode.jsx**: Recursive component for rendering collapsible tree nodes
- **HexView.jsx**: Component for rendering hex bytes with hover highlighting

## Technologies

- React 18
- Material-UI (MUI) 5
- Vite 5
- WebAssembly (Rust-compiled ASN.1 decoder)
- Emotion (CSS-in-JS styling for MUI)

## Documentation

Comprehensive documentation is available in this directory:

- `QUICKSTART.md`: Step-by-step tutorial
- `USAGE_GUIDE.md`: Detailed usage instructions
- `HEX_VIEW_FEATURE.md`: Hex view documentation
- `HEX_COLOR_CODING.md`: Color coding and hover features
- `COMPACT_VIEW.md`: Compact tree view details
- `FULLSCREEN_UI.md`: Full-screen layout design
- `FINAL_OPTIMIZATIONS.md`: Latest optimizations
- `UI_OVERVIEW.md`: Complete UI reference
- `MUI_INTEGRATION.md`: Material-UI integration details
- `WHATS_NEW.md`: Feature changelog
