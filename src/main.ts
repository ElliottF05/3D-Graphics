import * as CPPInterface from './cppInterface.ts';
import * as Graphics from './graphics.ts';
import * as Input from './input.ts';

import './database.ts';

while (!CPPInterface.CPPmoduleInitialized) {
    console.log("Waiting for CPPinterface to be initialized...");
    await new Promise(r => setTimeout(r, 10));
}

console.log("Hello from main.ts!");

// Main game/simulation loop
let running: boolean = true;
async function loop(): Promise<void> {
    let readyForNextFrame: boolean = true;
    while (true) {
        readyForNextFrame = false;
        setTimeout(() => {
            readyForNextFrame = true;
        },40);
        if (running) {
            Input.processInput();
            Graphics.setCanvasImage();
        }
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
}
export function isRunning(): boolean {
    return running;
}

// Set the scene and start the main loop
CPPInterface.CPPsetupScene();
loop();