extern crate cc;
extern crate pkg_config;

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
    let mut build = build.file(path.join("rpmalloc.c")).opt_level(2);
    // add defines for enabled features

    #[rustfmt::skip]
    let features = [
        ( "ENABLE_PRELOAD", cfg!(feature = "preload") ),
        ( "ENABLE_STATISTICS", cfg!(feature = "statistics") ),
        ( "ENABLE_VALIDATE_ARGS", cfg!(feature = "validate_args") ),
        ( "ENABLE_ASSERTS", cfg!(feature = "asserts") ),
        ( "ENABLE_GUARDS", cfg!(feature = "guards") ),
        ( "ENABLE_UNLIMITED_CACHE", cfg!(feature = "unlimited_cache") ),
        ( "ENABLE_UNLIMITED_GLOBAL_CACHE", cfg!(feature = "unlimited_global_cache") ),
        ( "ENABLE_UNLIMITED_THREAD_CACHE", cfg!(feature = "unlimited_thread_cache") ),
        ( "ENABLE_GLOBAL_CACHE", cfg!(feature = "global_cache") ),
        ( "ENABLE_THREAD_CACHE", cfg!(feature = "thread_cache") ),
        ( "ENABLE_ADAPTIVE_THREAD_CACHE", cfg!(feature = "adaptive_thread_cache") ),
    ];

    for (name, value) in features.iter() {
        if *value {
            build = build.define(name, "1");
        }
    }

    // set platform-specific compile and link flags

    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "linux" => {
            build = build.define("_GNU_SOURCE", "1");
            println!("cargo:rustc-link-lib=pthread");
        }
        "macos" => {
            build = build
                .flag("-Wno-padded")
                .flag("-Wno-documentation-unknown-command")
                .flag("-Wno-static-in-inline");
        }
        _ => (),
    }

    build.compile("librpmalloc.a")
}
