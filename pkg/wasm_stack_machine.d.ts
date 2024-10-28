/* tslint:disable */
/* eslint-disable */
export class StackMachine {
  free(): void;
  constructor();
  /**
   * @param {number} value
   */
  push(value: number): void;
  /**
   * @returns {number | undefined}
   */
  pop(): number | undefined;
  add(): void;
  sub(): void;
  /**
   * @returns {number | undefined}
   */
  peek(): number | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_stackmachine_free: (a: number, b: number) => void;
  readonly stackmachine_new: () => number;
  readonly stackmachine_push: (a: number, b: number) => void;
  readonly stackmachine_pop: (a: number, b: number) => void;
  readonly stackmachine_add: (a: number) => void;
  readonly stackmachine_sub: (a: number) => void;
  readonly stackmachine_peek: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
