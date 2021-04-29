/* tslint:disable */
/* eslint-disable */
/**
*/
export function run_app(): void;
/**
* Handler for `console.log` invocations.
*
* If a test is currently running it takes the `args` array and stringifies
* it and appends it to the current output of the test. Otherwise it passes
* the arguments to the original `console.log` function, psased as
* `original`.
* @param {Array<any>} args
*/
export function __wbgtest_console_log(args: Array<any>): void;
/**
* Handler for `console.debug` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_debug(args: Array<any>): void;
/**
* Handler for `console.info` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_info(args: Array<any>): void;
/**
* Handler for `console.warn` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_warn(args: Array<any>): void;
/**
* Handler for `console.error` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_error(args: Array<any>): void;
/**
* Runtime test harness support instantiated in JS.
*
* The node.js entry script instantiates a `Context` here which is used to
* drive test execution.
*/
export class WasmBindgenTestContext {
  free(): void;
/**
* Creates a new context ready to run tests.
*
* A `Context` is the main structure through which test execution is
* coordinated, and this will collect output and results for all executed
* tests.
*/
  constructor();
/**
* Inform this context about runtime arguments passed to the test
* harness.
*
* Eventually this will be used to support flags, but for now it's just
* used to support test filters.
* @param {any[]} args
*/
  args(args: any[]): void;
/**
* Executes a list of tests, returning a promise representing their
* eventual completion.
*
* This is the main entry point for executing tests. All the tests passed
* in are the JS `Function` object that was plucked off the
* `WebAssembly.Instance` exports list.
*
* The promise returned resolves to either `true` if all tests passed or
* `false` if at least one test failed.
* @param {any[]} tests
* @returns {Promise<any>}
*/
  run(tests: any[]): Promise<any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run_app: () => void;
  readonly __wbgt_should_set_style_prop_36: (a: number) => void;
  readonly __wbgt_should_generate_random_string_37: (a: number) => void;
  readonly __wbgt_should_create_carousel_dot_component_7: (a: number) => void;
  readonly __wbgt_should_create_dropdown_item_10: (a: number) => void;
  readonly __wbgt_should_create_form_input_13: (a: number) => void;
  readonly __wbgt_should_create_form_group_horizontal_oriented_14: (a: number) => void;
  readonly __wbgt_should_create_form_group_vertical_oriented_15: (a: number) => void;
  readonly __wbgt_should_create_form_label_17: (a: number) => void;
  readonly __wbgt_should_create_plain_text_31: (a: number) => void;
  readonly __wbgt_should_create_paragraph_text_32: (a: number) => void;
  readonly __wbgt_should_create_alert_text_33: (a: number) => void;
  readonly __wbgt_should_create_tag_text_34: (a: number) => void;
  readonly __wbgt_should_add_delete_icon_tag_text_35: (a: number) => void;
  readonly __wbgt_should_create_carousel_container_component_5: (a: number) => void;
  readonly __wbgt_should_create_carousel_controls_component_6: (a: number) => void;
  readonly __wbgt_should_create_carousel_dot_component_8: (a: number) => void;
  readonly __wbgt_should_create_modal_component_24: (a: number) => void;
  readonly __wbgt_should_hide_modal_component_from_doom_25: (a: number) => void;
  readonly __wbgt_should_create_navbar_item_28: (a: number) => void;
  readonly __wbgt_should_create_clickable_navbar_item_29: (a: number) => void;
  readonly __wbgt_should_create_spinner_30: (a: number) => void;
  readonly __wbgt_should_create_dropdown_container_9: (a: number) => void;
  readonly __wbgt_should_create_form_input_16: (a: number) => void;
  readonly __wbgt_should_create_a_container_21: (a: number) => void;
  readonly __wbgt_should_create_item_22: (a: number) => void;
  readonly __wbgt_should_create_clickable_item_23: (a: number) => void;
  readonly __wbgt_should_create_card_with_three_parts_2: (a: number) => void;
  readonly __wbgt_should_create_card_with_single_content_3: (a: number) => void;
  readonly __wbgt_should_ignore_parts_when_single_content_exist_4: (a: number) => void;
  readonly __wbgt_should_create_form_textarea_20: (a: number) => void;
  readonly __wbgt_should_trigger_action_when_button_clicked_0: (a: number) => void;
  readonly __wbgt_should_create_button_component_1: (a: number) => void;
  readonly __wbgt_should_create_form_select_18: (a: number) => void;
  readonly __wbgt_should_create_form_component_11: (a: number) => void;
  readonly __wbgt_should_submit_the_form_12: (a: number) => void;
  readonly __wbgt_should_create_form_submit_19: (a: number) => void;
  readonly __wbgt_should_create_navbar_dropdown_container_26: (a: number) => void;
  readonly __wbgt_should_create_dropdown_item_27: (a: number) => void;
  readonly __wbg_wasmbindgentestcontext_free: (a: number) => void;
  readonly wasmbindgentestcontext_new: () => number;
  readonly wasmbindgentestcontext_args: (a: number, b: number, c: number) => void;
  readonly wasmbindgentestcontext_run: (a: number, b: number, c: number) => number;
  readonly __wbgtest_console_log: (a: number) => void;
  readonly __wbgtest_console_debug: (a: number) => void;
  readonly __wbgtest_console_info: (a: number) => void;
  readonly __wbgtest_console_warn: (a: number) => void;
  readonly __wbgtest_console_error: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf2d76668a219091c: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he01031ac25a1844f: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke0_mut__h76abdf846b2875ca: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures__invoke3_mut__h436b6d968c2c616e: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h7de2f46c0a06fb4d: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
