/* @ts-self-types="./spow-server-wasm.d.ts" */
import * as wasm from "./spow-server-wasm_bg.wasm";
import { __wbg_set_wasm } from "./spow-server-wasm_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Pow
} from "./spow-server-wasm_bg.js";
