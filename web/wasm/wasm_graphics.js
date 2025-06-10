import { startWorkers } from './snippets/wasm-bindgen-rayon-38edf6e439f6d70d/src/workerHelpers.js';

let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.buffer !== wasm.memory.buffer) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().slice(ptr, ptr + len));
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
};

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_7.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_7.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

export function enter_edit_mode() {
    wasm.enter_edit_mode();
}

export function exit_edit_mode() {
    wasm.exit_edit_mode();
}

export function delete_selected_object() {
    wasm.delete_selected_object();
}

/**
 * @param {boolean} follow_camera
 */
export function set_follow_camera(follow_camera) {
    wasm.set_follow_camera(follow_camera);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * @param {MaterialProperties} props
 */
export function set_selected_object_material_properties(props) {
    _assertClass(props, MaterialProperties);
    var ptr0 = props.__destroy_into_raw();
    wasm.set_selected_object_material_properties(ptr0);
}

export function enter_ray_tracing_mode() {
    wasm.enter_ray_tracing_mode();
}

export function stop_ray_tracing() {
    wasm.stop_ray_tracing();
}

/**
 * @param {number} x
 * @param {number} y
 * @param {number} z
 */
export function translate_selected_obj(x, y, z) {
    wasm.translate_selected_obj(x, y, z);
}

/**
 * @param {number} x
 * @param {number} y
 * @param {number} z
 */
export function rotate_selected_obj(x, y, z) {
    wasm.rotate_selected_obj(x, y, z);
}

/**
 * @param {number} scale_factor
 */
export function scale_selected_obj(scale_factor) {
    wasm.scale_selected_obj(scale_factor);
}

/**
 * @param {number} radius
 */
export function add_sphere(radius) {
    wasm.add_sphere(radius);
}

/**
 * @param {number} x
 * @param {number} y
 * @param {number} z
 */
export function add_box(x, y, z) {
    wasm.add_box(x, y, z);
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
 * @param {Uint8Array | null} [glb_bytes]
 */
export function add_custom_object(glb_bytes) {
    var ptr0 = isLikeNone(glb_bytes) ? 0 : passArray8ToWasm0(glb_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.add_custom_object(ptr0, len0);
}

/**
 * @param {number} fov
 */
export function set_fov(fov) {
    wasm.set_fov(fov);
}

/**
 * @param {number} focal_dist
 */
export function set_focal_dist(focal_dist) {
    wasm.set_focal_dist(focal_dist);
}

/**
 * @param {number} defocus_angle
 */
export function set_defocus_angle(defocus_angle) {
    wasm.set_defocus_angle(defocus_angle);
}

export function load_scene_random_spheres() {
    wasm.load_scene_random_spheres();
}

/**
 * @param {Uint8Array | null} [glb_bytes]
 */
export function load_scene_fantasy_book(glb_bytes) {
    var ptr0 = isLikeNone(glb_bytes) ? 0 : passArray8ToWasm0(glb_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.load_scene_fantasy_book(ptr0, len0);
}

/**
 * @param {Uint8Array | null} [glb_bytes]
 */
export function load_scene_magic_bridge(glb_bytes) {
    var ptr0 = isLikeNone(glb_bytes) ? 0 : passArray8ToWasm0(glb_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.load_scene_magic_bridge(ptr0, len0);
}

export function load_scene_cornell_box() {
    wasm.load_scene_cornell_box();
}

/**
 * @param {Uint8Array | null} [stl_bytes]
 */
export function load_scene_cornell_box_extra(stl_bytes) {
    var ptr0 = isLikeNone(stl_bytes) ? 0 : passArray8ToWasm0(stl_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.load_scene_cornell_box_extra(ptr0, len0);
}

export function load_scene_simple_light() {
    wasm.load_scene_simple_light();
}

/**
 * @param {Uint8Array | null} [stl_bytes]
 */
export function load_scene_gandalf_bust(stl_bytes) {
    var ptr0 = isLikeNone(stl_bytes) ? 0 : passArray8ToWasm0(stl_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.load_scene_gandalf_bust(ptr0, len0);
}

/**
 * @param {Uint8Array | null} [glb_bytes]
 */
export function load_scene_roza_bust(glb_bytes) {
    var ptr0 = isLikeNone(glb_bytes) ? 0 : passArray8ToWasm0(glb_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.load_scene_roza_bust(ptr0, len0);
}

/**
 * @param {Uint8Array | null} [stl_bytes]
 */
export function load_scene_dragon(stl_bytes) {
    var ptr0 = isLikeNone(stl_bytes) ? 0 : passArray8ToWasm0(stl_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.load_scene_dragon(ptr0, len0);
}

/**
 * @param {Uint8Array | null} [skull_stl_bytes]
 * @param {Uint8Array | null} [sculpture_stl_bytes]
 */
export function load_scene_mirror_box(skull_stl_bytes, sculpture_stl_bytes) {
    var ptr0 = isLikeNone(skull_stl_bytes) ? 0 : passArray8ToWasm0(skull_stl_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(sculpture_stl_bytes) ? 0 : passArray8ToWasm0(sculpture_stl_bytes, wasm.__wbindgen_malloc);
    var len1 = WASM_VECTOR_LEN;
    wasm.load_scene_mirror_box(ptr0, len0, ptr1, len1);
}

/**
 * @param {Uint8Array | null} [stl_bytes]
 */
export function load_scene_suzanne_monkey(stl_bytes) {
    var ptr0 = isLikeNone(stl_bytes) ? 0 : passArray8ToWasm0(stl_bytes, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.load_scene_suzanne_monkey(ptr0, len0);
}

export function init_and_begin_game_loop() {
    wasm.init_and_begin_game_loop();
}

/**
 * @param {Uint8Array} glb_bytes
 * @returns {boolean}
 */
export function load_glb_model(glb_bytes) {
    const ptr0 = passArray8ToWasm0(glb_bytes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.load_glb_model(ptr0, len0);
    return ret !== 0;
}

/**
 * @param {number} num_threads
 * @returns {Promise<any>}
 */
export function initThreadPool(num_threads) {
    const ret = wasm.initThreadPool(num_threads);
    return ret;
}

/**
 * @param {number} receiver
 */
export function wbg_rayon_start_worker(receiver) {
    wasm.wbg_rayon_start_worker(receiver);
}

function __wbg_adapter_22(arg0, arg1, arg2) {
    wasm.closure266_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_25(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3c31e5190cd52939(arg0, arg1);
}

const MaterialPropertiesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_materialproperties_free(ptr >>> 0, 1));

export class MaterialProperties {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MaterialProperties.prototype);
        obj.__wbg_ptr = ptr;
        MaterialPropertiesFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MaterialPropertiesFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_materialproperties_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    get mat_is_editable() {
        const ret = wasm.__wbg_get_materialproperties_mat_is_editable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set mat_is_editable(arg0) {
        wasm.__wbg_set_materialproperties_mat_is_editable(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get r() {
        const ret = wasm.__wbg_get_materialproperties_r(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set r(arg0) {
        wasm.__wbg_set_materialproperties_r(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get g() {
        const ret = wasm.__wbg_get_materialproperties_g(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set g(arg0) {
        wasm.__wbg_set_materialproperties_g(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get b() {
        const ret = wasm.__wbg_get_materialproperties_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set b(arg0) {
        wasm.__wbg_set_materialproperties_b(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get material_type() {
        const ret = wasm.__wbg_get_materialproperties_material_type(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set material_type(arg0) {
        wasm.__wbg_set_materialproperties_material_type(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get extra_prop() {
        const ret = wasm.__wbg_get_materialproperties_extra_prop(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set extra_prop(arg0) {
        wasm.__wbg_set_materialproperties_extra_prop(this.__wbg_ptr, arg0);
    }
    /**
     * @param {boolean} mat_is_editable
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} material_type
     * @param {number} extra_prop
     */
    constructor(mat_is_editable, r, g, b, material_type, extra_prop) {
        const ret = wasm.materialproperties_new(mat_is_editable, r, g, b, material_type, extra_prop);
        this.__wbg_ptr = ret >>> 0;
        MaterialPropertiesFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const wbg_rayon_PoolBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wbg_rayon_poolbuilder_free(ptr >>> 0, 1));

export class wbg_rayon_PoolBuilder {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(wbg_rayon_PoolBuilder.prototype);
        obj.__wbg_ptr = ptr;
        wbg_rayon_PoolBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        wbg_rayon_PoolBuilderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wbg_rayon_poolbuilder_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    numThreads() {
        const ret = wasm.wbg_rayon_poolbuilder_numThreads(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    receiver() {
        const ret = wasm.wbg_rayon_poolbuilder_receiver(this.__wbg_ptr);
        return ret >>> 0;
    }
    build() {
        wasm.wbg_rayon_poolbuilder_build(this.__wbg_ptr);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_addEventListener_90e553fdce254421 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
    }, arguments) };
    imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
        const ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_button_f75c56aec440ea04 = function(arg0) {
        const ret = arg0.button;
        return ret;
    };
    imports.wbg.__wbg_call_672a4d21634d4a24 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_document_d249400bd7bd996d = function(arg0) {
        const ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_drawWasmPixelsToCanvas_379c2ebdb0ea88b4 = function(arg0, arg1, arg2, arg3) {
        drawWasmPixelsToCanvas(getArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
    };
    imports.wbg.__wbg_error_524f506f44df1645 = function(arg0) {
        console.error(arg0);
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_getContext_e9cf379449413580 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_getElementById_f827f0d6648718a8 = function(arg0, arg1, arg2) {
        const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getRandomValues_3a70830e53f8ccf5 = function() { return handleError(function (arg0) {
        globalThis.crypto.getRandomValues(arg0);
    }, arguments) };
    imports.wbg.__wbg_height_838cee19ba8597db = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_df82a4d3437bf1cc = function(arg0) {
        let result;
        try {
            result = arg0 instanceof CanvasRenderingContext2D;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_2ea67072a7624ac5 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof HTMLCanvasElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_KeyboardEvent_1f79ad7d32051924 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof KeyboardEvent;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_MouseEvent_ea92df42ebd8c1f9 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof MouseEvent;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_def73ea0955fc569 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_key_7b5c6cb539be8e13 = function(arg0, arg1) {
        const ret = arg1.key;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_length_a446193dc22c12f8 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_log_c222819a41e063d3 = function(arg0) {
        console.log(arg0);
    };
    imports.wbg.__wbg_movementX_1aa05f864931369b = function(arg0) {
        const ret = arg0.movementX;
        return ret;
    };
    imports.wbg.__wbg_movementY_8acfedb38a70e624 = function(arg0) {
        const ret = arg0.movementY;
        return ret;
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_new_a12002a7f91c75be = function(arg0) {
        const ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_newnoargs_105ed471475aaf50 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_newwithlength_a381634e90c276d4 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_now_d18023d54d4e5500 = function(arg0) {
        const ret = arg0.now();
        return ret;
    };
    imports.wbg.__wbg_performance_c185c0cdc2766575 = function(arg0) {
        const ret = arg0.performance;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_pointerLockElement_4e5150cbf7338524 = function(arg0) {
        const ret = arg0.pointerLockElement;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_requestAnimationFrame_d7fd890aaefc3246 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.requestAnimationFrame(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_requestPointerLock_304dd9ccfe548767 = function(arg0) {
        arg0.requestPointerLock();
    };
    imports.wbg.__wbg_set_65595bdd868b3009 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_startWorkers_2ca11761e08ff5d5 = function(arg0, arg1, arg2) {
        const ret = startWorkers(arg0, arg1, wbg_rayon_PoolBuilder.__wrap(arg2));
        return ret;
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07 = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0 = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_subarray_aa9065fa9dc5df96 = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_updateDofStrength_f0fe88a3ad923679 = function(arg0) {
        wasmToJsBridge.updateDofStrength(arg0);
    };
    imports.wbg.__wbg_updateFocalDistance_6ffa242b7f65da0d = function(arg0) {
        wasmToJsBridge.updateFocalDistance(arg0);
    };
    imports.wbg.__wbg_updateFollowCamera_908caa0ae66d6375 = function(arg0) {
        wasmToJsBridge.updateFollowCamera(arg0 !== 0);
    };
    imports.wbg.__wbg_updateFov_a21a31450b56f273 = function(arg0) {
        wasmToJsBridge.updateFov(arg0);
    };
    imports.wbg.__wbg_updateGameStatus_90e602e4cb6a2474 = function(arg0) {
        wasmToJsBridge.updateGameStatus(arg0 >>> 0);
    };
    imports.wbg.__wbg_updateSelectedObjMatProps_1d25fed871aa2c6b = function(arg0) {
        wasmToJsBridge.updateSelectedObjMatProps(arg0 === 0 ? undefined : MaterialProperties.__wrap(arg0));
    };
    imports.wbg.__wbg_warn_4ca3906c248c47c4 = function(arg0) {
        console.warn(arg0);
    };
    imports.wbg.__wbg_width_5dde457d606ba683 = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = arg0.original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper599 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 267, __wbg_adapter_22);
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper601 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 267, __wbg_adapter_25);
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_3;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbindgen_module = function() {
        const ret = __wbg_init.__wbindgen_wasm_module;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {
    imports.wbg.memory = memory || new WebAssembly.Memory({initial:23,maximum:65536,shared:true});
}

function __wbg_finalize_init(instance, module, thread_stack_size) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;

    if (typeof thread_stack_size !== 'undefined' && (typeof thread_stack_size !== 'number' || thread_stack_size === 0 || thread_stack_size % 65536 !== 0)) { throw 'invalid stack size' }
    wasm.__wbindgen_start(thread_stack_size);
    return wasm;
}

function initSync(module, memory) {
    if (wasm !== undefined) return wasm;

    let thread_stack_size
    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module, memory, thread_stack_size} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports, memory);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module, thread_stack_size);
}

async function __wbg_init(module_or_path, memory) {
    if (wasm !== undefined) return wasm;

    let thread_stack_size
    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path, memory, thread_stack_size} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('wasm_graphics_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports, memory);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module, thread_stack_size);
}

export { initSync };
export default __wbg_init;
