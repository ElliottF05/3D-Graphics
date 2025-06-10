import * as wasm from '@wasm/wasm_graphics';
import React, { createContext, useContext, useState, useEffect, ReactNode, useCallback } from 'react';
import { IWasmToJsBridge, wasmToJsBridge, GameStatus } from '@/wasmToJSBridge'; // Adjust path
import { loadGlbModel } from '.';

interface IGameContext {
    // State variables
    selectedObjMatProps: wasm.MaterialProperties | null | undefined; 
    gameStatus: GameStatus;
    followCamera: boolean;
    fov: number; 
    focalDistance: number;
    dofStrength: number;
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
    const [gameStatus, setGameStatus] = useState<GameStatus>('Editing');
    const [followCamera, setFollowCamera] = useState<boolean>(false);
    const [fov, setFov] = useState<number>(90);
    const [focalDistance, setFocalDistance] = useState<number>(10.0);
    const [dofStrength, setDofStrength] = useState<number>(0.0);

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
            updateFollowCamera: (follow) => {
                console.log("GameProvider: Bridge updating follow camera status", follow);
                setFollowCamera(follow);
            },
            updateFov: (fov) => {
                console.log("GameProvider: Bridge updating FOV", fov);
                setFov(fov);
            },
            updateFocalDistance: (focalDistance) => {
                console.log("GameProvider: Bridge updating focal distance", focalDistance);
                setFocalDistance(focalDistance);
            },
            updateDofStrength: (defocusAngle) => {
                // defocus angle is range [0,0.05], need to map to [0,100]
                const dofStrength = defocusAngle * 2000;
                console.log("GameProvider: Bridge updating DOF strength", dofStrength, ", defocusAngle", defocusAngle);
                setDofStrength(dofStrength);
            },
            // getGlbBytes: async (url) => {
            //     console.log("GameProvider: Bridge loading GLB model from URL", url);
            //     try {
            //         const response = await fetch(url);
            //         const glbBuffer = await response.arrayBuffer();
            //         const glbBytes = new Uint8Array(glbBuffer);
            //         return glbBytes;
            //     } catch (error) {
            //         console.error("GameProvider: Bridge failed to load GLB model", error);
            //         return new Uint8Array();
            //     }
            // },
            // Implement other bridge functions here to update context state
        };

        // Assign our bridge implementations to the global object
        Object.assign(wasmToJsBridge, bridge);

        // Cleanup (optional, if you need to reset bridge functions on unmount)
        // return () => {
        //     // Reset bridge functions to defaults if necessary
        // };
    }, []); // Empty dependency array: setup bridge once on mount

    const value: IGameContext = {
        selectedObjMatProps,
        gameStatus,
        followCamera,
        fov,
        focalDistance,
        dofStrength,
    };

    return <GameContext.Provider value={value}>{children}</GameContext.Provider>;
};