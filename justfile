set shell := ["bash", "-uc"]

export TAG := `cat Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs`
WASM_TARGET := '--target wasm32-unknown-unknown'

# install needed cargo utitlies
setup:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    cargo install cargo-hack
    cargo install cargo-minimal-versions
    cargo install cargo-msrv

clippy:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    cargo clippy

    cargo clippy --features server
    cargo clippy --features client
    cargo clippy --features server,client

    cargo clippy --features server {{ WASM_TARGET }}
    cargo clippy --features client {{ WASM_TARGET }}
    cargo clippy --features server,client {{ WASM_TARGET }}

# clippy lint + check with minimal versions from nightly
check:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    cargo update

    cargo +nightly clippy -- -D warnings
    cargo +nightly clippy --features server -- -D warnings
    cargo +nightly clippy --features client -- -D warnings

    echo 'Checking minimal versions'
    cargo minimal-versions check --all-features

    cargo update

# prints out the currently set version
version:
    echo {{ TAG }}

# builds the code
build:
    #!/usr/bin/env bash
    set -euxo pipefail
    # build as musl to make sure this works
    cargo build --release --target x86_64-unknown-linux-musl

# build wasm modules
build-wasm:
    #!/usr/bin/env bash

    rm -rf prebuilt/browser/wasm/*
    rm -rf prebuilt/server/wasm/*

    cd wasm-builder
    wasm-pack build --release -d ../prebuilt/browser/wasm --no-pack --out-name spow-wasm --features client
    wasm-pack build --release -d ../prebuilt/server/wasm --no-pack --out-name spow-server-wasm --features server
    cd ..

    rm -rf examples/svelte-wasm/src/spow
    cp -r prebuilt/browser examples/svelte-wasm/src/spow

    rm -f prebuilt/browser/wasm/.gitignore
    rm -f prebuilt/server/wasm/.gitignore
    rm -f examples/svelte-wasm/src/spow/wasm/.gitignore

    git add prebuilt
    git add examples/svelte-wasm/src/spow

# runs the full set of tests
test:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    cargo test

# runs the example svelte ui for testing wasm
run-js:
    #!/usr/bin/env bash
    clear
    cd examples/svelte-wasm
    npm run dev

# verifies the MSRV
msrv-verify:
    cargo msrv verify

# find's the new MSRV, if it needs a bump
msrv-find:
    cargo msrv --min 1.70.0

# verify thats everything is good
verify: check test build msrv-verify

# makes sure everything is fine
verfiy-is-clean: verify
    #!/usr/bin/env bash
    set -euxo pipefail

    # make sure everything has been committed
    git diff --exit-code

    echo all good

# sets a new git tag and pushes it
release: verfiy-is-clean
    #!/usr/bin/env bash
    set -euxo pipefail

    # make sure git is clean
    git diff --quiet || exit 1

    git tag "v$TAG"
    git push origin "v$TAG"

# publishes the current version to cargo.io
publish: verfiy-is-clean
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo publish
