import path from "path"
import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

// https://vite.dev/config/
export default defineConfig({
    plugins: [react(), tailwindcss()],
    build: {
        target: 'esnext',
    },
    base: "./",
    server: {
        headers: {
            'Cross-Origin-Embedder-Policy': 'require-corp',
            'Cross-Origin-Opener-Policy': 'same-origin',
            'Cross-Origin-Resource-Policy': 'cross-origin',
        }

    },
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
        "@wasm": path.resolve(__dirname, "./wasm"),
      },
    },
    worker: {
        format: "es"
    }
  })