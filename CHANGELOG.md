# Changelog

## v0.5.0

This version just bumps internal dependencies.

## v0.4.0

### Breaking

There are now `server` and `client` features that must be enabled properly.  
The old behavior was to simply separate code by target arch. However, this prevented you from compiling the server
side logic as WASM, which you might want to do in certain cases.

The output directory has changed to `prebuilt/`, because you will now also find prebuilt server side WASM that
can't only solve a challenge, but also create a new one and validate. This makes it possible to host the server
side not only from a Rust backend, but basically anything that can work with WASM in the backend like NodeJS.

## v0.3.0

- The `asm` feature has been made opt-it, because it is problematic on Windows
- rebuild the wasm binary with latest updates and wasm-bindgen

## v0.2.0

- Changed the default difficulty to `20`, which is more reasonable for modern processors
- Added `Pow::init_bytes()` to init the secrets from given bytes

## v0.1.1

Include more documentation on docs.rs

## v0.1.0

Open Source Release
