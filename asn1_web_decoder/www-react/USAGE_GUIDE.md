# Usage Guide - ASN.1 PEM Decoder with Hex View

## Quick Start

### 1. Load the Application
Open the application in your browser at `http://localhost:8080` (development) or your deployed URL.

### 2. Enter PEM Data
In the **left panel (Input)**, paste your PEM-formatted ASN.1 data:
```
-----BEGIN CERTIFICATE-----
MIIDXTCCAkWgAwIBAgIJAKVpbF9K...
-----END CERTIFICATE-----
```

### 3. Decode
Click the **"Decode"** button with the play icon, or press `Ctrl+Enter`.

### 4. Explore
The application displays three synchronized views:
- **Left**: Your input
- **Center**: Decoded tree structure
- **Right**: Hexadecimal representation

### 5. Interact
**Hover** over any element in the tree view (center panel), and watch the corresponding hex bytes **highlight** in the hex view (right panel)!

## Three-Panel Layout

### Left Panel: Input Section
**Purpose**: Enter or paste PEM-encoded ASN.1 data

**Features**:
- Large text area with monospace font
- **Decode button** (purple gradient) - Primary action
- **Clear button** - Removes all input and results
- **Sample button** - Loads example certificate
- Error messages appear below buttons
- Keyboard shortcut: `Ctrl+Enter` to decode

**Tips**:
- Paste from clipboard
- Text area auto-grows/shrinks
- Supports any PEM format (certificates, keys, etc.)

### Center Panel: Tree View
**Purpose**: Shows decoded ASN.1 structure

**Features**:
- Hierarchical tree display
- Expandable/collapsible nodes
- Color-coded badges:
  - 🔵 **Blue** - Universal tags
  - 🟠 **Orange** - Context tags  
  - 🟣 **Purple** - Application tags
- Construction type indicators:
  - 🟢 **Green** - Constructed
  - 🔴 **Red** - Primitive
- Length information for each node
- Value display for primitive types

**Interactions**:
- Click ▶/▼ icons to expand/collapse
- Hover over nodes to highlight hex
- Smooth animations

### Right Panel: Hex View
**Purpose**: Shows raw hexadecimal bytes

**Features**:
- Professional hex dump format
- Offset column (8-digit hex addresses)
- 16 bytes per row
- Space-separated bytes
- **Interactive highlighting** on hover
- Purple border matching theme

**Format Example**:
```
00000000: 30 82 03 c7 30 82 02 af a0 03 02 01 02 02 14 43
00000010: 0f 40 4c d3 19 c1 c8 3e 14 33 80 40 ad 4d 36 15
```

## Interactive Highlighting

### How It Works

1. **Hover** your mouse over any node in the tree view
2. The corresponding bytes in the hex view **automatically highlight**
3. Highlighted bytes show:
   - Purple background (`#667eea`)
   - White text
   - Slight scale effect (1.05x)
   - Soft shadow

### What Gets Highlighted

Each ASN.1 node consists of:
- **Tag byte** (1 byte) - Node type
- **Length bytes** (1+ bytes) - Content length
- **Content bytes** (variable) - Actual data

When you hover, **all bytes** for that node (tag + length + content) are highlighted together.

### Visual Feedback

**Tree View**:
- Node background changes to light purple
- Slight 2px translation to the right
- Smooth transition

**Hex View**:
- Selected bytes get purple background
- Text changes to white for contrast
- Scale animation draws attention
- Returns to normal when mouse leaves

## Color Coding System

### Tag Class Colors (Chips)

**Universal (Blue)**
- Standard ASN.1 types
- Examples: INTEGER, SEQUENCE, UTF8String
- Color: `#1976d2` (blue)

**Context (Orange)**  
- Context-specific tags
- Usually marked as `[CONTEXT] Tag N`
- Color: `#e65100` (orange)

**Application (Purple)**
- Application-specific tags
- Less common
- Color: `#7b1fa2` (purple)

### Construction Type Colors (Chips)

**Constructed (Green)**
- Contains child elements
- Examples: SEQUENCE, SET
- Outlined green chip
- Color: `#2e7d32`

**Primitive (Red/Pink)**
- Contains raw value
- Examples: INTEGER, UTF8String, OCTET STRING
- Outlined red chip  
- Color: `#c2185b`

## Common ASN.1 Types

### Universal Tags You'll See

**SEQUENCE (Tag 16)** - Constructed container
- Most common type
- Contains ordered list of elements
- Green "CONSTRUCTED" badge

**INTEGER (Tag 2)** - Numeric value
- Primitive type
- Shows decimal value
- Red "PRIMITIVE" badge

**UTF8String (Tag 12)** - Text
- Primitive type
- Shows readable string value

**OCTET STRING (Tag 4)** - Binary data
- Primitive type
- Shows byte length

**OBJECT IDENTIFIER (Tag 6)** - OID
- Primitive type
- Shows dot notation (e.g., `1.2.840.113549`)

**BIT STRING (Tag 3)** - Bit field
- Primitive type
- Shows unused bits and length

## Use Cases

### 1. Certificate Inspection
- Paste X.509 certificate
- Explore issuer, subject, validity
- View public key structure
- Check extensions

### 2. Learning ASN.1
- Understand tag-length-value encoding
- See how nested structures work
- Correlate tree view with hex bytes
- Verify encoding rules

### 3. Debugging
- Find specific byte positions
- Verify tag values
- Check length calculations
- Identify encoding issues

### 4. Security Analysis
- Inspect certificate chains
- Verify signature structure
- Check algorithm identifiers
- Analyze key parameters

## Keyboard Shortcuts

- `Ctrl+Enter` - Decode input
- `Tab` - Navigate between buttons
- `Enter` - Activate focused button
- `Arrow Keys` - Scroll tree/hex view
- Mouse wheel - Scroll panels

## Tips & Tricks

### Best Practices

1. **Use Sample Button** - Click "Sample" to see example output
2. **Start Collapsed** - Click nodes to expand only what you need
3. **Hover Slowly** - Give time for highlighting to appear
4. **Use Zoom** - Browser zoom (Ctrl/Cmd +) for better hex readability
5. **Compare Offsets** - Use hex offset column to find specific positions

### Performance

- **Large Files**: Works well up to 100KB
- **Scroll Smoothly**: Both panels scroll independently
- **Clear Often**: Use Clear button between different inputs

### Troubleshooting

**No Highlighting**
- Ensure you've decoded the data first
- Try hovering on different nodes
- Check if node has byte offset data

**Slow Performance**
- Try smaller PEM inputs
- Close other browser tabs
- Use latest browser version

**Can't See Hex**
- Scroll right panel
- Check if decoding succeeded
- Look for error messages

## Mobile Usage

On mobile devices:
- Panels stack vertically (Input → Tree → Hex)
- **Tap** nodes instead of hover
- Pinch to zoom for better readability
- Swipe to scroll individual panels

## Responsive Breakpoints

- **Desktop** (>1200px): Three columns side-by-side
- **Tablet** (768-1200px): Stacked vertical layout
- **Mobile** (<768px): Full-width stacked layout

## Advanced Features

### Offset Information
Each tree node shows its byte position internally. This enables:
- Precise highlighting
- Debugging byte sequences
- Verifying encoder output

### Nested Highlighting
When hovering over a parent node, only that node's bytes highlight (not children). This helps understand the structure boundaries.

### State Management
The application maintains:
- Current input
- Decoded tree structure
- Hex data
- Highlight range (start/end bytes)
- Error state
- Loading state

## Getting Help

If something doesn't work:
1. Check error messages in red alert boxes
2. Verify PEM format is correct
3. Try the Sample button to ensure app works
4. Clear and try again
5. Check browser console for errors

## Example Workflows

### Inspecting a Certificate

1. Copy certificate from file
2. Paste in input area
3. Click Decode
4. Expand "SEQUENCE (Tag 16)" nodes
5. Find "subject" or "issuer" sections
6. Hover to see exact byte positions
7. Note the hex encoding

### Learning ASN.1 Encoding

1. Load sample certificate
2. Find a simple INTEGER node
3. Hover over it
4. See the highlighted bytes in hex:
   - First byte: tag (02 for INTEGER)
   - Second byte: length
   - Remaining bytes: value
5. Repeat for other types

### Debugging Encoding Issues

1. Paste problematic ASN.1 data
2. Find the suspicious node
3. Hover to see hex bytes
4. Check if tag/length/value match expectations
5. Compare with reference implementation
6. Note any discrepancies

## Summary

The ASN.1 PEM Decoder provides a powerful, interactive way to understand and debug ASN.1 encoded data. The three-panel layout with synchronized highlighting makes it easy to correlate the structured tree view with the raw hexadecimal encoding.

Enjoy exploring ASN.1 structures!
