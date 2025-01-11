import * as CPPInterface from './cppInterface.ts';
import * as Main from './main.ts';

// DOM Canvas Elements
const canvas = document.getElementById('canvas') as HTMLCanvasElement;
const colorPicker = document.getElementById('color-picker') as HTMLInputElement;

// User input variables
var pressedKeys: { [key: string]: boolean } = {};
var keysPressedInLastFrame: { [key: string]: boolean } = {}
let mouseX: number = 0;
let mouseY: number = 0;
let moveMultiplier: number = 0.25;
let mouseMultiplier: number = 0.01;
let pointerLocked: boolean = false;
let r: number = 0;
let g: number = 0;
let b: number = 0;
window.onkeyup = function(e: KeyboardEvent) { pressedKeys[e.key] = false; }
window.onkeydown = function(e: KeyboardEvent) { pressedKeys[e.key] = true; }

// Pre set color picker
colorPicker.value = "#7d7d7d";
hexToRgb(colorPicker.value);
CPPInterface.CPPsetSelectedColors(r, g, b);

// Process user input and send it to the C++ code
export function processInput(): void {
    if (!pointerLocked) {
        return;
    }

    if (keysPressedInLastFrame['p']) {
        console.log("processing P pressed");
        if (Main.isRunning()) {
            Main.pause();
        } else {
            Main.unpause();
        }
    }

    if (keysPressedInLastFrame['r']) {
        console.log("processing R pressed");
        if (!Main.isRayTracing()) {
            Main.beginRayTracing();
        }
    }

    keysPressedInLastFrame = {};


    // INFO FOR _user_input:
    // user_input(int cameraMoveFoward, int cameraMoveSide, int cameraMoveUp, int cameraRotateZ, int cameraRotateY, int userInputCode)
    let cameraMoveFoward: number = 0;
    let cameraMoveSide: number = 0;
    let cameraMoveUp: number = 0;
    let cameraRotateZ: number = 0;
    let cameraRotateY: number = 0;

    if (Main.isRunning()) {
        if (pressedKeys['w']) {
            cameraMoveFoward += 1;
            // CPPInterface.CPPuserInput(1, 0, 0, 0, 0, 0);
        }
        if (pressedKeys['s']) {
            cameraMoveFoward -= 1;
            // CPPInterface.CPPuserInput(-1, 0, 0, 0, 0, 0);
        }
        if (pressedKeys['a']) {
            cameraMoveSide += 1;
            // CPPInterface.CPPuserInput(0, -1, 0, 0, 0, 0);
        }
        if (pressedKeys['d']) {
            cameraMoveSide -= 1;
            // CPPInterface.CPPuserInput(0, 1, 0, 0, 0, 0);
        }
        if (pressedKeys[' ']) {
            cameraMoveUp += 1;
            // CPPInterface.CPPuserInput(0, 0, 1, 0, 0, 0);
        }
        if (pressedKeys['Shift']) {
            cameraMoveUp -= 1;
            // CPPInterface.CPPuserInput(0, 0, -1, 0, 0, 0);
        }

        if (pressedKeys['ArrowLeft']) {
            cameraRotateZ += 1;
            // CPPInterface.CPPuserInput(0, 0, 0, 1, 0, 0);
        }
        if (pressedKeys['ArrowRight']) {
            cameraRotateZ -= 1;
            // CPPInterface.CPPuserInput(0, 0, 0, -1, 0, 0);
        }
        if (pressedKeys['ArrowUp']) {
            cameraRotateY += 1;
            // CPPInterface.CPPuserInput(0, 0, 0, 0, 1, 0);
        }
        if (pressedKeys['ArrowDown']) {
            cameraRotateY -= 1;
            // CPPInterface.CPPuserInput(0, 0, 0, 0, -1, 0);
        }
        if (pointerLocked) {
            CPPInterface.CPPuserInput(0, 0, 0, - mouseX * mouseMultiplier, - mouseY * mouseMultiplier, 0);
            mouseX = 0;
            mouseY = 0;
        }

        cameraMoveFoward *= moveMultiplier;
        cameraMoveSide *= moveMultiplier;
        cameraMoveUp *= moveMultiplier;
        CPPInterface.CPPuserInput(cameraMoveFoward, cameraMoveSide, cameraMoveUp, cameraRotateZ, cameraRotateY, 0);
    }

}

// Event listeners...
document.addEventListener('keydown', (event: KeyboardEvent) => {
    if (event.key == 'p') {
        console.log('P pressed');
        keysPressedInLastFrame['p'] = true;
    }
    if (event.key == 'r') {
        console.log('R pressed');
        keysPressedInLastFrame['r'] = true;
    }
    if (event.key == '9') {
        // Database.exportSceneData("testName");
    }
    if (event.key == '0') {
        // Database.importSceneData(7);
    }
});
document.addEventListener('pointerlockchange', () => {
    if (document.pointerLockElement) {
        pointerLocked = true;
    } else {
        pointerLocked = false;
    }
    console.log("Pointer lock changed to: " + pointerLocked);
});

document.addEventListener('click', (event) => {
    if (pointerLocked && Main.isRunning()) {
        if (event.button == 0) {
            CPPInterface.CPPuserInput(0,0,0,0,0, 1)
        } else if (event.button == 2) {
            CPPInterface.CPPuserInput(0,0,0,0,0, 2)
        }
    }
});

// Function to convert hex to RGB
function hexToRgb(hex: string) {
    // Remove the '#' if present
    hex = hex.replace(/^#/, '');
    
    // Parse the r, g, b values from the hex string
    const bigint = parseInt(hex, 16);
    r = (bigint >> 16) & 255;
    g = (bigint >> 8) & 255;
    b = bigint & 255;
}
colorPicker.addEventListener('input', () => {
    const hexColor = colorPicker.value; // Get the hexadecimal color value
    hexToRgb(hexColor); // Convert hex to RGB
    // console.log(r + ", " + g + ", " + b);
});
colorPicker.addEventListener('change', () => {
    console.log("Color picker change with colors: " + r + ", " + g + ", " + b);
    CPPInterface.CPPsetSelectedColors(r, g, b);
});

canvas.addEventListener('mousemove', (event) => {
    if (Main.isRunning() && pointerLocked) {
        mouseX += event.movementX;
        mouseY += event.movementY;
    }
});

canvas.addEventListener('click', () => {
    console.log("Requesting pointer lock...");
    canvas.requestPointerLock();
});