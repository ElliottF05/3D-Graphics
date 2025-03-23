import React from 'react';
import ReactDOM from 'react-dom/client';
import * as wasm from '../wasm/wasm_graphics'

import App from './App.tsx'

// Create a root and render the App component
const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);

// Initialize and start the game loop
await wasm.default();
wasm.init_and_begin_game_loop();

// Helper functions to interface with the WebAssembly module
async function loadGltfModel(gltfUrl: string, binUrl: string) {
    console.log("Loading GLTF model from JS side");
    try {
        // Fetch the GLTF file
        const gltfResponse = await fetch(gltfUrl);
        const gltfBuffer = await gltfResponse.arrayBuffer();
        const gltfBytes = new Uint8Array(gltfBuffer);
        
        // Fetch the BIN file
        const binResponse = await fetch(binUrl);
        const binBuffer = await binResponse.arrayBuffer();
        const binBytes = new Uint8Array(binBuffer);

        console.log("GLTF and BIN files loaded");
        console.log("GLTF bytes:", gltfBytes);
        console.log("BIN bytes:", binBytes);

        // Call the Wasm function and get a simple success/failure
        const success = wasm.load_gltf_model(gltfBytes, binBytes);
        // const success = false; // Placeholder for actual Wasm function call

        if (success) {
            console.log("GLTF model loaded successfully");
            // Maybe trigger a redraw or update
        } else {
            console.error("Failed to load GLTF model");
        }
        
        return success;
    } catch (error) {
        console.error("Error loading GLTF model form JS side", error);
        return false;
    }
}

loadGltfModel("../static/goose_low_poly_gltf/scene.gltf", "../static/goose_low_poly_gltf/scene.bin");