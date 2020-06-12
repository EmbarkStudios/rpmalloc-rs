#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

//! # 🐏 rpmalloc
//!
//! [![Build Status](https://github.com/EmbarkStudios/rpmalloc-rs/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/rpmalloc-rs/actions?workflow=CI)
//! [![Crates.io](https://img.shields.io/crates/v/rpmalloc.svg)](https://crates.io/crates/rpmalloc)
//! [![Docs](https://docs.rs/rpmalloc/badge.svg)](https://docs.rs/rpmalloc)
//! [![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
//! [![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](http://embark.dev)
//!
//! Cross-platform Rust global memory allocator using [rpmalloc](https://github.com/rampantpixels/rpmalloc).
//!
//! See the [rpmalloc README](https://github.com/mjansson/rpmalloc/blob/master/README.md) for a detailed description of how the allocator works, peforms, and compares with other allocators.
//!
//! ## How to use
//!
//! To use rpmalloc as the global allocator in your Rust binary crate, in `Cargo.toml` add:
//!
//! ```toml
//! [dependencies]
//! rpmalloc = "0.2.0"
//! ```
//!
//! And then in one of your `.rs` files:
//!
//! ```rust
//! #[global_allocator]
//! static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;
//! ```
//!
//! ### Configuration
//!
//! It is also possible to configure how the allocator should be built through a set of feature flags that correspond to the rpmalloc C library `ENABLE_x` defines:
//!
//! - Overall: `statistics`, `validate_args`, `asserts`, `guards`
//! - Cache: `unlimited_cache`, `unlimited_global_cache`, `unlimited_thread_cache`, `global_cache`, `thread_cache`, `adaptive_thread_cache`
//!
//! Example usage:
//!
//! ```toml
//! [dependencies]
//! rpmalloc = { version = "0.1.0", features = ["guards", "statistics"] }
//! ```
//!
//! See [rpmalloc README](https://github.com/mjansson/rpmalloc/blob/master/README.md) for detailed descriptions of the config options.
//!
//! Note that all of these have not been tested together with this Rust crate.
//!
//! ## Support
//!
//! This crate has been tested to support the following platforms and Rust targets:
//!
//! - `x86_64-pc-windows-msvc`
//! - `x86_64-apple-darwin`
//! - `x86_64-unknown-linux-gnu`
//!
//! PRs to increase the amount of supported targets are welcome, but they should add CI verification and avoid adding additional dependencies.
//!
//! ## Contributing
//!
//! We welcome community contributions to this project.
//!
//! Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! Note that the [rpmalloc](https://github.com/rampantpixels/rpmalloc) library this crate uses is under public domain, and can also be licensed under MIT.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

use rpmalloc_sys as ffi;

use std::alloc::{GlobalAlloc, Layout};
use std::mem::MaybeUninit;

/// rpmalloc global allocator wrapper
pub struct RpMalloc;

unsafe impl GlobalAlloc for RpMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::rpaligned_alloc(layout.align(), layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::rpfree(ptr as *mut ffi::c_void)
    }
}

impl RpMalloc {
    /// Retrieve global allocation stats.
    ///
    /// Some of the fields in these will only be used if the `statistics` feature is enabled
    pub fn global_stats() -> ffi::rpmalloc_global_statistics_t {
        let mut stats: MaybeUninit<ffi::rpmalloc_global_statistics_t> = MaybeUninit::uninit();
        unsafe {
            ffi::rpmalloc_global_statistics(stats.as_mut_ptr());
            stats.assume_init()
        }
    }

    /// Retrieve allocation stats for the current thread.
    ///
    /// Some of the fields in these will only be used if the `statistics` feature is enabled
    pub fn thread_stats() -> ffi::rpmalloc_thread_statistics_t {
        let mut stats: MaybeUninit<ffi::rpmalloc_thread_statistics_t> = MaybeUninit::uninit();
        unsafe {
            ffi::rpmalloc_thread_statistics(stats.as_mut_ptr());
            stats.assume_init()
        }
    }
}
