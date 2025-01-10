// @ts-ignore
import WasmModule from './cpp/3D-Graphics.js'

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



export function CPPrenderScene(): void {
    // @ts-ignore
    CPPmodule._renderScene();
}
export function CPPrenderSceneRayTracing(startIndex: number): number {
    // @ts-ignore
    return CPPmodule._renderSceneRayTracing(startIndex);
}
export function CPPgetImageBuffer(): number {
    // console.log("cppInterface.ts: CPPgetImageBuffer()");
    // @ts-ignore
    return CPPmodule._getImageBuffer();
};
export function CPPuserInput(a: number, b: number, c: number, d: number, e: number, f: number): void {
    // @ts-ignore
    CPPmodule._userInput(a,b,c,d,e,f);
}


export function CPPgetSceneDataBuffer(): number {
    // @ts-ignore
    return CPPmodule._getSceneDataBuffer();
}
export function CPPallocateSceneDataBuffer(size: number): number {
    // @ts-ignore
    return CPPmodule._allocateSceneDataBuffer(size);
}
export function CPPloadSceneToCPP(dataPointer: number) {
    // @ts-ignore
    CPPmodule._loadSceneToCPP(dataPointer);
}

// export function CPPsetSelectedColors(r: number, g: number, b: number): void {
//     // @ts-ignore
//     return CPPmodule._EXTERN_setSelectedColors(r, g, b);
// }



// export function CPPsetupScene(): void {
//     // @ts-ignore
//     //_EXTERN_setupScene();
//     CPPmodule._EXTERN_setupScene();
// }
// export function CPPgetBuffer(): number {
//     // @ts-ignore
//     return CPPmodule._EXTERN_getBuffer();
// }

