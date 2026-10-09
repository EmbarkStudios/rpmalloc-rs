use std::env;
use std::path::PathBuf;

fn main() {
    let mut path: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("rpmalloc");
    path.push("rpmalloc");

    if pkg_config::find_library("librpmalloc").is_ok() {
        return;
    }

    let mut build = cc::Build::new();
    let c_file = path.join("rpmalloc.c");
    println!("cargo:rerun-if-changed={}", c_file.display());
    println!("cargo:rerun-if-changed=src/layout.c");
    let mut build = build
        .file(c_file)
        .file("src/layout.c")
        .include(&path)
        .opt_level(2);
    // add defines for enabled features

    #[rustfmt::skip]
    let features = [
        ( "ENABLE_STATISTICS", cfg!(feature = "statistics") ),
        ( "ENABLE_VALIDATE_ARGS", cfg!(feature = "validate_args") ),
        ( "ENABLE_ASSERTS", cfg!(feature = "asserts") ),
        ( "ENABLE_LEAK_DETECTION", cfg!(feature = "leak_detection") ),
    ];

    for (name, value) in features.iter() {
        if *value {
            build = build.define(name, "1");
        }
    }

    // These default to 1 upstream, so they take an explicit 0 to turn off

    #[rustfmt::skip]
    let disabled_features = [
        ( "ENABLE_UNMAP", cfg!(feature = "disable_unmap") ),
        ( "ENABLE_DECOMMIT", cfg!(feature = "disable_decommit") ),
    ];

    for (name, value) in disabled_features.iter() {
        if *value {
            build = build.define(name, "0");
        }
    }

    // `rpmalloc.c` only pulls in `malloc.c` - the standard library malloc/free/new/delete overrides
    // - when `ENABLE_OVERRIDE` is set, and we deliberately do not: this crate hands out the
    // allocator through `#[global_allocator]` instead. Upstream defaults the define to 1 and
    // adjusts finalize-time behaviour accordingly (it ignores `unmap_on_finalize` and turns off
    // leak detection), so spell out that the override is absent.
    build = build.define("ENABLE_OVERRIDE", "0");

    // set platform-specific compile and link flags

    // 2.0 allocates through C11 atomics, which `vcruntime_c11_stdatomic.h` refuses to compile below
    // `/std:c11` and which MSVC additionally keeps behind an experimental switch - the same pair
    // the upstream Visual Studio project selects. clang-cl implements them outright, and GCC and
    // clang already default to a new enough standard.
    let compiler = build.get_compiler();
    if compiler.is_like_msvc() {
        build = build.std("c11");
        if !compiler.is_like_clang_cl() {
            build = build.flag("/experimental:c11atomics");
        }
    }

    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "linux" => {
            build = build.define("_GNU_SOURCE", "1");
            println!("cargo:rustc-link-lib=pthread");
        }
        "windows" => {
            // `rpmalloc_initialize()` asks for the lock pages privilege to enable huge pages
            println!("cargo:rustc-link-lib=advapi32");
        }
        "macos" => {
            build = build
                .flag("-Wno-padded")
                .flag("-Wno-documentation-unknown-command")
                .flag("-Wno-static-in-inline");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=advapi32");
        }
        _ => (),
    }

    build.compile("librpmalloc.a");
}
