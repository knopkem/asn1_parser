# Deployment Guide

## Overview

The ASN.1 Web Decoder is a static website that can be deployed to any static hosting service. After building, the `www/` directory contains everything needed to run the application.

## Prerequisites

- Built WebAssembly module (run `./build.sh` first)
- The `www/` directory with the following structure:
  ```
  www/
  ├── index.html
  ├── styles.css
  ├── app.js
  └── pkg/
      ├── asn1_web_decoder_bg.wasm
      ├── asn1_web_decoder.js
      └── asn1_web_decoder.d.ts
  ```

## Deployment Options

### 1. GitHub Pages

**Method 1: Automated via GitHub Actions (Recommended)**

This repository includes a GitHub Actions workflow that automatically builds and deploys:

1. Enable GitHub Pages in your repository:
   - Go to Settings → Pages
   - Under "Build and deployment", select "GitHub Actions" as the source

2. The workflow (`.github/workflows/deploy-pages.yml`) will:
   - Trigger on pushes to `main` branch
   - Install Rust and wasm-pack
   - Build the WebAssembly module
   - Deploy to GitHub Pages

3. Access your site at `https://<username>.github.io/<repository>/`

**Method 2: Manual deployment using gh-pages branch**
```bash
# From the asn1_web_decoder directory
cd www
git init
git add .
git commit -m "Initial deployment"
git branch -M gh-pages
git remote add origin https://github.com/USERNAME/REPO.git
git push -u origin gh-pages
```

**Method 3: Using docs/ folder**
```bash
# From the repository root
cp -r asn1_web_decoder/www ./docs
git add docs/
git commit -m "Add ASN.1 decoder docs"
git push

# Then enable GitHub Pages in repository settings, pointing to /docs
```

### 2. Netlify

**Option A: Drag and Drop**
1. Go to https://app.netlify.com
2. Drag the `www/` folder to the deployment area
3. Your site will be live instantly

**Option B: Netlify CLI**
```bash
npm install -g netlify-cli
cd www
netlify deploy --prod
```

**Option C: Git Integration**
Create a `netlify.toml` in the repository root:
```toml
[build]
  base = "asn1_web_decoder"
  command = "./build.sh"
  publish = "www"

[[headers]]
  for = "/*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
```

### 3. Vercel

```bash
npm install -g vercel
cd www
vercel --prod
```

Or create `vercel.json` in the repository root:
```json
{
  "buildCommand": "cd asn1_web_decoder && ./build.sh",
  "outputDirectory": "asn1_web_decoder/www",
  "headers": [
    {
      "source": "/(.*).wasm",
      "headers": [
        {
          "key": "Content-Type",
          "value": "application/wasm"
        }
      ]
    }
  ]
}
```

### 4. AWS S3 + CloudFront

```bash
# Create S3 bucket
aws s3 mb s3://asn1-decoder

# Upload files
cd www
aws s3 sync . s3://asn1-decoder --acl public-read

# Set WASM MIME type
aws s3 cp pkg/ s3://asn1-decoder/pkg/ \
  --recursive \
  --exclude "*" \
  --include "*.wasm" \
  --content-type "application/wasm" \
  --acl public-read

# Enable static website hosting
aws s3 website s3://asn1-decoder \
  --index-document index.html \
  --error-document index.html
```

### 5. Firebase Hosting

```bash
npm install -g firebase-tools
firebase login
firebase init hosting

# Choose www as public directory
# Configure as single-page app: No

# Deploy
firebase deploy --only hosting
```

Create `firebase.json`:
```json
{
  "hosting": {
    "public": "www",
    "ignore": [
      "firebase.json",
      "**/.*",
      "**/node_modules/**"
    ],
    "headers": [
      {
        "source": "**/*.wasm",
        "headers": [
          {
            "key": "Content-Type",
            "value": "application/wasm"
          }
        ]
      }
    ]
  }
}
```

### 6. Cloudflare Pages

**Via Dashboard:**
1. Go to Cloudflare Pages dashboard
2. Connect your GitHub repository
3. Set build command: `cd asn1_web_decoder && ./build.sh`
4. Set build output directory: `asn1_web_decoder/www`
5. Deploy

**Via Wrangler CLI:**
```bash
npm install -g wrangler
cd www
wrangler pages publish . --project-name asn1-decoder
```

### 7. Apache Server

Upload the `www/` directory to your server and ensure `.htaccess` includes:

```apache
# .htaccess
AddType application/wasm .wasm

<IfModule mod_headers.c>
    <FilesMatch "\.wasm$">
        Header set Content-Type "application/wasm"
    </FilesMatch>
</IfModule>
```

### 8. Nginx

Add to your nginx configuration:

```nginx
server {
    listen 80;
    server_name your-domain.com;
    root /path/to/www;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location ~ \.wasm$ {
        add_header Content-Type application/wasm;
    }

    # Optional: Enable gzip compression
    gzip on;
    gzip_types application/wasm application/javascript text/css text/html;
}
```

### 9. Docker

Create `Dockerfile` in the `asn1_web_decoder` directory:

```dockerfile
FROM rust:1.75 as builder

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /app
COPY . .
RUN ./build.sh

FROM nginx:alpine
COPY --from=builder /app/www /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
```

Create `nginx.conf`:
```nginx
server {
    listen 80;
    root /usr/share/nginx/html;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location ~ \.wasm$ {
        add_header Content-Type application/wasm;
    }

    gzip on;
    gzip_types application/wasm application/javascript text/css;
}
```

Build and run:
```bash
docker build -t asn1-decoder .
docker run -p 8080:80 asn1-decoder
```

## Important Notes

### WASM MIME Type

Most hosting services automatically set the correct MIME type for `.wasm` files, but if you encounter issues, ensure your server sends:
```
Content-Type: application/wasm
```

### CORS Headers

If you host the WASM file on a different domain than the HTML, you'll need CORS headers:
```
Access-Control-Allow-Origin: *
```

### Single Page Application

This is not a SPA (no routing), so no special configuration is needed for URL handling.

### HTTPS

For production deployments, always use HTTPS. Most hosting services provide free SSL certificates via Let's Encrypt.

## Testing Deployment

After deployment, test your site:

1. Open the deployed URL
2. Click "Load Sample" button
3. Click "Decode"
4. Verify the tree view appears with decoded structure

## Troubleshooting

### WASM fails to load
- Check browser console for errors
- Verify WASM MIME type is correct
- Check file permissions (should be readable)
- Ensure file path is correct (check DevTools Network tab)

### Blank page
- Check browser console for JavaScript errors
- Verify all files are uploaded
- Check index.html loads correctly

### Decode fails
- Check browser console for errors
- Verify the WASM module initialized
- Test with sample data first

## Performance Optimization

### Enable Compression
Most hosting services support gzip/brotli compression. Enable it for:
- `.wasm` files
- `.js` files
- `.css` files
- `.html` files

### CDN
Consider using a CDN for global distribution:
- Cloudflare (free tier available)
- AWS CloudFront
- Fastly
- BunnyCDN

### Caching
Set appropriate cache headers:
```
Cache-Control: public, max-age=31536000, immutable
```
For versioned assets (WASM, JS), use long cache times.

## Monitoring

Consider adding analytics:
- Google Analytics
- Plausible Analytics
- Fathom Analytics

Example for Plausible (add to `index.html`):
```html
<script defer data-domain="your-domain.com" src="https://plausible.io/js/script.js"></script>
```

## Updating

To update the deployment:
1. Make changes to Rust or frontend code
2. Run `./build.sh`
3. Deploy the `www/` directory again

Most hosting services support automatic deployments from Git.
