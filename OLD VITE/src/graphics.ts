import * as CPPInterface from './cppInterface.ts';

// DOM Canvas Elements
const canvas = document.getElementById('canvas') as HTMLCanvasElement;
const canvasContext = canvas.getContext('2d') as CanvasRenderingContext2D;

// Set the canvas image to the image data from the C++ code
export function setCanvasImage(): void {
    const imageData: ImageData = canvasContext.createImageData(500, 500);
    const buffer_data: number = CPPInterface.CPPgetBuffer();
    var uint8array: Uint8ClampedArray = new Uint8ClampedArray(CPPInterface.CPPmodule.HEAPU8.buffer, buffer_data, 500 * 500 * 4);
    imageData.data.set(uint8array);
    canvasContext.putImageData(imageData, 0, 0);
}
