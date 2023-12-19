# svelte-wasm example

This example shows how to calculate a Proof of Work in the browser. It uses SvelteKit in this example, but this should
work with any framework or technology with more or less work regarding the implementation.

It uses 2 versions:
- wasm
- JS only

The wasm version should always be preferred, since it is a lot faster, which means you can set higher difficulties
without a negative impact on the UX.  
You should only use the JS version, if you have a good reason to do this.

The modules used in the UI are just copied over from the `/frontend` folder of the main project.  
These are re-built with each release, or it can be triggered with `just build-frontend`.

In the future, these may be released via `npm` for easier usage and there might be pre-built component to give you
a head start on implementing PoW's in the UI.
