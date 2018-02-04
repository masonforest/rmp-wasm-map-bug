This is a minimal testcase of Rust issue [#48002](https://github.com/rust-lang/rust/issues/48002)

To run it in docker run:

    docker run -v$(pwd):/wasm-map-bug:ro masonforest/wasm-map-bug  rustc +nightly -C opt-level=1 --target wasm32-unknown-unknown --crate-type=cdylib -o wasm-map-bug.wasm wasm-map-bug/src/lib.rs && node run.js

Or to run it locally run:

    rustc +nightly -C opt-level=1 --target wasm32-unknown-unknown --crate-type=cdylib -o wasm-map-bug.wasm src/lib.rs && node run.js


The expected output is:

    Pointer to five points to: 5
    Pointer to six points to: 6

The actual output is:

    Pointer to five points to: 5
    Pointer to six points to: 3612


