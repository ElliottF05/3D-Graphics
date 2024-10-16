import './style.css'

// INITIALIZING THE CPP MODULE

// @ts-ignore
export const CPPmodule = Module;
export let CPPmoduleInitialized: boolean = false;

// Wait for the module to be initialized...
console.log("Loading module...");
CPPmodule.onRuntimeInitialized = () => {
    CPPmoduleInitialized = true;
};
while (!CPPmoduleInitialized) {
    console.log("Waiting for module to be initialized...");
    await new Promise(r => setTimeout(r, 10));
}
console.log("Module loaded!!");


export function CPPsetupScene(): void {
    // @ts-ignore
    _EXTERN_setupScene();
}
export function CPPgetBuffer(): number {
    // @ts-ignore
    return _EXTERN_getBuffer();
}
export function CPPuserInput(a: number, b: number, c: number, d: number, e: number, f: number): void {
    // @ts-ignore
    _EXTERN_userInput(a,b,c,d,e,f);
}
export function CPPgetSceneMetaDataSize(): number {
    // @ts-ignore
    return _EXTERN_getSceneMetaDataSize();
}
export function CPPgetSceneMetaDataBuffer(): number {
    // @ts-ignore
    return _EXTERN_getSceneMetaDataBuffer();
}
export function CPPgetScenePosDataBuffer(): number {
    // @ts-ignore
    return _EXTERN_getScenePosDataBuffer();
}
export function CPPgetSceneColorDataBuffer(): number {
    // @ts-ignore
    return _EXTERN_getSceneColorDataBuffer();
}