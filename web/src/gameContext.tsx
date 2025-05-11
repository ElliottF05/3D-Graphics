import * as wasm from '@wasm/wasm_graphics';
import React, { createContext, useContext, useState, useEffect, ReactNode, useCallback } from 'react';
import { IWasmToJsBridge, wasmToJsBridge, GameStatus } from '@/wasmToJsBridge'; // Adjust path

interface IGameContext {
    // State variables
    selectedObjMatProps: wasm.MaterialProperties | null | undefined; 
    gameStatus: GameStatus;
    // Add other shared states here, e.g., selectedObjectProperties, rayTraceProgress

    // Actions callable from React components (which might then call WASM)
    // Add more actions as needed

}

const GameContext = createContext<IGameContext | undefined>(undefined);

export const useGameContext = () => {
    const context = useContext(GameContext);
    if (context === undefined) {
        throw new Error('useGameContext must be used within a GameProvider');
    }
    return context;
};

interface GameProviderProps {
    children: ReactNode;
}

export const GameProvider: React.FC<GameProviderProps> = ({ children }) => {
    const [selectedObjMatProps, setSelectedObjMatProps] = useState<wasm.MaterialProperties | null | undefined>(null);
    const [gameStatus, setGameStatus] = useState<GameStatus>('Rasterizing'); // Default mode

    // Setup the WASM to JS bridge implementations
    useEffect(() => {
        const bridge: IWasmToJsBridge = {
            updateSelectedObjMatProps: (props) => {
                console.log("GameProvider: Bridge updating selected object material properties", props);
                setSelectedObjMatProps(props);
            },
            updateGameStatus: (newStatus) => {
                let status: GameStatus;
                switch (newStatus) {
                    case 0:
                        status = 'Rasterizing';
                        break;
                    case 1:
                        status = 'Editing';     
                        break;
                    case 2:
                        status = 'RayTracing';
                        break;
                    default:
                        console.warn("GameProvider: Bridge received unknown game status", newStatus);
                        status = 'Rasterizing'; // Default fallback 
                }
                console.log("GameProvider: Bridge updating game status to", newStatus, status);
                setGameStatus(status);
                // TODO: should I also update selectedObjMatProps here?
            },
            // Implement other bridge functions here to update context state
        };

        // Assign our bridge implementations to the global object
        Object.assign(wasmToJsBridge, bridge);

        // Cleanup (optional, if you need to reset bridge functions on unmount)
        // return () => {
        //     // Reset bridge functions to defaults if necessary
        // };
    }, []); // Empty dependency array: setup bridge once on mount

    // Actions callable from UI components
    const enterEditMode = useCallback(() => {
        // wasm.enter_edit_mode(); // Call your WASM function
        console.log("Context: Requesting WASM to enter edit mode");
        // WASM should then call myAppWasmBridge.updateGameMode("Editing")
        // and myAppWasmBridge.updateObjectSelectionStatus(false)
        // For now, we can simulate the mode change if WASM isn't calling back yet
        // setAppMode('Editing');
        // setIsObjectSelected(false);
        // Call actual WASM function here:
        // wasm.enter_edit_mode(); // This function in wasm.rs should set game state
                                // and then call js_update_game_mode("Editing")
                                // and js_update_object_selection_status(false)
    }, []);

    const exitEditModeAndConfirm = useCallback(() => {
        // wasm.confirm_edits_and_exit_mode(); // Call your WASM function
        console.log("Context: Requesting WASM to confirm edits and exit edit mode");
        // WASM should then call myAppWasmBridge.updateGameMode("Normal")
        // and potentially update other states.
        // For now, simulate:
        // setAppMode('Normal');
        // setIsObjectSelected(false); // Usually after confirming, nothing is selected
        // wasm.confirm_edits(); // This is your existing function
        // wasm.exit_edit_mode(); // You'll need a new WASM function for this
    }, []);

    const enterRayTraceMode = useCallback(() => {
        // wasm.enter_ray_trace_mode(); // Call your WASM function
        console.log("Context: Requesting WASM to enter ray trace mode");
        // setAppMode('RayTracing');
    }, []);


    const value: IGameContext = {
        selectedObjMatProps,
        gameStatus,
    };

    return <GameContext.Provider value={value}>{children}</GameContext.Provider>;
};