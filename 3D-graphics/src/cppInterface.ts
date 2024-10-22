import './style.css'
// @ts-ignore
import WasmModule from './cpp/3D-graphics.js'

// INITIALIZING THE CPP MODULE

// @ts-ignore
export let CPPmodule;
export let CPPmoduleInitialized: boolean = false;
// @ts-ignore
WasmModule().then((initializedModule) => {
    CPPmodule = initializedModule;
    CPPmoduleInitialized = true;
});

// Wait for the module to be initialized...
console.log("Loading module...");
while (!CPPmoduleInitialized) {
    console.log("Waiting for module to be initialized...");
    await new Promise(r => setTimeout(r, 10));
}
console.log("Module loaded!!");


export function CPPsetupScene(): void {
    // @ts-ignore
    //_EXTERN_setupScene();
    CPPmodule._EXTERN_setupScene();
}
export function CPPgetBuffer(): number {
    // @ts-ignore
    return CPPmodule._EXTERN_getBuffer();
}
export function CPPuserInput(a: number, b: number, c: number, d: number, e: number, f: number): void {
    // @ts-ignore
    CPPmodule._EXTERN_userInput(a,b,c,d,e,f);
}

export function CPPgetDataBufferSize(): number {
    // @ts-ignore
    return CPPmodule._EXTERN_getDataBufferSize();
}
export function CPPgetDataBufferPointer(): number {
    // @ts-ignore
    return CPPmodule._EXTERN_getDataBufferPointer();
}
export function CPPsetDataBufferPointer(size: number): number {
    // @ts-ignore
    return CPPmodule._EXTERN_setDataBufferPointer(size);
}
export function CPPloadScene(data: number): void {
    // @ts-ignore
    return CPPmodule._EXTERN_loadScene(data);
}
