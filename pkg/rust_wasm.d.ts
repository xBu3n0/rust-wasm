/* tslint:disable */
/* eslint-disable */
/**
*/
export class Perceptron {
  free(): void;
/**
* @param {Uint32Array} params
* @param {Float64Array} data
* @param {number} answ_index
* @param {number} rows
* @param {number} cols
*/
  constructor(params: Uint32Array, data: Float64Array, answ_index: number, rows: number, cols: number);
/**
* @param {number} step_size
*/
  set_step_size(step_size: number): void;
/**
* @param {number} n_iter
* @returns {string}
*/
  train_with_verbose(n_iter: number): string;
/**
* @param {number} n_iter
*/
  train_without_verbose(n_iter: number): void;
/**
* @returns {string}
*/
  get_perceptron(): string;
/**
* @returns {number}
*/
  get_accuracy(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_perceptron_free: (a: number) => void;
  readonly perceptron_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly perceptron_set_step_size: (a: number, b: number) => void;
  readonly perceptron_train_with_verbose: (a: number, b: number, c: number) => void;
  readonly perceptron_train_without_verbose: (a: number, b: number) => void;
  readonly perceptron_get_perceptron: (a: number, b: number) => void;
  readonly perceptron_get_accuracy: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
