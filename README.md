This is a minimal testcase of [Rust MessagePack](https://github.com/3Hren/msgpack-rust) issue [#160](https://github.com/3Hren/msgpack-rust/issues/160)

To run it in docker run:

    docker run -w/rmp-wasm-map-bug -v$(pwd):/rmp-wasm-map-bug masonforest/rmp-wasm-map-bug cargo +nightly rustc --target wasm32-unknown-unknown --lib -- -O && node run.js

Or to run it locally run:

    rustc +nightly -C opt-level=1 --target wasm32-unknown-unknown --crate-type=cdylib -o wasm-map-bug.wasm src/lib.rs && node run.js


The expected output is:

    Pointer to five points to: 5
    Pointer to six points to: 6

The actual output is:

    Pointer to five points to: 5
    Pointer to six points to: 2310
