/**
 * Defines the contract for functions that WASM can call on the JavaScript side.
 * These functions will typically update React state (e.g., via a Context).
 */
export interface IWasmToJsBridge {
    updateObjectSelectionStatus: (isSelected: boolean) => void;
    updateGameStatus: (newStatus: 'Rasterizing' | 'Editing' | 'RayTracing') => void;
}

/**
 * The global bridge object that WASM will interact with.
 * Its methods will be implemented by our React Context provider to update UI state.
 */
class WasmToJsBridge implements IWasmToJsBridge {
    public updateObjectSelectionStatus: (isSelected: boolean) => void = (isSelected) => {
        console.warn("WasmToJsBridge.updateObjectSelectionStatus called before React context initialized it.", isSelected);
    };
    public updateGameStatus: (newStatus: 'Rasterizing' | 'Editing' | 'RayTracing') => void = (newStatus) => {
        console.warn("WasmToJsBridge.updateGameStatus called before React context initialized it.", newStatus);
    };

    // Implement other methods with default warnings
}

// Make the bridge instance globally accessible for WASM.
if (!(window as any).wasmToJsBridge) {
    (window as any).wasmToJsBridge = new WasmToJsBridge();
}

// Export the instance type if needed elsewhere in JS/TS, though WASM interacts via the global.
export const wasmToJsBridge = (window as any).wasmToJsBridge as IWasmToJsBridge;

// To make this globally available for `declare global` in other files if you choose that route for typing window:
export {};