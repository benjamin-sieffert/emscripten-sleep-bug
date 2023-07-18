web:
	cargo build --target wasm32-unknown-emscripten --release

native:
	cargo build --release

clown:
	cargo build --target asmjs-unknown-emscripten --release
