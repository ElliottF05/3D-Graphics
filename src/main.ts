import * as CPPInterface from './cppInterface.ts';
import * as Graphics from './graphics.ts';
import * as Input from './input.ts';
import './style.css';

import './database.ts';

while (!CPPInterface.CPPmoduleInitialized) {
    console.log("Waiting for CPPinterface to be initialized...");
    await new Promise(r => setTimeout(r, 10));
}

console.log("Hello from main.ts!");

// Main game/simulation loop
let running: boolean = true;
let raytracing: boolean = false;
let startIndexRayTracing = 0;
async function loop(): Promise<void> {
    let readyForNextFrame: boolean = true;
    while (true) {
        console.log(running, raytracing);
        readyForNextFrame = false;
        setTimeout(() => {
            readyForNextFrame = true;
        },40);
        Input.processInput();
        if (running) {
            CPPInterface.CPPrenderScene();
            // CPPInterface.CPPrenderSceneRayTracing(0);
        }
        if (raytracing && !running) {
            console.log("raytracing!!!");
            startIndexRayTracing = CPPInterface.CPPrenderSceneRayTracing(startIndexRayTracing);
            console.log(startIndexRayTracing);
            if (startIndexRayTracing < 0) {
                startIndexRayTracing = 0;
                raytracing = false;
            }
        }
        Graphics.setCanvasImage();
        // Wait for 40ms before the next frame
        while (!readyForNextFrame) {
            await new Promise(r => setTimeout(r, 1));
        };
    }
}

export function pause(): void {
    running = false;
}
export function unpause(): void {
    running = true;
    raytracing = false;
}
export function isRunning(): boolean {
    return running;
}
export function beginRayTracing() {
    console.log("beginning ray tracing");
    raytracing = true;
    running = false;
}
export function isRayTracing(): boolean {
    return raytracing;
}

// Set the scene and start the main loop
Graphics.setCanvasImage();
loop();