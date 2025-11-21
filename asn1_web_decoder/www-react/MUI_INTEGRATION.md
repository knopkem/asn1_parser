# Material-UI Integration Guide

## Overview

The ASN.1 PEM Decoder React application has been enhanced with Material-UI (MUI) to provide a professional, polished user interface with Material Design principles.

## Material-UI Components Used

### Layout & Structure
- **ThemeProvider** - Provides custom theme throughout the app
- **CssBaseline** - Normalizes styles across browsers
- **Container** - Responsive fixed-width container
- **Box** - Flexible layout component with sx prop
- **Paper** - Elevated surface for content

### Typography
- **Typography** - Consistent text styling with variant system

### Form Controls
- **TextField** - Multi-line input for PEM data
  - Configured with `multiline`, `minRows`, `maxRows`
  - Custom monospace font for code input
  - Full Material Design styling

### Buttons & Actions
- **Button** - Primary action buttons
  - `variant="contained"` for primary actions
  - `variant="outlined"` for secondary actions
  - `startIcon` for icons
  - Custom gradient background for decode button
- **IconButton** - Tree expand/collapse controls
- **Stack** - Layout for button groups

### Feedback
- **Alert** - Error message display
- **CircularProgress** - Loading indicator
- **Chip** - Badges for ASN.1 tags
  - Color coding based on tag class (primary, warning, secondary)
  - Outlined variant for constructed/primitive indicators

### Icons
From `@mui/icons-material`:
- **PlayArrow** - Decode action
- **Clear** - Clear input
- **CloudDownload** - Load sample
- **ExpandMore** / **ChevronRight** - Tree node expansion

## Custom Theme

```javascript
const theme = createTheme({
  palette: {
    primary: {
      main: '#667eea',  // Purple-blue
    },
    secondary: {
      main: '#764ba2',  // Deep purple
    },
  },
})
```

The theme maintains the original color scheme with Material Design's color system.

## Styling Approach

### sx Prop
Material-UI's `sx` prop is used throughout for component styling:

```javascript
<Box sx={{ 
  minHeight: '100vh',
  background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
  py: 3 
}}>
```

**Benefits**:
- Type-safe styling
- Access to theme values
- Responsive breakpoints
- Performance optimized

### Responsive Design

Grid layout using MUI's breakpoint system:

```javascript
<Box sx={{ 
  display: 'grid',
  gridTemplateColumns: { xs: '1fr', md: '1fr 1fr' },
  gap: 3
}}>
```

- `xs`: Mobile (single column)
- `md`: Desktop (two columns)

## Component Enhancements

### InputSection
- **TextField** with monospace font
- **Button** group with icons and consistent spacing
- **Alert** for error display with severity levels

### OutputSection
- **Paper** with elevation and custom background
- **CircularProgress** for loading state
- **Typography** for empty state messaging

### TreeNode
- **IconButton** for expand/collapse with smooth transitions
- **Chip** components for tag visualization
  - Color-coded by tag class
  - Outlined variant for constructed/primitive
- **Box** with hover effects
- Proper spacing with Material Design tokens

## Color Coding

### Tag Classes
- **Universal** → Primary (blue)
- **Context** → Warning (orange)
- **Application** → Secondary (purple)
- **Default** → Default (grey)

### Constructed/Primitive
- **Constructed** → Success (green)
- **Primitive** → Error (red/pink)

## Accessibility

Material-UI components include:
- Proper ARIA attributes
- Keyboard navigation support
- Focus management
- Screen reader compatibility
- High contrast support

## Performance

- Tree-shaking: Only used components are bundled
- CSS-in-JS: Styles are generated on-demand
- Production build: Optimized and minified
- Gzip compression: 118.55 KB (from 372.81 KB)

## Migration from Custom CSS

### Before (Custom CSS)
```css
.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}
```

### After (Material-UI)
```javascript
<Button 
  variant="contained"
  sx={{ 
    background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
  }}
>
  Decode
</Button>
```

## Benefits of Material-UI

1. **Consistency**: Design system ensures visual consistency
2. **Accessibility**: Built-in accessibility features
3. **Responsive**: Mobile-first responsive design
4. **Customizable**: Easy to customize with theme
5. **Documentation**: Extensive documentation and examples
6. **Community**: Large community and ecosystem
7. **Updates**: Regular updates and maintenance
8. **TypeScript**: Full TypeScript support available
9. **Icons**: Comprehensive icon library included
10. **Testing**: Well-tested components

## Future Enhancements

Potential additions using MUI:
- **Dark Mode**: Toggle with `ThemeProvider` and `useTheme`
- **Snackbar**: Toast notifications for actions
- **Dialog**: Modal dialogs for settings
- **Drawer**: Side drawer for additional options
- **Tabs**: Tabbed interface for multiple decoders
- **Tooltip**: Helpful hints on hover
- **Menu**: Dropdown menus for advanced options
- **Autocomplete**: For common PEM types
- **DataGrid**: For tabular ASN.1 data display

## MCP Server Integration

The Material-UI MCP server is configured in `.vscode/mcp.json`:

```json
{
  "servers": {
    "mui-mcp": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@mui/mcp@latest"]
    }
  }
}
```

This provides enhanced development experience with Material-UI component suggestions and documentation.
