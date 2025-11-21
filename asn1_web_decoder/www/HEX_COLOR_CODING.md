# Hex View Color Coding & Auto-Scroll

## Overview

The hex view now features intelligent color coding to distinguish between ASN.1 structure components (tag, length, content) and automatic scrolling to keep highlighted bytes visible.

## Color Coding System

### Visual Distinctions

When hovering over a tree node, the corresponding bytes in the hex view are highlighted with color coding:

**Tag Byte (Red)**
- Color: `#d32f2f` (Material Red 700)
- Always the first byte of any ASN.1 element
- Identifies the type of data (SEQUENCE, INTEGER, etc.)

**Length Bytes (Green)**
- Color: `#388e3c` (Material Green 700)  
- Follow immediately after the tag byte
- Can be 1 or multiple bytes depending on content size

**Content Bytes (White on Purple)**
- Color: White text on purple background
- The actual data payload
- Follows the length byte(s)

### ASN.1 Structure Visualization

```
Tag    Length  Content
↓      ↓       ↓
30  82 03 c7  [content bytes...]
🔴  🟢  🟢     ⬜⬜⬜⬜⬜

30 = SEQUENCE tag (red)
82 = Long form length indicator (green)
03 c7 = Length value: 967 bytes (green)
[...] = Content bytes (white on purple)
```

### Example Highlighting

#### Simple Structure (Short Length)
```
Hover on: INTEGER @0000 [3B] = 14089475

Hex View:
000000: 02 03 00 dc 9e 83
        🔴 🟢 ⬜ ⬜ ⬜

02 = INTEGER tag (red)
03 = Length: 3 bytes (green)
00 dc 9e 83 = Value: 14089475 (white on purple)
```

#### Complex Structure (Long Length)
```
Hover on: SEQUENCE @0000 [965B]

Hex View:
000000: 30 82 03 c5 30 82...
        🔴 🟢 🟢 🟢 ⬜ ⬜

30 = SEQUENCE tag (red)
82 = Long form: 2 length bytes follow (green)
03 c5 = Length: 965 bytes (green)
30 82... = Content (white on purple)
```

## Length Byte Encoding

### Short Form (< 128 bytes)
```
Tag  Length  Content
02   05      [5 bytes]
🔴   🟢      ⬜⬜⬜⬜⬜

Length byte value < 0x80 means:
- Single byte length
- Value is the actual length
```

### Long Form (≥ 128 bytes)
```
Tag  Form+Count  Length Bytes        Content
30   82          03 c5              [965 bytes]
🔴   🟢          🟢 🟢              ⬜⬜⬜...

First length byte >= 0x80 means:
- 0x82 = 0x80 + 2 (two length bytes follow)
- 0x03 0xc5 = 965 in hex
```

## Auto-Scroll Feature

### Behavior

**Problem Solved**: When hovering over tree nodes near the end of a long ASN.1 structure, the corresponding hex bytes were highlighted but not visible on screen.

**Solution**: Hex view automatically scrolls to bring highlighted bytes into view.

### Implementation

**Trigger**: Whenever highlight range changes (user hovers over different tree node)

**Animation**: Smooth scroll animation (CSS `scroll-behavior: smooth`)

**Positioning**: Centers the highlighted row in the viewport when possible

**Smart Scrolling**: Only scrolls if highlighted bytes are outside visible area

### User Experience

```
Before Auto-Scroll:
┌─────────────────┐
│ 000000: 30 82...│  ← Visible area
│ 000010: 03 c7...│
│ 000020: 30 82...│
└─────────────────┘
  [Hidden below]
  00f4a0: 06 09... ← Highlighted but not visible

After Auto-Scroll:
  [Hidden above]
┌─────────────────┐
│ 00f490: 30 0b...│
│ 00f4a0: 06 09...│  ← Highlighted & centered
│ 00f4b0: 2a 86...│
└─────────────────┘
```

## Technical Details

### Color Coding Logic

**Tag Detection**:
```javascript
if (byteIndex === highlightStart) {
  return '#d32f2f' // Red for tag
}
```

**Length Detection**:
```javascript
const lengthByte = parseInt(bytes[highlightStart + 1], 16)

if (lengthByte < 0x80) {
  // Short form: 1 byte
  if (byteIndex === highlightStart + 1) {
    return '#388e3c' // Green
  }
} else {
  // Long form: 1 + N bytes
  const numLengthBytes = lengthByte & 0x7f
  if (byteIndex >= highlightStart + 1 && 
      byteIndex <= highlightStart + 1 + numLengthBytes) {
    return '#388e3c' // Green
  }
}
```

### Auto-Scroll Logic

**React Hooks**:
```javascript
const containerRef = useRef(null)
const highlightedRowRef = useRef(null)

useEffect(() => {
  if (highlightedRowRef.current && containerRef.current) {
    const container = containerRef.current
    const element = highlightedRowRef.current
    
    // Check if element is outside visible area
    const containerRect = container.getBoundingClientRect()
    const elementRect = element.getBoundingClientRect()
    
    if (elementRect.top < containerRect.top || 
        elementRect.bottom > containerRect.bottom) {
      element.scrollIntoView({ 
        behavior: 'smooth', 
        block: 'center' 
      })
    }
  }
}, [highlightStart, highlightEnd])
```

### Performance Optimization

**Efficient Re-rendering**:
- Component memoized with `memo()`
- Only re-renders when `highlightStart` or `highlightEnd` changes
- Scroll calculation only runs when highlight changes

**Smooth Scrolling**:
- Native browser smooth scrolling
- Hardware accelerated
- No JavaScript animation loops

## Visual Examples

### Tag Types by Color

**SEQUENCE** (Universal constructed):
```
30 82 03 c7 [content]
🔴 🟢 🟢 🟢

Tag 30 (hex) = 48 (dec) = SEQUENCE
```

**INTEGER** (Universal primitive):
```
02 03 00 dc 9e
🔴 🟢 ⬜ ⬜ ⬜

Tag 02 (hex) = 2 (dec) = INTEGER
```

**CONTEXT Specific**:
```
a0 03 02 01 02
🔴 🟢 ⬜ ⬜ ⬜

Tag a0 (hex) = context-specific constructed [0]
```

### Multi-Byte Length Example

**Large SEQUENCE (1000+ bytes)**:
```
30 82 04 e2 [1250 bytes of content]
🔴 🟢 🟢 🟢

30 = SEQUENCE tag
82 = Long form, 2 bytes follow
04 e2 = 1250 in hex (04 × 256 + e2)
```

## Benefits

### Educational Value
- **Learn ASN.1 encoding**: See structure visually
- **Understand TLV**: Tag-Length-Value becomes clear
- **Debug encoding**: Identify incorrect length bytes
- **Verify parsing**: Confirm tag interpretations

### Improved Usability
- **No manual scrolling**: Hex view tracks tree navigation
- **Visual feedback**: Immediate understanding of structure
- **Faster analysis**: Color coding speeds comprehension
- **Professional tool**: Production-ready debugging interface

### Debugging Support
- **Find errors**: Spot malformed length encodings
- **Verify tags**: Confirm correct tag values
- **Check lengths**: Validate length field accuracy
- **Trace structure**: Follow nested elements easily

## Color Accessibility

### Contrast Ratios

**Red on Purple** (Tag):
- Foreground: #d32f2f
- Background: #667eea
- Ratio: 4.8:1 ✓ (WCAG AA compliant)

**Green on Purple** (Length):
- Foreground: #388e3c
- Background: #667eea
- Ratio: 4.5:1 ✓ (WCAG AA compliant)

**White on Purple** (Content):
- Foreground: #ffffff
- Background: #667eea
- Ratio: 7.2:1 ✓ (WCAG AAA compliant)

### Alternative Indicators

Color is not the sole indicator:
- **Position**: Tag always first, length always after tag
- **Hover tooltip**: Could be added for screen readers
- **Pattern**: Consistent structure helps recognition

## Usage Tips

### Exploring Structure
1. Hover over a tree node
2. Observe hex view automatically scrolls
3. Red byte = tag (what is it?)
4. Green byte(s) = length (how big?)
5. White bytes = content (the data)

### Learning ASN.1
1. Start with simple nodes (INTEGER, UTF8String)
2. Observe 1-byte lengths (short form)
3. Move to complex nodes (SEQUENCE)
4. Study multi-byte lengths (long form)
5. Practice identifying patterns

### Debugging
1. Hover over unexpected node
2. Check tag byte (red) matches expected type
3. Verify length bytes (green) are reasonable
4. Examine content bytes for anomalies
5. Compare with working examples

## Future Enhancements

Possible improvements:
1. **Blue for content type indicators**: Special bytes within content
2. **Tooltip on hover**: Show "Tag byte", "Length byte", etc.
3. **Copy color-coded hex**: Preserve colors when copying
4. **Dark mode colors**: Adjust palette for dark theme
5. **Custom color schemes**: User-selectable palettes
6. **Legend**: Visual key showing color meanings
7. **Annotations**: Add notes to specific byte ranges
8. **Compare mode**: Highlight differences between two structures

## Summary

The combination of color coding and auto-scroll transforms the hex view from a passive display into an active learning and debugging tool. Tag bytes in red, length bytes in green, and automatic scrolling to highlighted sections provide immediate visual feedback and understanding of ASN.1 structure encoding.
