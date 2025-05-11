import * as wasm from '@wasm/wasm_graphics'

/**
 * Defines the contract for functions that WASM can call on the JavaScript side.
 * These functions will typically update React state (e.g., via a Context).
 */
export type GameStatus = 'Rasterizing' | 'Editing' | 'RayTracing';
export interface IWasmToJsBridge {
    updateSelectedObjMatProps: (props: wasm.MaterialProperties | null | undefined) => void;
    /**
     * @param newStatus 0 = Rasterizing, 1 = Editing, 2 = RayTracing
     */
    updateGameStatus: (newStatus: number) => void;
    updateFollowCamera: (follow: boolean) => void;
    updateFov: (fov: number) => void;
}

/**
 * The global bridge object that WASM will interact with.
 * Its methods will be implemented by our React Context provider to update UI state.
 */
class WasmToJsBridge implements IWasmToJsBridge {
    public updateSelectedObjMatProps: (props: wasm.MaterialProperties | null | undefined) => void = (props) => {
        console.warn("WasmToJsBridge.updateSelectedObjMatProps called before React context initialized it.", props);
    };
    public updateGameStatus: (newStatus: number) => void = (newStatus) => {
        console.warn("WasmToJsBridge.updateGameStatus called before React context initialized it.", newStatus);
    };
    public updateFollowCamera: (follow: boolean) => void = (follow) => {
        console.warn("WasmToJsBridge.updateFollowCamera called before React context initialized it.", follow);
    };
    public updateFov: (fov: number) => void = (fov) => {
        console.warn("WasmToJsBridge.updateFov called before React context initialized it.", fov);
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