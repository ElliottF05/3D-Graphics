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

/**
 * Loads a GLTF model from the specified URLs.
 * @param gltfUrl The URL of the GLTF file.
 * @param binUrl The URL of the BIN file.
 * @returns A promise that resolves when the model is loaded.
 */
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

/**
 * Loads a GLTF model from a base URL.
 * @param baseUrl The base URL for the GLTF and BIN files, without the file extension.
 * GLTF url will be `${baseUrl}.gltf`. BIN url will be `${baseUrl}.bin`
 * @returns A promise that resolves when the model is loaded.
 */
async function loadGltfModelFromBaseUrl(baseUrl: string) {
    console.log("Loading GLTF model from base URL");
    const gltfUrl = `${baseUrl}.gltf`;
    const binUrl = `${baseUrl}.bin`;
    return loadGltfModel(gltfUrl, binUrl);
}

async function loadGlbModel(url) {
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

// loadGltfModelFromBaseUrl("../static/goose_low_poly_gltf/scene");
// loadGltfModelFromBaseUrl("../static/medieval_fantasy_book/scene");
loadGlbModel("../static/medieval_fantasy_book.glb");