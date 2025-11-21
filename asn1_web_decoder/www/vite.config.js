import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  base: '/asn1_parser/',
  server: {
    port: 8080
  }
})
