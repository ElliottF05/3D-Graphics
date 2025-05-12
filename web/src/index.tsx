import React from 'react';
import ReactDOM from 'react-dom/client';
import * as wasm from '../wasm/wasm_graphics'

import App from './react_components/App'

import './index.css'

// Create a root and render the App component
const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);

// WASM-JS Bridge
if (!(window as any).wasmBridge) {
    (window as any).wasmBridge = {};
}

(window as any).wasmBridge.jsSetIsObjectSelected = (isSelected: boolean) => {
    console.warn("WASM tried to update selection, but React component bridge isn't fully set up yet or was unmounted.");
};


// Helper functions to interface with the WebAssembly module
async function loadGlbModel(url: string) {
    try {
        const response = await fetch(url);
        const glbBuffer = await response.arrayBuffer();
        const glbBytes = new Uint8Array(glbBuffer);
        
        // Call the Wasm function with the single GLB file
        const success = wasm.load_glb_model(glbBytes);
        return success;
    } catch (error) {
        console.error("Error loading GLB model:", error);
        return false;
    }
}

// Initialize and start the game loop
await wasm.default();
// await wasm.initThreadPool(navigator.hardwareConcurrency);
wasm.init_and_begin_game_loop();

// loadGltfModelFromBaseUrl("../static/goose_low_poly_gltf/scene");
// loadGltfModelFromBaseUrl("../static/medieval_fantasy_book/scene");
// loadGlbModel("../static/medieval_fantasy_book.glb");
// loadGlbModel("../static/low_poly_forest.glb");