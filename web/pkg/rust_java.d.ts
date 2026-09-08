/* tslint:disable */
/* eslint-disable */

export class RustJavaWeb {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static create(canvas: HTMLCanvasElement, width: number, height: number): Promise<RustJavaWeb>;
    diagnostics(): string;
    key(key_code: number, pressed: boolean): Promise<void>;
    loadJar(name: string, bytes: Uint8Array): Promise<void>;
    present(): boolean;
    rendererInfo(): string;
    setKeyState(key_code: number, pressed: boolean): void;
    shutdown(): Promise<void>;
}

export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_rustjavaweb_free: (a: number, b: number) => void;
    readonly rustjavaweb_create: (a: any, b: number, c: number) => any;
    readonly rustjavaweb_loadJar: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly rustjavaweb_present: (a: number) => [number, number, number];
    readonly rustjavaweb_key: (a: number, b: number, c: number) => any;
    readonly rustjavaweb_setKeyState: (a: number, b: number, c: number) => void;
    readonly rustjavaweb_diagnostics: (a: number) => [number, number];
    readonly rustjavaweb_rendererInfo: (a: number) => [number, number];
    readonly rustjavaweb_shutdown: (a: number) => any;
    readonly start: () => void;
    readonly wasm_bindgen__convert__closures_____invoke__h183410d60433b943: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h7bb661a3f8da6b28: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h26ae64def96ea1ac: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hddb16e33481cb9dc: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
