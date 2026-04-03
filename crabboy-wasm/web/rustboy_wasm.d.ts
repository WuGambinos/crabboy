/* tslint:disable */
/* eslint-disable */
/**
* @param {number} value
* @returns {Key | undefined}
*/
export function button_value_to_joypad_key(value: number): Key | undefined;
/**
* @param {any} value
* @returns {Key | undefined}
*/
export function js_value_to_joypad_key(value: any): Key | undefined;
/**
*/
export enum Key {
  Right = 0,
  Left = 1,
  Up = 2,
  Down = 3,
  A = 4,
  B = 5,
  Start = 6,
  Select = 7,
}
/**
*/
export class Rgb {
  free(): void;
}
/**
*/
export class WebGameBoy {
  free(): void;
/**
*/
  constructor();
/**
*/
  reset(): void;
/**
* @param {Uint8Array} rom
*/
  boot(rom: Uint8Array): void;
/**
* @param {number} value
*/
  on_button_down(value: number): void;
/**
* @param {number} value
*/
  on_button_up(value: number): void;
/**
* @param {any} value
*/
  on_key_down(value: any): void;
/**
* @param {any} value
*/
  on_key_up(value: any): void;
/**
*/
  run(): void;
/**
*/
  render_frame(): void;
/**
*/
  draw(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly button_value_to_joypad_key: (a: number) => number;
  readonly js_value_to_joypad_key: (a: number) => number;
  readonly __wbg_webgameboy_free: (a: number, b: number) => void;
  readonly webgameboy_new: () => number;
  readonly webgameboy_reset: (a: number) => void;
  readonly webgameboy_boot: (a: number, b: number, c: number) => void;
  readonly webgameboy_on_button_down: (a: number, b: number) => void;
  readonly webgameboy_on_button_up: (a: number, b: number) => void;
  readonly webgameboy_on_key_down: (a: number, b: number) => void;
  readonly webgameboy_on_key_up: (a: number, b: number) => void;
  readonly webgameboy_run: (a: number) => void;
  readonly webgameboy_render_frame: (a: number) => void;
  readonly webgameboy_draw: (a: number) => void;
  readonly __wbg_rgb_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
