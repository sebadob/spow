let wasm = import('../spow/wasm/spow-wasm');

onmessage = ev => {
    wasm.then(wasm => {
        postMessage(wasm.pow_work_wasm(ev.data));
    });
}
