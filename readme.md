## `emscripten_sleep` Bug Reproducer

This repo is forked from an emscripten-SDL-Rust Template that seemed to work OK in February 2022. In 2023 it is a bit broken.

This reproducer is built with Rust/cargo. Rust can be installed via package manager or https://rustup.rs/ .

A simple build is executed via `cargo build --target wasm32-unknown-emscripten --release` and will put the artifacts under `target/wasm32-unknown-emscripten/release`.

The `./dev-server.py` can be used to automatically copy all required files into `web/` and serve them.

A simple `cargo run` can be used to build and execute the native (non browser) version for comparison.

### Emscripten bindings

The bindings to call "C" emscripten functions are in `emscripten_wrappers.rs`. They are forked without changes from the original repo.

### Changing build parameters

The file `build.rs` configures the options that `emcc` is called with when building.

`emcc` is always taken from `$PATH` and will then use its own `.emscripten` to find all the other pieces of the wasm pipeline.



