/* tslint:disable */
/* eslint-disable */
export function enter_edit_mode(): void;
export function exit_edit_mode(): void;
export function delete_selected_object(): void;
export function set_follow_camera(follow_camera: boolean): void;
export function set_selected_object_material_properties(props: MaterialProperties): void;
export function enter_ray_tracing_mode(): void;
export function stop_ray_tracing(): void;
export function translate_selected_obj(x: number, y: number, z: number): void;
export function rotate_selected_obj(x: number, y: number, z: number): void;
export function scale_selected_obj(scale_factor: number): void;
export function add_sphere(radius: number): void;
export function add_box(x: number, y: number, z: number): void;
export function add_custom_object(glb_bytes?: Uint8Array | null): void;
export function set_fov(fov: number): void;
export function set_focal_dist(focal_dist: number): void;
export function set_defocus_angle(defocus_angle: number): void;
export function load_scene_random_spheres(): void;
export function load_scene_fantasy_book(glb_bytes?: Uint8Array | null): void;
export function load_scene_magic_bridge(glb_bytes?: Uint8Array | null): void;
export function load_scene_cornell_box(): void;
export function load_scene_cornell_box_extra(stl_bytes?: Uint8Array | null): void;
export function load_scene_simple_light(): void;
export function load_scene_gandalf_bust(stl_bytes?: Uint8Array | null): void;
export function load_scene_roza_bust(glb_bytes?: Uint8Array | null): void;
export function load_scene_dragon(stl_bytes?: Uint8Array | null): void;
export function load_scene_mirror_box(skull_stl_bytes?: Uint8Array | null, sculpture_stl_bytes?: Uint8Array | null): void;
export function load_scene_suzanne_monkey(stl_bytes?: Uint8Array | null): void;
export function init_and_begin_game_loop(): void;
export function load_glb_model(glb_bytes: Uint8Array): boolean;
export function initThreadPool(num_threads: number): Promise<any>;
export function wbg_rayon_start_worker(receiver: number): void;
export class MaterialProperties {
  free(): void;
  constructor(mat_is_editable: boolean, r: number, g: number, b: number, material_type: number, extra_prop: number);
  mat_is_editable: boolean;
  r: number;
  g: number;
  b: number;
  material_type: number;
  extra_prop: number;
}
export class wbg_rayon_PoolBuilder {
  private constructor();
  free(): void;
  numThreads(): number;
  receiver(): number;
  build(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly __wbg_materialproperties_free: (a: number, b: number) => void;
  readonly __wbg_get_materialproperties_mat_is_editable: (a: number) => number;
  readonly __wbg_set_materialproperties_mat_is_editable: (a: number, b: number) => void;
  readonly __wbg_get_materialproperties_r: (a: number) => number;
  readonly __wbg_set_materialproperties_r: (a: number, b: number) => void;
  readonly __wbg_get_materialproperties_g: (a: number) => number;
  readonly __wbg_set_materialproperties_g: (a: number, b: number) => void;
  readonly __wbg_get_materialproperties_b: (a: number) => number;
  readonly __wbg_set_materialproperties_b: (a: number, b: number) => void;
  readonly __wbg_get_materialproperties_material_type: (a: number) => number;
  readonly __wbg_set_materialproperties_material_type: (a: number, b: number) => void;
  readonly __wbg_get_materialproperties_extra_prop: (a: number) => number;
  readonly __wbg_set_materialproperties_extra_prop: (a: number, b: number) => void;
  readonly materialproperties_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly enter_edit_mode: () => void;
  readonly exit_edit_mode: () => void;
  readonly delete_selected_object: () => void;
  readonly set_follow_camera: (a: number) => void;
  readonly set_selected_object_material_properties: (a: number) => void;
  readonly enter_ray_tracing_mode: () => void;
  readonly stop_ray_tracing: () => void;
  readonly translate_selected_obj: (a: number, b: number, c: number) => void;
  readonly rotate_selected_obj: (a: number, b: number, c: number) => void;
  readonly scale_selected_obj: (a: number) => void;
  readonly add_sphere: (a: number) => void;
  readonly add_box: (a: number, b: number, c: number) => void;
  readonly add_custom_object: (a: number, b: number) => void;
  readonly set_fov: (a: number) => void;
  readonly set_focal_dist: (a: number) => void;
  readonly set_defocus_angle: (a: number) => void;
  readonly load_scene_random_spheres: () => void;
  readonly load_scene_fantasy_book: (a: number, b: number) => void;
  readonly load_scene_magic_bridge: (a: number, b: number) => void;
  readonly load_scene_cornell_box: () => void;
  readonly load_scene_cornell_box_extra: (a: number, b: number) => void;
  readonly load_scene_simple_light: () => void;
  readonly load_scene_gandalf_bust: (a: number, b: number) => void;
  readonly load_scene_roza_bust: (a: number, b: number) => void;
  readonly load_scene_dragon: (a: number, b: number) => void;
  readonly load_scene_mirror_box: (a: number, b: number, c: number, d: number) => void;
  readonly load_scene_suzanne_monkey: (a: number, b: number) => void;
  readonly init_and_begin_game_loop: () => void;
  readonly load_glb_model: (a: number, b: number) => number;
  readonly __wbg_wbg_rayon_poolbuilder_free: (a: number, b: number) => void;
  readonly wbg_rayon_poolbuilder_numThreads: (a: number) => number;
  readonly wbg_rayon_poolbuilder_receiver: (a: number) => number;
  readonly wbg_rayon_poolbuilder_build: (a: number) => void;
  readonly initThreadPool: (a: number) => any;
  readonly wbg_rayon_start_worker: (a: number) => void;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_7: WebAssembly.Table;
  readonly closure266_externref_shim: (a: number, b: number, c: any) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3c31e5190cd52939: (a: number, b: number) => void;
  readonly __wbindgen_thread_destroy: (a?: number, b?: number, c?: number) => void;
  readonly __wbindgen_start: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number }} module - Passing `SyncInitInput` directly is deprecated.
* @param {WebAssembly.Memory} memory - Deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number } | SyncInitInput, memory?: WebAssembly.Memory): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number }} module_or_path - Passing `InitInput` directly is deprecated.
* @param {WebAssembly.Memory} memory - Deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number } | InitInput | Promise<InitInput>, memory?: WebAssembly.Memory): Promise<InitOutput>;
