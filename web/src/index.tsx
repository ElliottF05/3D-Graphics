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
};

loadGlbModel("../static/medieval_fantasy_book.glb");
// loadGlbModel("../static/low_poly_forest.glb");