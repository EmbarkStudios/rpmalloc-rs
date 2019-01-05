extern crate pkg_config;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut path: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("rpmalloc-greedy");
    path.push("rpmalloc");

    if !pkg_config::find_library("librpmalloc").is_ok() {
        let mut build = cc::Build::new();
        build
            .file(path.join("rpmalloc.c"))
            .opt_level(2)
            .compile("librpmalloc.a")
    }
}
