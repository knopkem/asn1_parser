# Full-Screen UI Design

## Overview

The ASN.1 PEM Decoder now features a modern, full-screen layout optimized for maximum viewing area. The input field is presented as a modal dialog that automatically hides after decoding, allowing the decoded structure and hex view to occupy the entire viewport.

## Key Features

### Full-Screen Layout
- **100% viewport height**: Application fills the entire browser window
- **No wasted space**: Eliminates margins and padding around main content
- **Responsive grid**: Two-column layout adapts to screen size
- **Proper scrolling**: Each panel scrolls independently

### Modal Input Dialog
- **On-demand input**: Opens as a modal dialog
- **Auto-hide**: Automatically closes after successful decode
- **Large text area**: 20+ rows for comfortable editing
- **Always accessible**: Floating action button (FAB) to reopen

### Floating Action Button (FAB)
- **Bottom-right corner**: Fixed position for easy access
- **Edit icon**: Clear indication of purpose
- **Purple gradient**: Matches application theme
- **Shows only when needed**: Hidden when dialog is open

### Fixed App Bar
- **Always visible**: Top navigation stays in place
- **Application title**: "ASN.1 PEM Decoder"
- **Usage hint**: "Hover over tree nodes to highlight hex bytes"
- **Gradient background**: Purple theme

## Layout Structure

```
┌─────────────────────────────────────────────────────┐
│ App Bar (Fixed)                                     │
│ ASN.1 PEM Decoder                    [Hover hint]   │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌──────────────────┐  ┌──────────────────┐       │
│  │                  │  │                  │       │
│  │  Decoded         │  │  Hex View        │       │
│  │  Structure       │  │                  │       │
│  │                  │  │                  │       │
│  │  (scrollable)    │  │  (scrollable)    │       │
│  │                  │  │                  │       │
│  │                  │  │                  │       │
│  └──────────────────┘  └──────────────────┘       │
│                                                     │
│                                    [FAB Button]     │
└─────────────────────────────────────────────────────┘
```

## User Flow

### Initial State
1. Application loads with input dialog open
2. User sees large text area ready for input
3. Dialog displays placeholder text with example
4. Three action buttons: Decode, Sample, Clear

### After Decoding
1. User enters PEM data
2. Clicks "Decode" button
3. Dialog automatically closes
4. Two-panel view fills the screen
5. FAB appears in bottom-right corner

### Editing Input
1. User clicks FAB button
2. Dialog reopens with current input
3. User can modify or replace content
4. Click "Decode" again to update
5. Dialog closes, view updates

## Component Changes

### App.jsx
**New Features**:
- Full-screen container (`height: 100vh`)
- AppBar component for header
- Grid layout without Container wrapper
- Floating Action Button (FAB)
- Dialog state management (`dialogOpen`)
- Auto-close dialog on successful decode
- Auto-open dialog on decode error

**Layout**:
- `display: flex, flexDirection: column`
- AppBar at top
- Main content area flexGrow: 1
- Grid with two equal columns on desktop
- Overflow hidden to prevent page scroll

### InputDialog.jsx (New)
**Features**:
- Material-UI Dialog component
- Full-width modal (maxWidth: "md")
- 70% viewport height
- Large TextField (20-25 rows)
- Gradient header with close button
- Three action buttons in footer
- Error Alert display
- Keyboard shortcut (Ctrl+Enter)

**Styling**:
- Purple gradient header
- Monospace font for input
- Large, comfortable text area
- Prominent Decode button

### OutputSection.jsx
**Changes**:
- Full-height container
- Flexbox layout
- Independent scrolling
- Centered empty state
- Elevation 2 Paper (subtle shadow)

### HexViewSection.jsx
**Changes**:
- Full-height container
- Flexbox layout
- Independent scrolling
- Centered empty state
- Purple border styling

## Responsive Behavior

### Desktop (>960px)
- Two-column grid layout
- 50% width for each panel
- Both panels visible side-by-side
- Full viewport height

### Tablet/Mobile (<960px)
- Single column layout
- Panels stack vertically
- Each panel takes full width
- Scroll to see second panel

## Scrolling Behavior

### Independent Scrolling
Each panel has its own scrollbar:
- **Tree View**: Scrolls through decoded structure
- **Hex View**: Scrolls through hex dump

### Overflow Handling
- `overflow: auto` on each Paper component
- Only shows scrollbar when content exceeds height
- Smooth scrolling on all devices

### No Page Scroll
- Main container: `overflow: hidden`
- Body: `overflow: hidden`
- Prevents double scrollbars
- Cleaner user experience

## Accessibility

### Keyboard Navigation
- **Tab**: Navigate between buttons
- **Ctrl+Enter**: Decode input (in dialog)
- **Escape**: Close dialog (if data decoded)
- **Enter**: Activate focused button

### Screen Readers
- AppBar has proper heading structure
- Dialog has descriptive title
- FAB has aria-label
- Panels have semantic structure

### Focus Management
- Dialog auto-focuses text area
- FAB is keyboard accessible
- Logical tab order

## Performance Optimizations

### Efficient Rendering
- Only visible panels render content
- Memo-ized components (HexView)
- Minimal re-renders on state changes

### Scroll Performance
- CSS `overflow: auto` for hardware acceleration
- No scroll event listeners
- Smooth native scrolling

### Memory Management
- Dialog content persists (no unmount)
- Tree nodes render only when visible
- Hex rows virtualized via native scroll

## Styling Details

### Colors
- **App Bar**: Purple gradient (#667eea to #764ba2)
- **Background**: Light gray (#f5f5f5)
- **Paper**: White (#ffffff) for tree, light gray (#fafafa) for hex
- **FAB**: Purple gradient matching app bar

### Spacing
- **Padding**: 2 units (16px) around main grid
- **Gap**: 2 units (16px) between panels
- **Panel padding**: 2 units (16px) inside

### Shadows
- **App Bar**: Default elevation
- **Paper**: Elevation 2 (subtle shadow)
- **FAB**: Default elevation with gradient

## Browser Compatibility

### Tested Browsers
- Chrome 90+ ✓
- Firefox 88+ ✓
- Safari 14+ ✓
- Edge 90+ ✓

### Required Features
- CSS Flexbox
- CSS Grid
- `100vh` units
- Dialog API (polyfilled by MUI)

## Known Limitations

### Small Screens
- On very small screens (<600px), panels may be cramped
- Consider portrait orientation for mobile

### Large Content
- Very large ASN.1 structures may slow down
- Hex view optimized for files up to 100KB

### Browser Zoom
- Layout maintains at 80%-150% zoom
- Text remains readable
- Panels may require scrolling at high zoom

## Tips for Users

### Maximize Space
1. Use browser full-screen (F11)
2. Hide browser bookmarks bar
3. Use desktop app mode (Chrome/Edge)

### Efficient Workflow
1. Keep frequently used PEM data in clipboard
2. Use Sample button for quick testing
3. Use Ctrl+Enter for faster decoding
4. Click FAB instead of menu navigation

### Better Visibility
1. Increase browser zoom for hex view
2. Collapse unused tree nodes
3. Resize browser window to preferred aspect ratio

## Future Enhancements

Possible improvements:
1. **Resizable panels**: Drag divider to adjust widths
2. **Panel toggle**: Hide hex or tree view temporarily
3. **Split orientations**: Horizontal or vertical split
4. **Full-screen panels**: Expand one panel to full screen
5. **Pinned dialog**: Keep dialog open while viewing results
6. **Quick actions**: Toolbar buttons for common operations
7. **View presets**: Save/load favorite layouts
8. **Dark mode**: Toggle dark theme
9. **Compact mode**: Reduce spacing for more content

## Migration from Previous UI

### What Changed
- **Removed**: Three-column layout with input on left
- **Added**: Modal dialog for input
- **Added**: Floating action button
- **Added**: Full-screen layout
- **Changed**: Two-column grid instead of three

### What Stayed Same
- All functionality preserved
- Same color scheme
- Same keyboard shortcuts
- Same interaction patterns
- Same data visualization

### Benefits
- More screen space for content
- Better focus on decoded data
- Cleaner visual hierarchy
- Easier mobile experience
- Professional appearance

## Summary

The new full-screen UI maximizes viewing area for the decoded ASN.1 structure and hex view. The modal input dialog keeps the interface clean while remaining easily accessible via the floating action button. Independent scrolling and proper overflow handling ensure a smooth, professional experience across all devices.
