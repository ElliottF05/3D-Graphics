import wasm from "vite-plugin-wasm";
import fs from "fs";
import { defineConfig } from "vite";
export default defineConfig({
  base: './',
  esbuild: {
    supported: {
      'top-level-await': true //browsers can handle top-level-await features
      },
    },
  plugins: [
    wasm(),
    {
      name: "isolation",
      configureServer(server) {
        server.middlewares.use((_req, res, next) => {
          res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
          res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
          next();
        });
      },
    },
  ],
  worker: {
    '@vite-ignore': true,
    plugins: [
        wasm(),
    ]
  },
  optimizeDeps: {
    exclude: ['3D-Graphics', '3D-Graphics.js', 'enable-threads.js']
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          threads: ['./src/enable-threads.js']
        }
      }
    },
    outDir: 'dist',
    }
});