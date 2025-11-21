# Hex View Feature

## Overview

The ASN.1 PEM Decoder now includes a **Hex View** panel that displays the raw hexadecimal representation of the decoded ASN.1 data. When you hover over elements in the tree view, the corresponding hex bytes are highlighted in the hex view panel.

## Features

### Three-Column Layout

The application now uses a three-column layout on desktop:
1. **Left**: Input section (PEM data entry)
2. **Center**: Tree view (decoded ASN.1 structure)
3. **Right**: Hex view (hexadecimal representation)

On smaller screens, the columns stack vertically for better mobile experience.

### Interactive Highlighting

**Hover Interaction**: When you hover over any node in the tree view, the corresponding bytes in the hex view are automatically highlighted.

**Visual Feedback**:
- Highlighted bytes have a purple background (primary color)
- White text for better contrast
- Slight scale animation (1.05x)
- Soft shadow effect
- Smooth transitions

### Hex Display Format

The hex view displays data in a professional format:
```
00000000: 30 82 03 c7 30 82 02 af a0 03 02 01 02 02 14 43
00000010: 0f 40 4c d3 19 c1 c8 3e 14 33 80 40 ad 4d 36 15
...
```

**Features**:
- **Offset column**: Shows byte offset in hexadecimal (8 digits)
- **Hex bytes**: 16 bytes per row, space-separated
- **Monospace font**: Courier New for readability
- **Row hover**: Subtle background on row hover

### Byte Offset Tracking

The Rust WASM decoder now tracks:
- `byte_offset`: Starting position of each ASN.1 node
- `byte_length`: Total length including tag, length, and content bytes

This information enables precise highlighting when hovering over tree nodes.

## Technical Implementation

### Rust WASM Changes

Added to `Asn1Node` struct:
```rust
pub byte_offset: usize,
pub byte_length: usize,
```

New WASM function:
```rust
#[wasm_bindgen]
pub fn pem_to_hex(pem_input: &str) -> Result<String, JsValue>
```

This function converts PEM input to a continuous hex string for display.

### React Components

#### New Components

**HexView.jsx**
- Displays hex data in formatted rows
- Handles highlighting based on byte ranges
- Memo-ized for performance
- Groups bytes into 16-byte rows

**HexViewSection.jsx**
- Wrapper component for HexView
- Styled Paper container
- Help text for user guidance

#### Modified Components

**App.jsx**
- Added `hexData` state
- Added `highlightRange` state (start/end)
- `handleNodeHover` callback for hover events
- Calls `pem_to_hex` during decode
- Three-column grid layout

**TreeNode.jsx**
- Added `onMouseEnter` and `onMouseLeave` handlers
- Passes `onNodeHover` to children
- Enhanced hover visual feedback

**OutputSection.jsx**
- Passes `onNodeHover` to TreeNode
- Updated to support callback propagation

### Styling

**Hex View Colors**:
- Background: `#fafafa` (light gray)
- Border: Secondary color (purple)
- Highlighted bytes: Primary color background (`#667eea`)
- Highlighted text: White
- Offset column: Gray text

**Tree View Enhancement**:
- Stronger hover effect (12% opacity instead of 8%)
- Slight translation on hover (2px right)
- Smooth transitions

## User Experience

### Workflow

1. **Paste PEM Data**: Enter certificate or other PEM data
2. **Decode**: Click decode button
3. **Explore Tree**: View the decoded ASN.1 structure
4. **Hover**: Move mouse over any tree element
5. **See Hex**: Corresponding bytes highlight in hex view
6. **Understand**: Visual connection between structure and raw data

### Benefits

**Educational**: Helps understand ASN.1 structure by showing raw encoding

**Debugging**: Identify specific byte positions for troubleshooting

**Verification**: Confirm expected values at byte level

**Learning**: See how tag-length-value encoding works

## Color Scheme

The hex view uses consistent colors with the rest of the application:

**Primary (Purple-Blue)**: 
- Highlighted hex bytes
- Tree view primary elements

**Secondary (Deep Purple)**:
- Hex view border
- Section title

**Neutral**:
- Background: Light gray (`#fafafa`)
- Text: Dark gray for offsets
- Hover: Very light background

## Performance

**Optimizations**:
- `memo()` wrapper on HexView component
- Efficient highlighting calculation
- CSS transitions for smooth animations
- Only re-renders on hex data or highlight change

**Memory**:
- Hex string stored as single state value
- Minimal re-renders during hover
- Byte arrays processed only once

## Responsive Design

**Desktop (>1200px)**:
- Three columns: Input | Tree | Hex
- Each column has adequate space
- 400px fixed width for input/hex, flexible center

**Tablet (768px - 1200px)**:
- Columns stack: Input, Tree, Hex (vertical)
- Full width for each section
- Maintain 600px max height

**Mobile (<768px)**:
- Single column layout
- Reduced padding
- Touch-friendly hover (tap to highlight)

## Accessibility

**Keyboard Support**:
- Hex view is read-only (no keyboard interaction needed)
- Tree navigation works with tab/enter
- Screen readers announce byte offsets

**Visual**:
- High contrast between highlighted and normal bytes
- Clear visual separation between sections
- Readable monospace font

**ARIA**:
- Semantic HTML structure
- Proper heading hierarchy
- Labels for all regions

## Future Enhancements

Possible additions:

1. **ASCII Column**: Show ASCII representation alongside hex
2. **Byte Search**: Search for specific hex sequences
3. **Copy Hex**: Copy selected hex ranges
4. **Hex Editing**: Edit bytes and re-encode (advanced)
5. **Color Coding**: Different colors for tags, length, content
6. **Split View**: Toggle hex view position (right/bottom)
7. **Zoom**: Adjustable font size for hex display
8. **Export**: Save hex dump to file
9. **Comparison**: Side-by-side hex comparison
10. **Annotations**: Add notes to specific byte ranges

## Browser Compatibility

Tested and working on:
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

Requires:
- ES6 support
- WebAssembly support
- CSS Grid
- Flexbox

## Known Limitations

1. **Large Files**: Hex view may be slow for very large PEM files (>1MB)
2. **Mobile Hover**: Touch devices require tap to activate hover
3. **Nested Highlights**: Only one highlight range active at a time
4. **Clipboard**: No built-in copy hex functionality yet

## Tips

- **Use on desktop** for best experience with hover interaction
- **Zoom browser** (Ctrl/Cmd +) if hex text is too small
- **Collapse nodes** in tree view to reduce visual clutter
- **Reference the offset column** to find byte positions
- **Compare with other tools** to verify byte-level accuracy
