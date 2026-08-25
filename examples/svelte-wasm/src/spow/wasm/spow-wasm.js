/* @ts-self-types="./spow-wasm.d.ts" */
import * as wasm from "./spow-wasm_bg.wasm";
import { __wbg_set_wasm } from "./spow-wasm_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    pow_work_wasm
} from "./spow-wasm_bg.js";
