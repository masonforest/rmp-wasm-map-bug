This is a minimal testcase of [Rust MessagePack](https://github.com/3Hren/msgpack-rust) issue [#160](https://github.com/3Hren/msgpack-rust/issues/160)

Run:

    cargo  rustc --verbose --target wasm32-unknown-unknown --lib -- -O && node run.js

The expected output is:

    Pointer to five points to: 5
    Pointer to six points to: 6

The actual output is:

    Pointer to five points to: 5
    Pointer to six points to: 2310
