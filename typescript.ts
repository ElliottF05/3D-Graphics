import { CPPgetBuffer, CPPsetupScene, CPPuserInput, CPPModule } from './index.js';
import { CPPgetSceneMetaDataSize, CPPgetSceneMetaDataBuffer, CPPgetScenePosDataBuffer, CPPgetSceneColorDataBuffer } from './index.js';

// DOM Canvas Elements
const canvas = document.getElementById('canvas') as HTMLCanvasElement;
const canvasContext = canvas.getContext('2d');

// Rendering loop variables
let running: boolean = true;

// User input variables
var pressedKeys: {} = {};
let mouseX: number = 0;
let mouseY: number = 0;
let mouseMultiplier: number = 1;
let pointerLocked: boolean = false;
window.onkeyup = function(e: KeyboardEvent) { pressedKeys[e.key] = false; }
window.onkeydown = function(e: KeyboardEvent) { pressedKeys[e.key] = true; }


// Begin the game/simulation loop
console.log("Setting up the scene...")
CPPsetupScene();
console.log("Scene setup complete. Beginning loop...");
loop();

// Main game/simulation loop
async function loop(): Promise<void> {
    let readyForNextFrame: boolean = true;
    while (true) {
        readyForNextFrame = false;
        setTimeout(() => {
            readyForNextFrame = true;
        },40);
        if (running) {
            processInput();
            setCanvasImage();
        }
        // Wait for 40ms before the next frame
        while (!readyForNextFrame) {
            await new Promise(r => setTimeout(r, 1));
        };
    }
}

// Set the canvas image to the image data from the C++ code
function setCanvasImage(): void {
    const imageData: ImageData = canvasContext.createImageData(500, 500);
    const buffer_data: number = CPPgetBuffer();
    var uint8array: Uint8ClampedArray = new Uint8ClampedArray(CPPModule.HEAPU8.buffer, buffer_data, 500 * 500 * 4);
    imageData.data.set(uint8array);
    canvasContext.putImageData(imageData, 0, 0);
}

// Process user input and send it to the C++ code
function processInput(): void {
    // INFO FOR _user_input:
    // user_input(int cameraMoveFoward, int cameraMoveSide, int cameraMoveUp, int cameraRotateZ, int cameraRotateY, int userInputCode)
    if (pressedKeys['w']) {
        CPPuserInput(1, 0, 0, 0, 0, 0);
    }
    if (pressedKeys['s']) {
        CPPuserInput(-1, 0, 0, 0, 0, 0);
    }
    if (pressedKeys['a']) {
        CPPuserInput(0, -1, 0, 0, 0, 0);
    }
    if (pressedKeys['d']) {
        CPPuserInput(0, 1, 0, 0, 0, 0);
    }
    if (pressedKeys[' ']) {
        CPPuserInput(0, 0, 1, 0, 0, 0);
    }
    if (pressedKeys['Shift']) {
        CPPuserInput(0, 0, -1, 0, 0, 0);
    }

    if (pressedKeys['ArrowLeft']) {
        CPPuserInput(0, 0, 0, 1, 0, 0);
    }
    if (pressedKeys['ArrowRight']) {
        CPPuserInput(0, 0, 0, -1, 0, 0);
    }
    if (pressedKeys['ArrowUp']) {
        CPPuserInput(0, 0, 0, 0, 1, 0);
    }
    if (pressedKeys['ArrowDown']) {
        CPPuserInput(0, 0, 0, 0, -1, 0);
    }
    if (pointerLocked) {
        CPPuserInput(0, 0, 0, - mouseX * mouseMultiplier, - mouseY * mouseMultiplier, 0);
        mouseX = 0;
        mouseY = 0;
    }
}

// Event listeners...
document.addEventListener('keydown', (event: KeyboardEvent) => {
    if (event.key == 'p') {
        console.log('P pressed');
        running = !running;
    }
    if (event.key == 'Escape') {
        console.log('Escape pressed');
        document.exitPointerLock();
        console.log(pointerLocked);
    }
});

document.addEventListener('click', (event) => {
    if (pointerLocked) {
        if (event.button == 0) {
            CPPuserInput(0,0,0,0,0, 1)
        } else if (event.button == 2) {
            CPPuserInput(0,0,0,0,0, 2)
        }
    }
});

canvas.addEventListener('mousemove', (event) => {
    if (running) {
        mouseX += event.movementX;
        mouseY += event.movementY;
    }
});

canvas.addEventListener('click', () => {
    canvas.requestPointerLock();
    pointerLocked = true;
});


// Exporting scene data
// TODO: change this to a button press
document.addEventListener('keydown', (event: KeyboardEvent) => {    
    if (event.key == '1') {
        console.log("Getting scene data...");

        var metadata_size: number = CPPgetSceneMetaDataSize();
        var metadata_buffer_pointer: number = CPPgetSceneMetaDataBuffer();

        var metadata = new Uint32Array(CPPModule.HEAPU32.buffer, metadata_buffer_pointer, metadata_size);

        var num_objects: number = metadata[0];
        var object_sizes: number[] = [];
        var num_triangles: number = 0;
        for (let i = 0; i < num_objects; i++) {
            object_sizes.push(metadata[1 + i]);
            num_triangles += metadata[1 + i];
        }

        var pos_buffer_pointer: number = CPPgetScenePosDataBuffer();
        var color_buffer_pointer: number = CPPgetSceneColorDataBuffer();

        var pos_data = new Float32Array(CPPModule.HEAPF32.buffer, pos_buffer_pointer, num_triangles * 9);
        var color_data = new Uint32Array(CPPModule.HEAPU32.buffer, color_buffer_pointer, num_triangles * 3);

        var scene_data = {
        'metadata' : Array.from(metadata),
        'pos_data' : Array.from(pos_data),
        'color_data' : Array.from(color_data)
        }
        console.log(scene_data);

    }
});