# ASN.1 Web Decoder Demo

## Quick Start

1. **Build the project:**
   ```bash
   ./build.sh
   ```

2. **Start the local server:**
   ```bash
   cd www
   ./serve.sh
   ```

3. **Open your browser to:** http://localhost:8080

## Testing the Application

### Option 1: Use the Sample Button
Click the "Load Sample" button to load a pre-configured sample certificate.

### Option 2: Generate Your Own Certificate
```bash
openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem -days 365 -nodes \
  -subj "/C=US/ST=California/L=San Francisco/O=Test Org/CN=example.com"

# Then paste the contents of cert.pem into the web form
cat cert.pem
```

### Option 3: Use an Existing Certificate
You can decode any PEM-formatted certificate or key:
- SSL/TLS certificates from websites
- Private keys
- CSRs (Certificate Signing Requests)
- Public keys

## What You'll See

After clicking "Decode", you'll see an interactive tree view showing:

### Root Level
```
PEM: CERTIFICATE [PEM] [Length: 911]
```

### Nested Structure
The decoder will show:
- **Tag Types**: SEQUENCE, SET, INTEGER, OID, etc.
- **Tag Classes**: UNIVERSAL, CONTEXT, APPLICATION, PRIVATE
- **Construction**: CONSTRUCTED or PRIMITIVE
- **Lengths**: Size in bytes
- **Values**: Decoded content for primitive types

### Example Output
```
├─ SEQUENCE (Tag 16) [UNIVERSAL] [CONSTRUCTED] [Length: 907]
   ├─ SEQUENCE (Tag 16) [UNIVERSAL] [CONSTRUCTED] [Length: 627]
   │  ├─ [CONTEXT] Tag 0 [CONSTRUCTED] [Length: 3]
   │  │  └─ INTEGER (Tag 2) [UNIVERSAL] [PRIMITIVE] [Length: 1]
   │  │     Value: 2
   │  ├─ INTEGER (Tag 2) [UNIVERSAL] [PRIMITIVE] [Length: 20]
   │  │  Value: 0x709bcf9834d33d0ae5aa50d5e3cf6d0ca5509101
   │  ├─ SEQUENCE (Tag 16) [UNIVERSAL] [CONSTRUCTED] [Length: 13]
   │  │  ├─ OBJECT IDENTIFIER (Tag 6) [UNIVERSAL] [PRIMITIVE] [Length: 9]
   │  │  │  Value: OID: 1.2.840.113549.1.1.11
   │  │  └─ NULL (Tag 5) [UNIVERSAL] [PRIMITIVE] [Length: 0]
   │  │     Value: NULL
```

## Features to Try

1. **Expand/Collapse Nodes**: Click on any node with children to expand or collapse it
2. **View Values**: See decoded values for primitive types (integers, strings, OIDs)
3. **Color Coding**: Different badge colors for tag classes and construction types
4. **Keyboard Shortcut**: Press Ctrl+Enter in the text area to decode

## Common ASN.1 Elements in Certificates

- **Version**: INTEGER (usually 2 for X.509v3)
- **Serial Number**: Large INTEGER
- **Signature Algorithm**: OID (e.g., 1.2.840.113549.1.1.11 for SHA256WithRSA)
- **Issuer/Subject**: Distinguished Name (DN) as nested SEQUENCEs and SETs
- **Validity**: UTCTime or GeneralizedTime
- **Public Key**: BIT STRING within a SEQUENCE
- **Extensions**: Context-specific tagged elements

## Troubleshooting

### Error: "Failed to parse PEM"
- Ensure the input starts with `-----BEGIN ...-----`
- Ensure the input ends with `-----END ...-----`
- Check that the base64 content is not corrupted

### Error: "Unexpected end of data"
- The PEM content may be truncated
- Try copying the complete certificate including all lines

### No Output
- Check browser console (F12) for JavaScript errors
- Ensure the WebAssembly module loaded correctly
- Try refreshing the page

## Browser Compatibility

The application works in modern browsers with WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Deployment

To deploy to production:

1. **Build the project**: `./build.sh`
2. **Copy www directory** to your hosting service
3. Ensure your server serves `.wasm` files with the correct MIME type: `application/wasm`

### Example Nginx Configuration
```nginx
location ~ \.wasm$ {
    add_header Content-Type application/wasm;
}
```

### GitHub Pages
Simply push the `www` directory contents to your `gh-pages` branch.

## Performance

- **WASM Module Size**: ~62KB (optimized)
- **Decode Time**: <10ms for typical certificates
- **No Server Required**: Everything runs in the browser

## Privacy

All decoding happens entirely in your browser. No data is sent to any server.
