# `emscripten_sleep` Bug Reproducer

This reproducer is built with Rust/cargo. Installing Rust can be done via package manager or https://rustup.rs/ .

A simple build is executed via `cargo build --target wasm32-unknown-emscripten --release` and will put the artifacts under `target/wasm32-unknown-emscripten/release`.

The `./dev-server.py` can be used to automatically copy all required files into `web/` and serve them.

## Changing build parameters

The file `build.rs` configures the options that `emcc` is called with when building.

`emcc` is always taken from `$PATH` and will then use its own `.emscripten` to find all the other pieces of the wasm pipeline.



