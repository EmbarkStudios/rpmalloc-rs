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
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! Note that the [rpmalloc](https://github.com/rampantpixels/rpmalloc) library this crate uses is under public domain, and can also be licensed under MIT.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

// BEGIN - Embark standard lints v0.3
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::explicit_into_iter_loop,
    clippy::filter_map_next,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::pub_enum_variant_names,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::verbose_file_reads,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.3
// crate-specific exceptions:
#![allow(unsafe_code)] // FFI bindings need it

use rpmalloc_sys as ffi;

use std::alloc::{GlobalAlloc, Layout};
use std::mem::MaybeUninit;

/// rpmalloc global allocator wrapper
pub struct RpMalloc;

unsafe impl GlobalAlloc for RpMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::rpaligned_alloc(layout.align(), layout.size()) as *mut u8
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ffi::rpaligned_calloc(layout.align(), 1, layout.size()) as *mut u8
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
