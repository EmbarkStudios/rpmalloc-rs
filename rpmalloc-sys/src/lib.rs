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
#![allow(non_snake_case, non_camel_case_types)]
#![allow(unsafe_code)] // FFI bindings need it
#![deny(missing_docs)]

//! # 🐏 rpmalloc-sys
//!
//! [![Build Status](https://github.com/EmbarkStudios/rpmalloc-rs/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/rpmalloc-rs/actions?workflow=CI)
//! [![Crates.io](https://img.shields.io/crates/v/rpmalloc-sys.svg)](https://crates.io/crates/rpmalloc-sys)
//! [![Docs](https://docs.rs/rpmalloc-sys/badge.svg)](https://docs.rs/rpmalloc-sys)
//! [![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](../CODE_OF_CONDUCT.md)
//! [![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](http://embark.dev)
//!
//! Unsafe FFI bindings to [rpmalloc](https://github.com/rampantpixels/rpmalloc) C library
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

#[cfg(test)]
mod tests;

pub use libc::{c_int, c_uint, c_void, size_t};

/// Global memory statistics
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_global_statistics_t {
    /// Current amount of virtual memory mapped, all of which might not have been committed (only if ENABLE_STATISTICS=1)
    pub mapped: size_t,
    /// Peak amount of virtual memory mapped, all of which might not have been committed (only if ENABLE_STATISTICS=1)
    pub mapped_peak: size_t,
    /// Current amount of memory in global caches for small and medium sizes (<32KiB)
    pub cached: size_t,
    /// Current amount of memory allocated in huge allocations, i.e larger than LARGE_SIZE_LIMIT which is 2MiB by default (only if ENABLE_STATISTICS=1)
    pub huge_alloc: size_t,
    /// Peak amount of memory allocated in huge allocations, i.e larger than LARGE_SIZE_LIMIT which is 2MiB by default (only if ENABLE_STATISTICS=1)
    pub huge_alloc_peak: size_t,
    /// Total amount of memory mapped since initialization (only if ENABLE_STATISTICS=1)
    pub mapped_total: size_t,
    /// Total amount of memory unmapped since initialization  (only if ENABLE_STATISTICS=1)
    pub unmapped_total: size_t,
}

/// Mmemory span statistics for a thread
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_thread_span_statistics_t {
    /// Currently used number of spans
    pub current: size_t,
    /// High water mark of spans used
    pub peak: size_t,
    /// Number of spans transitioned to global cache
    pub to_global: size_t,
    /// Number of spans transitioned from global cache
    pub from_global: size_t,
    /// Number of spans transitioned to thread cache
    pub to_cache: size_t,
    /// Number of spans transitioned from thread cache
    pub from_cache: size_t,
    /// Number of spans transitioned to reserved state
    pub to_reserved: size_t,
    /// Number of spans transitioned from reserved state
    pub from_reserved: size_t,
    /// Number of raw memory map calls (not hitting the reserve spans but resulting in actual OS mmap calls)
    pub map_calls: size_t,
}

/// Memory size statistics for a thread
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rpmalloc_thread_size_statistics_t {
    /// Current number of allocations
    pub alloc_current: size_t,
    /// Peak number of allocations
    pub alloc_peak: size_t,
    /// Total number of allocations
    pub alloc_total: size_t,
    /// Total number of frees
    pub free_total: size_t,
    /// Number of spans transitioned to cache
    pub spans_to_cache: size_t,
    /// Number of spans transitioned from cache
    pub spans_from_cache: size_t,
    /// Number of spans transitioned from reserved state
    pub spans_from_reserved: size_t,
    /// Number of raw memory map calls (not hitting the reserve spans but resulting in actual OS mmap calls)
    pub map_calls: size_t,
}

/// Memory statistics for a thread
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rpmalloc_thread_statistics_t {
    /// Current number of bytes available in thread size class caches for small and medium sizes (<32KiB)
    pub sizecache: size_t,
    /// Current number of bytes available in thread span caches for small and medium sizes (<32KiB)
    pub spancache: size_t,
    /// Total number of bytes transitioned from thread cache to global cache (only if ENABLE_STATISTICS=1)
    pub thread_to_global: size_t,
    /// Total number of bytes transitioned from global cache to thread cache (only if ENABLE_STATISTICS=1)
    pub global_to_thread: size_t,
    /// Per span count statistics (only if ENABLE_STATISTICS=1)
    pub span_use: [rpmalloc_thread_span_statistics_t; 32],
    /// Per size class statistics (only if ENABLE_STATISTICS=1)
    pub size_use: [rpmalloc_thread_size_statistics_t; 128],
}

/*
typedef struct rpmalloc_config_t {
    //! Map memory pages for the given number of bytes. The returned address MUST be
    //  aligned to the rpmalloc span size, which will always be a power of two.
    //  Optionally the function can store an alignment offset in the offset variable
    //  in case it performs alignment and the returned pointer is offset from the
    //  actual start of the memory region due to this alignment. The alignment offset
    //  will be passed to the memory unmap function. The alignment offset MUST NOT be
    //  larger than 65535 (storable in an uint16_t), if it is you must use natural
    //  alignment to shift it into 16 bits. If you set a memory_map function, you
    //  must also set a memory_unmap function or else the default implementation will
    //  be used for both.
    void* (*memory_map)(pub size, size_t* offset);
    //! Unmap the memory pages starting at address and spanning the given number of bytes.
    //  If release is set to non-zero, the unmap is for an entire span range as returned by
    //  a previous call to memory_map and that the entire range should be released. The
    //  release argument holds the size of the entire span range. If release is set to 0,
    //  the unmap is a partial decommit of a subset of the mapped memory range.
    //  If you set a memory_unmap function, you must also set a memory_map function or
    //  else the default implementation will be used for both.
    void (*memory_unmap)(void* address, pub size, pub offset, pub release);
    //! Size of memory pages. The page size MUST be a power of two. All memory mapping
    //  requests to memory_map will be made with size set to a multiple of the page size.
    pub page_size;
    //! Size of a span of memory blocks. MUST be a power of two, and in [4096,262144]
    //  range (unless 0 - set to 0 to use the default span size).
    pub span_size;
    //! Number of spans to map at each request to map new virtual memory blocks. This can
    //  be used to minimize the system call overhead at the cost of virtual memory address
    //  space. The extra mapped pages will not be written until actually used, so physical
    //  committed memory should not be affected in the default implementation. Will be
    //  aligned to a multiple of spans that match memory page size in case of huge pages.
    pub span_map_count;
    //! Enable use of large/huge pages
    int enable_huge_pages;
    //! Debug callback if memory guards are enabled. Called if a memory overwrite is detected
    void (*memory_overwrite)(void* address);
} rpmalloc_config_t;
*/

extern "C" {
    /// Initialize allocator with default configuration
    pub fn rpmalloc_initialize() -> c_int;

    //extern int rpmalloc_initialize_config(const rpmalloc_config_t* config);
    //extern const rpmalloc_config_t* rpmalloc_config(void);

    /// Finalize allocator
    pub fn rpmalloc_finalize();

    /// Initialize allocator for calling thread
    pub fn rpmalloc_thread_initialize();

    /// Finalize allocator for calling thread
    pub fn rpmalloc_thread_finalize();

    /// Perform deferred deallocations pending for the calling thread hea
    pub fn rpmalloc_thread_collect();

    /// Query if allocator is initialized for calling thread
    pub fn rpmalloc_is_thread_initialized() -> c_int;

    /// Get per-thread statistics
    pub fn rpmalloc_thread_statistics(stats: *mut rpmalloc_thread_statistics_t);

    /// Get global statistics
    pub fn rpmalloc_global_statistics(stats: *mut rpmalloc_global_statistics_t);

    /// Dump all statistics in human readable format to file (should be a FILE*)
    pub fn rpmalloc_dump_statistics(file: *mut c_void);

    /// Allocate a memory block of at least the given size
    pub fn rpmalloc(size: size_t) -> *mut c_void;

    /// Free the given memory block
    pub fn rpfree(ptr: *mut c_void);

    /// Allocate a memory block of at least the given size and zero initialize it
    pub fn rpcalloc(num: size_t, size: size_t) -> *mut c_void;

    /// Reallocate the given block to at least the given size
    pub fn rprealloc(ptr: *mut c_void, size: size_t) -> *mut c_void;

    /// Reallocate the given block to at least the given size and alignment,
    /// with optional control flags (see RPMALLOC_NO_PRESERVE).
    /// Alignment must be a power of two and a multiple of sizeof(void*),
    /// and should ideally be less than memory page size. A caveat of rpmalloc
    /// internals is that this must also be strictly less than the span size (default 64KiB)
    pub fn rpaligned_realloc(
        ptr: *mut c_void,
        alignment: size_t,
        size: size_t,
        oldsize: size_t,
        flags: c_uint,
    ) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment.
    /// Alignment must be a power of two and a multiple of sizeof(void*),
    /// and should ideally be less than memory page size. A caveat of rpmalloc
    /// internals is that this must also be strictly less than the span size (default 64KiB)    
    pub fn rpaligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment, and zero initialize it.
    /// Alignment must be a power of two and a multiple of sizeof(void*),
    /// and should ideally be less than memory page size. A caveat of rpmalloc
    /// internals is that this must also be strictly less than the span size (default 64KiB)    
    pub fn rpaligned_calloc(alignment: size_t, num: size_t, size: size_t) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment.
    /// Alignment must be a power of two and a multiple of sizeof(void*),
    /// and should ideally be less than memory page size. A caveat of rpmalloc
    /// internals is that this must also be strictly less than the span size (default 64KiB)
    pub fn rpmemalign(alignment: size_t, size: size_t) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment.
    /// Alignment must be a power of two and a multiple of sizeof(void*),
    /// and should ideally be less than memory page size. A caveat of rpmalloc
    /// internals is that this must also be strictly less than the span size (default 64KiB)
    pub fn rpposix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;

    /// Query the usable size of the given memory block (from given pointer to the end of block)
    pub fn rpmalloc_usable_size(ptr: *mut c_void) -> size_t;
}
