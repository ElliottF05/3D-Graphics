#!/bin/bash

# Step 1: Build Rust WASM with Debug Symbols (-g)

# --- Pre-rayon version ---
# RUSTFLAGS='-C debuginfo=2 --cfg getrandom_backend="wasm_js"' cargo build --release --target wasm32-unknown-unknown
# --- End pre-rayon version ---

# --- Rayon version ---
echo "Building WASM with Rayon support..."
RUSTFLAGS='-C debuginfo=2 --cfg getrandom_backend="wasm_js" -C target-feature=+atomics,+bulk-memory -C link-arg=--max-memory=4294967296' \
  rustup run nightly-2024-08-02 \
  cargo build --release --target wasm32-unknown-unknown -Z build-std=panic_abort,std
# --- End rayon version ---

# Check if cargo build was successful
if [ $? -ne 0 ]; then
    echo "❌ Cargo build failed"
    exit 1
fi
echo "✅ Cargo build successful"


# Step 2: Generate JavaScript & TypeScript Bindings
echo "Running wasm-bindgen..."
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_graphics.wasm \
  --out-dir ../web/wasm \
  --target web \
  --typescript

if [ $? -ne 0 ]; then
    echo "❌ wasm-bindgen failed"
    exit 1
fi
echo "✅ wasm-bindgen successful"

# Step 3: Optimize WebAssembly after wasm-bindgen
if command -v wasm-opt &> /dev/null
then
    echo "Found wasm-opt. Optimizing WebAssembly..."
    wasm-opt -O4 -g -o ../web/wasm/wasm_graphics_bg.wasm ../web/wasm/wasm_graphics_bg.wasm

    if [ $? -ne 0 ]; then
        echo "⚠️ wasm-opt failed, but continuing without optimization."
    else
        echo "✅ wasm-opt successful"
    fi
else
    echo "ℹ️ wasm-opt not found. Skipping WebAssembly optimization."
fi

echo "🚀 WASM build completed!"