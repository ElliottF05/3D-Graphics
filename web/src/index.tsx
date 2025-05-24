import React from 'react';
import ReactDOM from 'react-dom/client';
import * as wasm from '../wasm/wasm_graphics'

import App from './react_components/App'

import './index.css'

// Initialize wasm module
await wasm.default();
await wasm.initThreadPool(navigator.hardwareConcurrency);
// Note: defer wasm.init_and_begin_game_loop() to MainCanvas, so 
// we can ensure the canvas is ready and the context is set up.

// Create a root and render the App component
const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);

// Helper functions to interface with the WebAssembly module
export async function loadGlbModel(url: string) {
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
};

export async function getGlbBytes(url: string): Promise<Uint8Array | null> {
    try {
        const response = await fetch(url);
        const glbBuffer = await response.arrayBuffer();
        const glbBytes = new Uint8Array(glbBuffer);
        return glbBytes;
    } catch (error) {
        console.error("Error fetching GLB bytes:", error);
        return null;
    }
}

// https://sketchfab.com/3d-models/medieval-fantasy-book-06d5a80a04fc4c5ab552759e9a97d91a
// loadGlbModel("../static/medieval_fantasy_book.glb");

// https://sketchfab.com/3d-models/magical-help-73fcb7197ba441419c768105c7db5d17
// loadGlbModel("../static/magical_help.glb");

// https://sketchfab.com/3d-models/this-tree-is-growing-60a1b5a73e184c8db7aa6007cd9d3462
// loadGlbModel("../static/this_tree_is_growing.glb");