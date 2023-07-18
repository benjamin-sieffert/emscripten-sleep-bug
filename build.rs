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
    ];

    for x in link_args {
        println!("cargo:rustc-link-arg=-s{}", x);
    }

    println!("cargo:rustc-link-arg=--embed-file=assets");
}
