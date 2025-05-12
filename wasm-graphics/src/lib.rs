// Main build script:
// wasm-pack build --target web --out-dir ../web/wasm

// wasm-bindgen custom build script:
// ./build.sh

mod wasm;
mod graphics;
mod utils;

// re-export the wasm_bindgen_rayon functions
pub use wasm_bindgen_rayon::init_thread_pool;