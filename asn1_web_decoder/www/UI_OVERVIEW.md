# UI Overview - ASN.1 PEM Decoder

## Application Layout

The ASN.1 PEM Decoder features a modern, full-screen design optimized for productivity.

### Visual Structure

```
┌────────────────────────────────────────────────────────────────┐
│  ASN.1 PEM Decoder           Hover over tree nodes...    [v]   │  ← App Bar
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌─────────────────────────────┐  ┌──────────────────────────┐│
│  │ Decoded Structure      ⬆️    │  │ Hex View            ⬆️   ││
│  ├─────────────────────────────┤  ├──────────────────────────┤│
│  │                             │  │ 00000000: 30 82 03 c7 ...││
│  │ ▶ PEM: CERTIFICATE          │  │ 00000010: 0f 40 4c d3 ...││
│  │   ▼ SEQUENCE (Tag 16)       │  │ 00000020: 1c 8 3e 14 33...││
│  │     ▶ SEQUENCE (Tag 16)     │  │ 00000030: 80 40 ad 4d  ...││
│  │     ▶ CONTEXT Tag 0         │  │ 00000040: 36 15 1d 28  ...││
│  │     ▶ INTEGER (Tag 2)       │  │ ...                      ││
│  │     ...                     │  │ ...                      ││
│  │                             │  │                          ││
│  │                             │  │                          ││
│  └─────────────────────────────┘  └──────────────────────────┘│
│                                                                │
│                                             ┌─────┐            │
│                                             │  ✏️  │  ← FAB     │
│                                             └─────┘            │
└────────────────────────────────────────────────────────────────┘
```

## Components Breakdown

### 1. App Bar (Top)
**Always visible header**
- **Left**: Application title "ASN.1 PEM Decoder"
- **Right**: Usage hint "Hover over tree nodes to highlight hex bytes"
- **Style**: Purple gradient background, white text
- **Height**: Standard AppBar (64px)

### 2. Main Content Area
**Full-screen workspace**
- **Layout**: 2-column grid on desktop, stacked on mobile
- **Background**: Light gray (#f5f5f5)
- **Padding**: 16px gap between panels
- **Behavior**: Fills remaining viewport height

### 3. Left Panel: Decoded Structure
**Tree view of ASN.1 nodes**
- **Header**: "Decoded Structure" (primary blue color)
- **Content**: Hierarchical tree with expand/collapse
- **Scrolling**: Independent vertical scroll
- **Interaction**: Hover to highlight hex
- **Empty state**: "Enter PEM data and click Decode to begin"

### 4. Right Panel: Hex View
**Raw hexadecimal display**
- **Header**: "Hex View" (secondary purple color)
- **Content**: Formatted hex dump (16 bytes per row)
- **Scrolling**: Independent vertical scroll
- **Highlighting**: Purple background on hover
- **Empty state**: "Decode PEM data to view hex"
- **Footer**: Hint text about hovering

### 5. Floating Action Button (FAB)
**Quick access to input**
- **Position**: Fixed bottom-right (16px from edges)
- **Icon**: Edit/pencil icon
- **Color**: Purple gradient
- **Visibility**: Only shown when dialog is closed and data is decoded
- **Action**: Opens input dialog

### 6. Input Dialog (Modal)
**Full-featured input interface**
- **Size**: Medium width (70% height)
- **Header**: "Enter PEM Data" with close button
- **Content**: Large textarea (20-25 rows)
- **Footer**: Three action buttons
- **Behavior**: Auto-closes on successful decode
- **Keyboard**: Ctrl+Enter to decode

## Interaction Flow

### Step 1: Initial Load
```
Application opens → Input dialog visible → User sees textarea
```

### Step 2: Enter Data
```
User pastes PEM → Or clicks "Sample" → Dialog shows data
```

### Step 3: Decode
```
Click "Decode" (or Ctrl+Enter) → Dialog closes → Two panels appear
```

### Step 4: Explore
```
Hover tree nodes → Hex bytes highlight → Understand structure
```

### Step 5: Modify
```
Click FAB → Dialog reopens → Edit input → Decode again
```

## Color Palette

### Primary Colors
- **Purple-Blue**: `#667eea` - Primary actions, tree view
- **Deep Purple**: `#764ba2` - Secondary actions, hex view
- **Gradient**: Linear from #667eea to #764ba2

### Background Colors
- **App background**: `#f5f5f5` - Light gray
- **Tree panel**: `#ffffff` - White
- **Hex panel**: `#fafafa` - Slightly off-white

### Interactive Colors
- **Highlight**: `#667eea` - Purple background for hex
- **Hover tree**: `rgba(102, 126, 234, 0.12)` - Light purple
- **Selected text**: White on purple background

### Text Colors
- **Primary text**: `rgba(0, 0, 0, 0.87)` - Dark gray
- **Secondary text**: `rgba(0, 0, 0, 0.6)` - Medium gray
- **Disabled text**: `rgba(0, 0, 0, 0.38)` - Light gray
- **White text**: `#ffffff` - On colored backgrounds

## Typography

### Headers
- **App Bar Title**: 20px, bold, white
- **Panel Headers**: 18px, bold, colored
- **Dialog Title**: 20px, bold, white

### Content
- **Tree nodes**: 14px, Courier New (monospace)
- **Hex bytes**: 14px, Courier New (monospace)
- **Buttons**: 16px, medium weight
- **Hints**: 12px, regular weight

## Spacing System

### Material-UI Units (1 unit = 8px)

- **App padding**: 2 units (16px)
- **Panel gap**: 2 units (16px)
- **Panel padding**: 2 units (16px)
- **Component margins**: 1 unit (8px)
- **FAB position**: 2 units (16px) from edges

## Responsive Breakpoints

### Desktop (>960px)
```
┌────────────────────────────────────┐
│        App Bar                     │
├──────────────┬─────────────────────┤
│   Tree View  │    Hex View         │
│              │                     │
│   (50%)      │      (50%)          │
└──────────────┴─────────────────────┘
```

### Mobile (<960px)
```
┌────────────────────┐
│     App Bar        │
├────────────────────┤
│                    │
│    Tree View       │
│   (100% width)     │
│                    │
├────────────────────┤
│                    │
│    Hex View        │
│   (100% width)     │
│                    │
└────────────────────┘
```

## Dialog Layout

### Input Dialog Structure
```
┌─────────────────────────────────────────┐
│ Enter PEM Data                      [×] │  ← Header (Purple)
├─────────────────────────────────────────┤
│                                         │
│  ┌───────────────────────────────────┐ │
│  │ Paste PEM data here...            │ │
│  │                                   │ │
│  │ -----BEGIN CERTIFICATE-----      │ │  ← Large Textarea
│  │ MIIDXTCCAk...                    │ │
│  │ -----END CERTIFICATE-----        │ │
│  │                                   │ │
│  │ (20-25 rows)                     │ │
│  └───────────────────────────────────┘ │
│                                         │
│  [!] Error message (if any)             │
│                                         │
├─────────────────────────────────────────┤
│  [▶ Decode]  [↓ Sample]  [× Clear]     │  ← Action Buttons
└─────────────────────────────────────────┘
```

## Tree Node Display

### Node Structure
```
▼ SEQUENCE (Tag 16) [UNIVERSAL] [CONSTRUCTED] [Length: 965]
  ▶ INTEGER (Tag 2) [UNIVERSAL] [PRIMITIVE] [Length: 3] = 14089475
  ▶ UTF8String (Tag 12) [UNIVERSAL] [PRIMITIVE] [Length: 11] = example.com
```

### Visual Elements
- **Toggle**: ▶/▼ icons for expand/collapse
- **Label**: Node type and tag number
- **Badges**: Colored chips for class and type
- **Length**: Byte count in brackets
- **Value**: Displayed for primitive types

## Hex View Format

### Row Structure
```
00000000: 30 82 03 c7 30 82 02 af a0 03 02 01 02 02 14 43
│         │                                             │
Offset    Hex Bytes (16 per row)                    Last byte
(8 hex)   (2 hex chars each, space-separated)
```

### Highlighting Example
```
00000020: 1c 8█3e█14 33 80 40 ad 4d 36 15 1d 28 00 00 00
          ▲       ▲
          Highlighted bytes (purple background, white text)
```

## State Indicators

### Loading State
- **Tree panel**: Centered spinning circle with "Decoding..." text
- **Dialog**: "Loading..." on disabled button

### Empty State
- **Tree panel**: "Enter PEM data and click Decode to begin"
- **Hex panel**: "Decode PEM data to view hex"

### Error State
- **Dialog**: Red Alert box with error message
- **Dialog stays open**: User can fix input

### Success State
- **Dialog closes**: Automatically hides
- **Panels populate**: Show decoded data
- **FAB appears**: Ready for next edit

## Keyboard Shortcuts

| Shortcut | Action | Context |
|----------|--------|---------|
| `Ctrl+Enter` | Decode input | In dialog |
| `Escape` | Close dialog | When data decoded |
| `Tab` | Navigate buttons | In dialog |
| `F11` | Browser fullscreen | Anywhere |

## Accessibility Features

### Semantic HTML
- Proper heading hierarchy (h1, h2, h3)
- ARIA labels on interactive elements
- Descriptive button text

### Keyboard Navigation
- All buttons keyboard accessible
- Logical tab order
- Focus indicators visible

### Screen Readers
- Descriptive labels for all components
- Status messages announced
- Hierarchical structure clear

### Visual
- High contrast text
- Color not sole indicator
- Readable font sizes

## Performance Characteristics

### Initial Load
- **WASM**: 95KB (loads once)
- **JS Bundle**: 388KB (122KB gzipped)
- **CSS**: <1KB (minimal)
- **Total**: ~484KB before compression

### Runtime
- **Tree rendering**: O(n) where n = nodes
- **Hex formatting**: O(n) where n = bytes
- **Highlighting**: O(1) constant time
- **Scrolling**: Hardware accelerated

### Memory
- **Dialog**: Persists (no unmount)
- **Tree**: Virtual scrolling via browser
- **Hex**: Efficient byte arrays
- **State**: Minimal React state

## Browser Support

| Browser | Version | Status |
|---------|---------|--------|
| Chrome | 90+ | ✅ Full support |
| Firefox | 88+ | ✅ Full support |
| Safari | 14+ | ✅ Full support |
| Edge | 90+ | ✅ Full support |
| Mobile Safari | 14+ | ✅ Full support |
| Chrome Mobile | 90+ | ✅ Full support |

## Summary

The full-screen UI maximizes content visibility while keeping input easily accessible via the modal dialog. The two-panel layout provides synchronized views of the decoded structure and raw hex data, with smooth interactions and professional styling throughout.
