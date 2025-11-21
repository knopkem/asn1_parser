# ASN.1 PEM Decoder - React Version

This is the React version of the ASN.1 PEM Decoder web application.

## Features

- Modern React application using functional components and hooks
- **Material-UI (MUI)** integration for professional, polished UI components
- **Full-screen layout** with modal input dialog
- **Two-panel view**: Tree structure and Hex view side-by-side
- **Interactive hex viewer** with hover highlighting
- **Floating Action Button** for quick input access
- Built with Vite for fast development and optimized production builds
- Component-based architecture for better maintainability
- WebAssembly integration for ASN.1 decoding
- Responsive design with Material Design principles
- Beautiful icons from Material Icons
- Real-time byte offset tracking and highlighting
- Independent scrolling for each panel

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
www-react/
├── index.html          # HTML entry point
├── src/
│   ├── main.jsx        # React entry point
│   ├── App.jsx         # Main application component
│   ├── index.css       # Global styles
│   ├── components/     # React components
│   │   ├── InputSection.jsx
│   │   ├── OutputSection.jsx
│   │   └── TreeNode.jsx
│   └── wasm/           # WebAssembly files
│       └── ...
├── package.json        # Dependencies and scripts
└── vite.config.js      # Vite configuration
```

## Components

- **App**: Main application component managing state and WASM initialization
- **InputSection**: Handles PEM input, buttons, and error display
- **OutputSection**: Displays the decoded tree structure
- **TreeNode**: Recursive component for rendering ASN.1 tree nodes

## Technologies

- React 18
- Material-UI (MUI) 5
- Vite 5
- WebAssembly (Rust-compiled ASN.1 decoder)
- Emotion (CSS-in-JS styling)
