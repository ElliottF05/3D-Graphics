/**
 * Defines the contract for functions that WASM can call on the JavaScript side.
 * These functions will typically update React state (e.g., via a Context).
 */
export interface IWasmToJsBridge {
    // Called by WASM when an object's selection status changes.
    updateObjectSelectionStatus: (isSelected: boolean) => void;

    // Called by WASM when the game's overall mode changes (e.g., entering/exiting edit mode).
    updateGameMode: (newMode: 'Normal' | 'Editing' | 'RayTracing') => void;

    // Example: If WASM needs to send specific data for the selected object
    // updateSelectedObjectData: (data: any) => void;

    // Add more functions here as your application needs them.
    // For example, if WASM needs to report progress on a long task:
    // reportRayTraceProgress: (progress: number) => void;
}

/**
 * The global bridge object that WASM will interact with.
 * Its methods will be implemented by our React Context provider to update UI state.
 */
class WasmToJsBridge implements IWasmToJsBridge {
    public updateObjectSelectionStatus: (isSelected: boolean) => void = (isSelected) => {
        console.warn("WasmToJsBridge.updateObjectSelectionStatus called before React context initialized it.", isSelected);
    };

    public updateGameMode: (newMode: 'Normal' | 'Editing' | 'RayTracing') => void = (newMode) => {
        console.warn("WasmToJsBridge.updateGameMode called before React context initialized it.", newMode);
    };

    // Implement other methods with default warnings
}

// Make the bridge instance globally accessible for WASM.
// Choose a unique name to avoid conflicts.
if (!(window as any).myAppWasmBridge) {
    (window as any).myAppWasmBridge = new WasmToJsBridge();
}

// Export the instance type if needed elsewhere in JS/TS, though WASM interacts via the global.
export const myAppWasmBridge = (window as any).myAppWasmBridge as IWasmToJsBridge;

// To make this globally available for `declare global` in other files if you choose that route for typing window:
export {};