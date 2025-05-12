import * as wasm from "@wasm/wasm_graphics";
import React, { useEffect, useRef } from 'react'

const linkCanvasToWasm = (canvas: HTMLCanvasElement | null) => {
    if (!canvas) {
        console.error("MainCanvas: Canvas element ref is null.");
        return;
    }
    const ctx = canvas.getContext('2d');
    if (!ctx) {
        console.error("MainCanvas: Could not get 2D context for canvas.");
        return;
    }

    (window as any).drawWasmPixelsToCanvas = (
        wasmPixelData: Uint8Array,
        width: number,
        height: number,
    ) => {

        // Ensure canvas dimensions match the data dimensions
        if (canvas.width !== width || canvas.height !== height) {
            console.warn(`Canvas dimensions (${canvas.width}x${canvas.height}) differ from data (${width}x${height})`);
        }

        const clampedArray = new Uint8ClampedArray(wasmPixelData.length);
        clampedArray.set(wasmPixelData);

        try {
            const imageData = new ImageData(clampedArray, width, height);
            ctx.putImageData(imageData, 0, 0);
        } catch (error) {
            console.error("MainCanvas: Error creating or putting ImageData:", error);
        }
    };

    // Now that drawWasmPixelsToCanvas is set up, initialize WASM and game loop
    try {
        console.log("MainCanvas: Initializing game loop...");
        wasm.init_and_begin_game_loop();
        console.log("MainCanvas: Game loop initiated.");
    } catch (error) {
        console.error("MainCanvas: Error initializing WASM game loop:", error);
    }
}

const MainCanvas = () => {
    console.log("MainCanvas render");

    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        linkCanvasToWasm(canvas);

    }, []); // Empty dependency array: run once after initial render and clean up on unmount

    return (
        <canvas 
            ref={canvasRef}
            width={500}
            height={500}
            id={"main-canvas"} 
            className="block bg-neutral-800 aspect-square max-w-full max-h-full shadow-lg"
        ></canvas>
    );
}

export default MainCanvas;