var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import { CPPgetBuffer, CPPsetupScene, CPPuserInput, CPPModule } from './index.js';
// DOM Canvas Elements
const canvas = document.getElementById('canvas');
const canvasContext = canvas.getContext('2d');
// Rendering loop variables
let running = true;
// User input variables
var pressedKeys = {};
let mouseX = 0;
let mouseY = 0;
let mouseMultiplier = 1;
let pointerLocked = false;
window.onkeyup = function (e) { pressedKeys[e.key] = false; };
window.onkeydown = function (e) { pressedKeys[e.key] = true; };
// Begin the game/simulation loop
console.log("Setting up the scene...");
CPPsetupScene();
console.log("Scene setup complete. Beginning loop...");
loop();
// Main game/simulation loop
function loop() {
    return __awaiter(this, void 0, void 0, function* () {
        let readyForNextFrame = true;
        while (true) {
            readyForNextFrame = false;
            setTimeout(() => {
                readyForNextFrame = true;
            }, 40);
            if (running) {
                processInput();
                setCanvasImage();
            }
            // Wait for 40ms before the next frame
            while (!readyForNextFrame) {
                yield new Promise(r => setTimeout(r, 1));
            }
            ;
        }
    });
}
// Set the canvas image to the image data from the C++ code
function setCanvasImage() {
    const imageData = canvasContext.createImageData(500, 500);
    const buffer_data = CPPgetBuffer();
    var uint8array = new Uint8ClampedArray(CPPModule.HEAPU8.buffer, buffer_data, 500 * 500 * 4);
    imageData.data.set(uint8array);
    canvasContext.putImageData(imageData, 0, 0);
}
// Process user input and send it to the C++ code
function processInput() {
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
        CPPuserInput(0, 0, 0, -mouseX * mouseMultiplier, -mouseY * mouseMultiplier, 0);
        mouseX = 0;
        mouseY = 0;
    }
}
// Event listeners...
document.addEventListener('keydown', (event) => {
    if (event.key == 'p') {
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
            CPPuserInput(0, 0, 0, 0, 0, 1);
        }
        else if (event.button == 2) {
            CPPuserInput(0, 0, 0, 0, 0, 2);
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
//# sourceMappingURL=typescript.js.map