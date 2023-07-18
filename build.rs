use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if !target.contains("emscripten") {
        return;
    }

    let link_args = [
        "USE_SDL=2",
        "USE_SDL_TTF=2",
        "ALLOW_MEMORY_GROWTH=1",
        "ASYNCIFY=1",
        // "TOTAL_MEMORY=128MB",
        // "NO_EXIT_RUNTIME=1",
        // "ASSERTIONS=2",
        // "SAFE_HEAP=1",
        // "DISABLE_EXCEPTION_CATCHING=0",
        // "WASM=1",
        // "WARN_UNALIGNED=1",
        // "DEMANGLE_SUPPORT=1",
        // "STACK_SIZE=52428800",
        // "FORCE_FILESYSTEM",
    ];

    for x in link_args {
        println!("cargo:rustc-link-arg=-s{}", x);
    }

    println!("cargo:rustc-link-arg=--embed-file=assets");
}
