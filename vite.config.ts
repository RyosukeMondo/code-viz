import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],

  // Vite options tailored for Tauri development
  clearScreen: false,

  // Tauri expects a fixed port, fail if that port is not available
  server: {
    host: process.env.VITE_HOST || '0.0.0.0', // Allow SSH tunnel access
    port: parseInt(process.env.VITE_PORT || '5180'),
    strictPort: true,
    watch: {
      // Ignore Rust build directories to prevent file watcher limit errors
      ignored: ['**/src-tauri/**', '**/target/**', '**/crates/**'],
    },
    // Proxy API requests to the Rust web backend server
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },

  // Path aliases
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
})
