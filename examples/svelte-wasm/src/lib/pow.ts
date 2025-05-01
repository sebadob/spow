export async function solvePow(challenge: string): Promise<string | undefined> {
    // spawn the worker in advance so it can async load the wasm already
    let worker = new Worker(new URL("powWorker.ts", import.meta.url));

    // Usually, it makes a lot of sense to have the `fetch` inside this function as well.
    // If the worker is spawned async in advance it can load the wasm while the `fetch`
    // is retrieving the challenge from the backend. With this strategy, the solving of
    // the PoW will have no real delay at all.

    return await new Promise((resolve) => {
        worker.onmessage = ev => {
            resolve(ev.data as string);
        }
        worker.postMessage(challenge);
    });
}
