# Changelog

## 2025-11-21 - UI Improvements

### Fixed
- **Replaced corrupt sample certificate** with a valid X.509 certificate
  - Old certificate had malformed base64 data
  - New certificate is properly signed and includes all standard fields
  - Generated with proper Subject/Issuer information

### Enhanced
- **Made tree view horizontally scrollable**
  - Added `overflow-x: auto` to `.tree-view` class
  - Added `overflow-y: auto` for vertical scrolling
  - Added `max-height: 600px` to prevent excessive vertical growth
  - Tree nodes now use `white-space: nowrap` to maintain formatting
  - Long OIDs, hex values, and nested structures now scroll horizontally

### CSS Changes
```css
.tree-view {
    overflow-x: auto;  /* Horizontal scroll for wide content */
    overflow-y: auto;  /* Vertical scroll */
    max-height: 600px; /* Limit height */
}

.tree-node {
    white-space: nowrap; /* Prevent text wrapping */
}
```

### Benefits
- Users can now view deeply nested ASN.1 structures without truncation
- Long hex values and OIDs are fully visible with horizontal scroll
- Sample certificate now successfully decodes and displays all fields
- Better UX for analyzing large certificates

### Testing
To test the improvements:
1. Load the sample certificate (click "Load Sample")
2. Click "Decode"
3. Verify the tree displays properly
4. Try scrolling horizontally if content is wide
5. Expand nested nodes to see deep structures
