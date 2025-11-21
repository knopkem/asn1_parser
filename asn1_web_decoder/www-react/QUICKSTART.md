# Quick Start Guide - ASN.1 PEM Decoder (React + Material-UI)

## Prerequisites

- Node.js v16 or higher
- npm or yarn package manager

## Installation

```bash
cd www-react
npm install
```

This will install all dependencies including:
- React 18
- Material-UI 7
- Emotion (CSS-in-JS)
- Vite
- Icons and other required packages

## Development

Start the development server with hot module replacement:

```bash
npm run dev
```

The application will be available at **http://localhost:8080**

Changes to the code will automatically reload in the browser.

## Building for Production

Create an optimized production build:

```bash
npm run build
```

The build output will be in the `dist/` directory:
- Minified JavaScript bundles
- Optimized assets
- WASM files
- Static HTML

## Preview Production Build

Preview the production build locally:

```bash
npm run preview
```

This serves the `dist/` directory to test the production build before deployment.

## Key Features

### Material-UI Components
- Professional, polished interface
- Responsive design out of the box
- Accessibility built-in
- Material Design icons

### User Interface
- **Input Section**: Multi-line text field with monospace font
- **Action Buttons**: Decode, Clear, Load Sample with icons
- **Error Handling**: Material-UI Alert component
- **Tree Visualization**: Collapsible tree with color-coded chips
- **Loading States**: CircularProgress indicator

### Keyboard Shortcuts
- **Ctrl+Enter**: Decode PEM input

## Project Structure

```
www-react/
├── index.html              # Entry point
├── package.json            # Dependencies & scripts
├── vite.config.js          # Vite configuration
├── src/
│   ├── main.jsx            # React app initialization
│   ├── App.jsx             # Main component with theme
│   ├── index.css           # Minimal global styles
│   ├── components/
│   │   ├── InputSection.jsx    # Input form with MUI
│   │   ├── OutputSection.jsx   # Output display with MUI
│   │   └── TreeNode.jsx        # Tree node with MUI
│   └── wasm/               # WebAssembly decoder
└── dist/                   # Production build (after build)
```

## Using the Application

1. **Paste PEM Data**: Enter your PEM-formatted ASN.1 data in the input field
2. **Click Decode**: Or press Ctrl+Enter to decode
3. **View Tree**: Explore the decoded structure
4. **Expand/Collapse**: Click arrow icons to expand/collapse nodes
5. **Load Sample**: Click "Sample" button to load example certificate

## Color Coding

### Tag Classes
- 🔵 **Blue (Primary)**: Universal tags
- 🟠 **Orange (Warning)**: Context tags
- 🟣 **Purple (Secondary)**: Application tags

### Construction Type
- 🟢 **Green (Success)**: Constructed
- 🔴 **Red (Error)**: Primitive

## Customization

### Theme Colors
Edit `src/App.jsx` to customize the color scheme:

```javascript
const theme = createTheme({
  palette: {
    primary: { main: '#667eea' },
    secondary: { main: '#764ba2' },
  },
})
```

### Component Styling
Use Material-UI's `sx` prop for custom styling:

```javascript
<Box sx={{ padding: 2, backgroundColor: 'primary.main' }}>
```

## Troubleshooting

### Port Already in Use
If port 8080 is in use, edit `vite.config.js`:

```javascript
server: {
  port: 3000  // Change to any available port
}
```

### Build Errors
Clear node_modules and reinstall:

```bash
rm -rf node_modules package-lock.json
npm install
```

### WASM Loading Issues
Ensure the `src/wasm/` directory contains the compiled WebAssembly files from the Rust project.

## Performance

- **Development**: Fast HMR with Vite
- **Production**: Optimized bundles
  - JavaScript: 372 KB (118 KB gzipped)
  - WASM: 63 KB
  - Loads in under 1 second on modern connections

## Browser Support

Supports all modern browsers:
- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Mobile browsers (iOS Safari, Chrome Mobile)

## Next Steps

- Add dark mode toggle
- Implement TypeScript
- Add unit tests
- Add more ASN.1 format support
- Export decoded data as JSON
- Add syntax highlighting

## Documentation

- [Material-UI Docs](https://mui.com/material-ui/getting-started/)
- [React Docs](https://react.dev/)
- [Vite Docs](https://vitejs.dev/)

## License

See main project LICENSE file.
