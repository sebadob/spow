export async function solvePow(challenge: string): Promise<string | undefined> {
    // spawn the worker in advance so it can async load the wasm already
    let worker = new Worker(new URL("powWorker.ts", import.meta.url));

    return await new Promise((resolve) => {
        worker.onmessage = ev => {
            resolve(ev.data as string);
        }
        worker.postMessage(challenge);
    });
}
