// EXPORTING C++ FUNCTIONS TO BE USED IN TYPESCRIPT

let moduleInitialized = false;
console.log("Loading module...");
Module.onRuntimeInitialized = () => {
    moduleInitialized = true;
};

// Wait for the module to be initialized...
while (!moduleInitialized) {
    console.log("Waiting for module to be initialized...");
    await new Promise(r => setTimeout(r, 10));
}

console.log("Module loaded!!");
export function CPPsetupScene() {
    _EXTERN_setupScene();
}
export function CPPgetBuffer() {
    return _EXTERN_getBuffer();
}
export function CPPuserInput(a,b,c,d,e,f) {
    _EXTERN_userInput(a,b,c,d,e,f);
}
export const CPPModule = Module;