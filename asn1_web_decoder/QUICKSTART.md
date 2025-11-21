# Quick Start Guide

Get the ASN.1 Web Decoder running in 3 easy steps!

## Step 1: Build

```bash
./build.sh
```

This compiles the Rust library to WebAssembly and generates all necessary files.

**What it does:**
- Checks for wasm-pack (installs if needed)
- Compiles Rust code to WebAssembly
- Generates JavaScript bindings
- Outputs to `www/pkg/` directory

**Time:** ~10-30 seconds (first build may take longer)

## Step 2: Serve

```bash
cd www
./serve.sh
```

Or use any static file server:
```bash
cd www
python3 -m http.server 8080
# or
npx serve
# or
php -S localhost:8080
```

## Step 3: Use

1. Open browser to **http://localhost:8080**
2. Click **"Load Sample"** to load example data
3. Click **"Decode"** to see the ASN.1 structure
4. Click tree nodes to expand/collapse

## Testing with Real Data

### Generate a Certificate
```bash
openssl req -x509 -newkey rsa:2048 -keyout key.pem -out cert.pem \
  -days 365 -nodes -subj "/C=US/ST=CA/O=Test/CN=example.com"
```

### Decode It
1. Copy contents of `cert.pem`
2. Paste into the web form
3. Click "Decode"

### Or Use Existing Certificates
```bash
# From a website
echo | openssl s_client -connect google.com:443 2>/dev/null | \
  openssl x509 -outform PEM > google-cert.pem

# Then paste the content into the web form
cat google-cert.pem
```

## Keyboard Shortcut

Press **Ctrl + Enter** in the text area to decode without clicking the button.

## Troubleshooting

### Build fails
- Ensure Rust is installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Check internet connection (for downloading dependencies)

### Server won't start
- Port 8080 might be in use, try a different port: `python3 -m http.server 8081`

### Page loads but decode fails
- Check browser console (F12) for errors
- Verify `www/pkg/` directory exists and contains `.wasm` file
- Try refreshing the page

### Invalid PEM error
- Ensure input starts with `-----BEGIN...` and ends with `-----END...`
- Check that base64 content is not corrupted
- Try the "Load Sample" button first

## Features to Explore

- **Expand All**: Click multiple nodes to expand the tree structure
- **Value Display**: See decoded integers, strings, OIDs, times
- **Badge Colors**: Different colors for tag classes and construction types
- **Clear Button**: Reset everything to start fresh
- **Sample Data**: Pre-loaded example for testing

## What Can You Decode?

- ✅ X.509 Certificates (most common)
- ✅ RSA/EC Private Keys
- ✅ Public Keys
- ✅ Certificate Signing Requests (CSRs)
- ✅ Any PEM-formatted ASN.1 structure

## Next Steps

- Read [README.md](README.md) for detailed documentation
- Check [DEMO.md](DEMO.md) for usage examples
- See [DEPLOYMENT.md](DEPLOYMENT.md) for hosting options

## File Structure After Build

```
www/
├── index.html          # Main page
├── styles.css          # Styling
├── app.js              # JavaScript logic
├── serve.sh            # Server script
└── pkg/                # Generated files (don't edit)
    ├── asn1_web_decoder_bg.wasm  # WebAssembly binary
    ├── asn1_web_decoder.js       # JS bindings
    └── asn1_web_decoder.d.ts     # TypeScript definitions
```

## Pro Tips

1. **Bookmark sample sites** that need certificate checking
2. **Use DevTools** (F12) to inspect the decoded JSON structure
3. **Copy certificate chains** from browsers to analyze multiple certificates
4. **Check validity dates** in the UTCTime/GeneralizedTime fields
5. **Look for OIDs** to understand certificate purposes and extensions

## Support

If you encounter issues:
1. Check browser console for errors
2. Verify build completed successfully
3. Ensure all files in `www/pkg/` exist
4. Try a different browser
5. Rebuild with `./build.sh`

---

**Enjoy decoding ASN.1 structures!** 🎉
