import React, { createContext, useContext, useState, useEffect, ReactNode, useCallback } from 'react';
import { IWasmToJsBridge, wasmToJsBridge } from '@/wasmToJsBridge'; // Adjust path

export type GameStatus = 'Rasterizing' | 'Editing' | 'RayTracing';

interface IGameContext {
    // State variables
    isObjectSelected: boolean;
    selectionVersion: number;
    gameStatus: GameStatus;
    // Add other shared states here, e.g., selectedObjectProperties, rayTraceProgress

    // Actions callable from React components (which might then call WASM)
    enterEditMode: () => void;
    exitEditModeAndConfirm: () => void; // Example: combines UI and WASM calls
    enterRayTraceMode: () => void;
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
    const [isObjectSelected, setIsObjectSelected] = useState<boolean>(false);
    const [selectionVersion, setSelectionVersion] = useState<number>(0);
    const [gameStatus, setGameStatus] = useState<GameStatus>('Rasterizing'); // Default mode

    // Setup the WASM to JS bridge implementations
    useEffect(() => {
        const bridge: IWasmToJsBridge = {
            updateObjectSelectionStatus: (isSelected) => {
                console.log("GameProvider: Bridge updating object selection to", isSelected);
                setIsObjectSelected(isSelected);
                if (isSelected) {
                    setSelectionVersion(prev => prev + 1);
                } else {
                    // Optionally, you could also increment version on deselect if some
                    // components need to react specifically to deselection via version.
                    // Or reset/clear other dependent states here.
                }
            },
            updateGameStatus: (newStatus) => {
                console.log("GameProvider: Bridge updating game status to", newStatus);
                setGameStatus(newStatus);
                // If entering edit mode, ensure no object is selected initially in UI
                if (newStatus === 'Editing') {
                    setIsObjectSelected(false); 
                }
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
        isObjectSelected,
        selectionVersion,
        gameStatus,
        enterEditMode,
        exitEditModeAndConfirm,
        enterRayTraceMode,
    };

    return <GameContext.Provider value={value}>{children}</GameContext.Provider>;
};