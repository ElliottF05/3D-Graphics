#!/bin/bash

# Step 1: Build Rust WASM with Debug Symbols (-g)
RUSTFLAGS="-C debuginfo=2" cargo build --release --target wasm32-unknown-unknown

# Step 2: Generate JavaScript & TypeScript Bindings (first, without optimization)
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_graphics.wasm \
  --out-dir ../web/wasm \
  --target web \
  --typescript

# Step 3: NOW Optimize WebAssembly after wasm-bindgen (fixes alignment issues)
wasm-opt -O4 -g -o ../web/wasm/wasm_graphics_bg.wasm ../web/wasm/wasm_graphics_bg.wasm

echo "✅ WASM build completed using wasm-bindgen!"