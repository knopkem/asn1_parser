#!/bin/bash

echo "Starting local server on http://localhost:8080"
echo "Press Ctrl+C to stop"
echo ""

python3 -m http.server 8080
