const canvas = document.getElementById('canvas');
const canvasContext = canvas.getContext('2d');

let sceneSet = false;

Module.onRuntimeInitialized = () => {
    console.log('Module loading...');
    _setup_scene();
    console.log("Module loaded, now drawing image...");
    setCanvasImage();
    console.log("First image drawn.");
}

document.addEventListener('keydown', (event) => {
    // INFO FOR _user_input:
    // user_input(int cameraMoveFoward, int cameraMoveSide, int cameraMoveUp, int cameraRotateZ, int cameraRotateY)
    let renderNeeded = false;
    if (event.key == 'w') {
        renderNeeded = true;
        _user_input(1, 0, 0, 0, 0);
    }
    if (event.key == 's') {
        renderNeeded = true;
        _user_input(-1, 0, 0, 0, 0);
    }
    if (event.key == 'a') {
        renderNeeded = true;
        _user_input(0, -1, 0, 0, 0);
    }
    if (event.key == 'd') {
        renderNeeded = true;
        _user_input(0, 1, 0, 0, 0);
    }
    if (event.key == ' ') {
        renderNeeded = true;
        _user_input(0, 0, 1, 0, 0);
    }
    if (event.key == 'Shift') {
        renderNeeded = true;
        _user_input(0, 0, -1, 0, 0);
    }

    if (event.key == 'ArrowLeft') {
        renderNeeded = true;
        _user_input(0, 0, 0, 1, 0);
    }
    if (event.key == 'ArrowRight') {
        renderNeeded = true;
        _user_input(0, 0, 0, -1, 0);
    }
    if (event.key == 'ArrowUp') {
        renderNeeded = true;
        _user_input(0, 0, 0, 0, 1);
    }
    if (event.key == 'ArrowDown') {
        renderNeeded = true;
        _user_input(0, 0, 0, 0, -1);
    }

    if (renderNeeded) {
        setCanvasImage();
    }
    
});

function setCanvasImage() {
    console.log("Setting canvas image");
    const imageData = canvasContext.createImageData(800, 800);
    const buffer_data = _get_buffer();
    var uint8array = new Uint8ClampedArray(Module.HEAPU8.buffer, buffer_data, 800 * 800 * 4);
    imageData.data.set(uint8array);
    canvasContext.putImageData(imageData, 0, 0);
}