import * as CPPInterface from './cppInterface.ts';
import * as Main from './main.ts';
import * as Database from './database.ts';

// DOM Canvas Elements
const canvas = document.getElementById('canvas') as HTMLCanvasElement;

// User input variables
var pressedKeys: { [key: string]: boolean } = {};
let mouseX: number = 0;
let mouseY: number = 0;
let mouseMultiplier: number = 1;
let pointerLocked: boolean = false;
window.onkeyup = function(e: KeyboardEvent) { pressedKeys[e.key] = false; }
window.onkeydown = function(e: KeyboardEvent) { pressedKeys[e.key] = true; }

// Process user input and send it to the C++ code
export function processInput(): void {
    // INFO FOR _user_input:
    // user_input(int cameraMoveFoward, int cameraMoveSide, int cameraMoveUp, int cameraRotateZ, int cameraRotateY, int userInputCode)
    if (pressedKeys['w']) {
        CPPInterface.CPPuserInput(1, 0, 0, 0, 0, 0);
    }
    if (pressedKeys['s']) {
        CPPInterface.CPPuserInput(-1, 0, 0, 0, 0, 0);
    }
    if (pressedKeys['a']) {
        CPPInterface.CPPuserInput(0, -1, 0, 0, 0, 0);
    }
    if (pressedKeys['d']) {
        CPPInterface.CPPuserInput(0, 1, 0, 0, 0, 0);
    }
    if (pressedKeys[' ']) {
        CPPInterface.CPPuserInput(0, 0, 1, 0, 0, 0);
    }
    if (pressedKeys['Shift']) {
        CPPInterface.CPPuserInput(0, 0, -1, 0, 0, 0);
    }

    if (pressedKeys['ArrowLeft']) {
        CPPInterface.CPPuserInput(0, 0, 0, 1, 0, 0);
    }
    if (pressedKeys['ArrowRight']) {
        CPPInterface.CPPuserInput(0, 0, 0, -1, 0, 0);
    }
    if (pressedKeys['ArrowUp']) {
        CPPInterface.CPPuserInput(0, 0, 0, 0, 1, 0);
    }
    if (pressedKeys['ArrowDown']) {
        CPPInterface.CPPuserInput(0, 0, 0, 0, -1, 0);
    }
    if (pointerLocked) {
        CPPInterface.CPPuserInput(0, 0, 0, - mouseX * mouseMultiplier, - mouseY * mouseMultiplier, 0);
        mouseX = 0;
        mouseY = 0;
    }
}

// Event listeners...
document.addEventListener('keydown', (event: KeyboardEvent) => {
    if (event.key == 'p') {
        console.log('P pressed');
        if (Main.isRunning()) {
            Main.pause();
        } else {
            Main.unpause();
        }
    }
    if (event.key == 'Escape') {
        console.log('Escape pressed');
        document.exitPointerLock();
        console.log(pointerLocked);
    }
    if (event.key == '1') {
        Database.test();
    }
    if (event.key == '2') {
        Database.importSceneData(11);
    }
});

document.addEventListener('click', (event) => {
    if (pointerLocked) {
        if (event.button == 0) {
            CPPInterface.CPPuserInput(0,0,0,0,0, 1)
        } else if (event.button == 2) {
            CPPInterface.CPPuserInput(0,0,0,0,0, 2)
        }
    }
});

canvas.addEventListener('mousemove', (event) => {
    if (Main.isRunning()) {
        mouseX += event.movementX;
        mouseY += event.movementY;
    }
});

canvas.addEventListener('click', () => {
    canvas.requestPointerLock();
    pointerLocked = true;
});