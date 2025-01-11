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
var running: boolean = true;
var raytracing: boolean = false;
var startYRayTracing = 0;
async function loop(): Promise<void> {
    let readyForNextFrame: boolean = true;
    while (true) {
        readyForNextFrame = false;
        setTimeout(() => {
            readyForNextFrame = true;
        },40);
        Input.processInput();
        console.log("raytracing = ", raytracing, ", running = ", running);
        if (running) {
            console.log("Main.ts: rendering scene");
            CPPInterface.CPPrenderScene();
            // CPPInterface.CPPrenderSceneRayTracing(0);
        }
        if (raytracing && !running) {
            console.log("raytracing!!!");
            startYRayTracing = CPPInterface.CPPrenderSceneRayTracing(startYRayTracing);
            // console.log(startIndexRayTracing);
            if (startYRayTracing < 0) {
                startYRayTracing = 0;
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
    startYRayTracing = 0;
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
    console.log("Main.beginRayTracing()");
    raytracing = true;
    running = false;
    console.log("raytracing = ", raytracing, ", running = ", running);
}
export function isRayTracing(): boolean {
    return raytracing;
}

// Set the scene and start the main loop
Graphics.setCanvasImage();
loop();