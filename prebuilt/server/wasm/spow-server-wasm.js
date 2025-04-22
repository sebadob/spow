import * as wasm from "./spow-server-wasm_bg.wasm";

export * from "./spow-server-wasm_bg.js";
import {__wbg_set_wasm} from "./spow-server-wasm_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
