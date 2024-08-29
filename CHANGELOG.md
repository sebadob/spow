# Changelog

## v0.4.0

### Breaking

There are now `server` and `client` features that must be enabled properly.  
The old behavior was to simply separate code by target arch. However, this prevented you from compiling the server
side logic as WASM, which you might want to do in certain cases.

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
