export async function solvePow(challenge: string): Promise<string | undefined> {
    // spawn the worker in advance so it can async load the wasm already
    let worker = new Worker(new URL("powWorker.ts", import.meta.url));

    // Usually, it makes a lot of sense to have the `fetch` inside this function as well.
    // If the worker is spawned async in advance it can load the wasm while the `fetch`
    // is retrieving the challenge from the backend. With this strategy, the overhead
    // from starting the worker in the first place and exchanging messages is very minimal.
    // It is even more visible in this example due to the challenge being available immediately.
    // In a real world scenario, you would only see a difference if your backend is really
    // fast with sending the answer, basically faster than the worker can async load the wasm.

    return await new Promise((resolve) => {
        worker.onmessage = ev => {
            resolve(ev.data as string);
        }
        worker.postMessage(challenge);
    });
}
