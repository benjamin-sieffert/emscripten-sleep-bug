## Reproducer for failing ASYNCIFY=2 program

Build with `--target wasm32-unknown-emscripten`, optionally use the "dev-server" to bundle and serve the whole thing.

Can also switch back to "ASYNCIFY=1" to have a more slim reproducer of the sleep bug (cf. main branch).
