extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut path: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("rpmalloc");
    path.push("rpmalloc");

    if !pkg_config::find_library("librpmalloc").is_ok() {
        let mut build = cc::Build::new();
        let mut build = build
            .file(path.join("rpmalloc.c"))
            .opt_level(2)
            .define("ENABLE_PRELOAD", "1")
            .define("ENABLE_OVERRIDE", "1");

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
}
