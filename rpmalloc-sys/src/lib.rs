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

pub use libc::{c_char, c_int, c_uint, c_void, size_t};

/// Maximum alignment accepted by the aligned allocation entry points
pub const RPMALLOC_MAX_ALIGNMENT: size_t = 256 * 1024;

/// Flag to [`rpaligned_realloc()`] to not preserve content in reallocation
pub const RPMALLOC_NO_PRESERVE: c_uint = 1;
/// Flag to [`rpaligned_realloc()`] to fail and return a null pointer if the grow cannot be done
/// in-place, in which case the original pointer is still valid (just like a call to `realloc()`
/// which fails to allocate a new block).
pub const RPMALLOC_GROW_OR_FAIL: c_uint = 2;

/// Global memory statistics
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_global_statistics_t {
    /// Current amount of virtual memory mapped, all of which might not have been committed (only if `ENABLE_STATISTICS=1`)
    pub mapped: size_t,
    /// Peak amount of virtual memory mapped, all of which might not have been committed (only if `ENABLE_STATISTICS=1`)
    pub mapped_peak: size_t,
    /// Running counter of total amount of memory committed (only if `ENABLE_STATISTICS=1`)
    pub committed: size_t,
    /// Running counter of total amount of memory decommitted (only if `ENABLE_STATISTICS=1`)
    pub decommitted: size_t,
    /// Current amount of virtual memory active and committed (only if `ENABLE_STATISTICS=1`)
    pub active: size_t,
    /// Peak amount of virtual memory active and committed (only if `ENABLE_STATISTICS=1`)
    pub active_peak: size_t,
    /// Current amount of memory allocated in huge block allocations, i.e blocks above the largest
    /// size class (only if `ENABLE_STATISTICS=1`)
    pub huge_alloc: size_t,
    /// Peak amount of memory allocated in huge block allocations, i.e blocks above the largest
    /// size class (only if `ENABLE_STATISTICS=1`)
    pub huge_alloc_peak: size_t,
    /// Current heap count (only if `ENABLE_STATISTICS=1`)
    pub heap_count: size_t,
}

/// Number of page types (small, medium-small, medium-large, large, huge) reported in
/// [`rpmalloc_thread_statistics_t::span_use`]
pub const RPMALLOC_PAGE_TYPE_COUNT: usize = 5;

/// Number of size classes reported in [`rpmalloc_thread_statistics_t::size_use`]
pub const RPMALLOC_SIZE_CLASS_COUNT: usize = 128;

/// Span statistics for a single page type
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_thread_span_statistics_t {
    /// Currently mapped number of spans of this page type
    pub current: size_t,
    /// Number of raw memory map calls for this page type resulting in actual OS mmap calls (only
    /// if `ENABLE_STATISTICS=1`)
    pub map_calls: size_t,
}

/// Memory size statistics for a single size class
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_thread_size_statistics_t {
    /// Current number of allocations
    pub alloc_current: size_t,
    /// Peak number of allocations
    pub alloc_peak: size_t,
    /// Total number of allocations
    pub alloc_total: size_t,
    /// Total number of frees
    pub free_total: size_t,
}

/// Memory statistics for a thread
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rpmalloc_thread_statistics_t {
    /// Current number of bytes available in thread size class caches (only if `ENABLE_STATISTICS=1`)
    pub sizecache: size_t,
    /// Current number of bytes available in thread page caches (only if `ENABLE_STATISTICS=1`)
    pub spancache: size_t,
    /// Per page type span statistics, indexed by page type
    pub span_use: [rpmalloc_thread_span_statistics_t; RPMALLOC_PAGE_TYPE_COUNT],
    /// Per size class statistics (only if `ENABLE_STATISTICS=1`)
    pub size_use: [rpmalloc_thread_size_statistics_t; RPMALLOC_SIZE_CLASS_COUNT],
}

/// Custom memory mapping interface, passed to [`rpmalloc_initialize()`]
///
/// Not source compatible with the `memory_map`/`memory_unmap` pair from rpmalloc 1.4: mapping is
/// now split into reserve (`memory_map`) plus on-demand `memory_commit`/`memory_decommit`.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_interface_t {
    /// Map memory pages for the given number of bytes. The returned address MUST be aligned to the
    /// given alignment, which will always be either 0 or the span size. The function can store an
    /// alignment offset in the `offset` variable in case it performs alignment and the returned
    /// pointer is offset from the actual start of the memory region due to this alignment. This
    /// alignment offset will be passed to [`Self::memory_unmap`]. The mapped size can be stored in
    /// the `mapped_size` variable, which will also be passed to [`Self::memory_unmap`] as the
    /// release parameter once the entire mapped region is ready to be released. If you set a
    /// `memory_map` function, you must also set a [`Self::memory_unmap`] function or else the
    /// default implementation will be used for both. This function must be thread safe.
    pub memory_map: Option<
        unsafe extern "C" fn(
            size: size_t,
            alignment: size_t,
            offset: *mut size_t,
            mapped_size: *mut size_t,
        ) -> *mut c_void,
    >,
    /// Commit a range of memory pages. Return non-zero if the operation failed and the address
    /// range could not be committed.
    pub memory_commit: Option<unsafe extern "C" fn(address: *mut c_void, size: size_t) -> c_int>,
    /// Decommit a range of memory pages. Return non-zero if the operation failed and the address
    /// range could not be decommitted.
    pub memory_decommit: Option<unsafe extern "C" fn(address: *mut c_void, size: size_t) -> c_int>,
    /// Unmap the memory pages starting at address and spanning the given number of bytes. If you
    /// set a `memory_unmap` function, you must also set a [`Self::memory_map`] function or else the
    /// default implementation will be used for both. This function must be thread safe.
    pub memory_unmap:
        Option<unsafe extern "C" fn(address: *mut c_void, offset: size_t, mapped_size: size_t)>,
    /// Called when a call to map memory pages fails (out of memory). If this callback is not set or
    /// returns zero the library will return a null pointer in the allocation call. If this callback
    /// returns non-zero the map call will be retried. The argument passed is the number of bytes
    /// that was requested in the map call. Only used if the default system memory map function is
    /// used ([`Self::memory_map`] is not set).
    pub map_fail_callback: Option<unsafe extern "C" fn(size: size_t) -> c_int>,
    /// Called when an assert fails, if asserts are enabled. Will use the standard `assert()` if
    /// this is not set.
    pub error_callback: Option<unsafe extern "C" fn(message: *const c_char)>,
}

/// Allocator configuration, passed to [`rpmalloc_initialize_config()`]
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_config_t {
    /// Size of memory pages. The page size MUST be a power of two. All memory mapping requests to
    /// [`rpmalloc_interface_t::memory_map`] will be made with size set to a multiple of the page
    /// size. Set to 0 to use the OS default page size.
    pub page_size: size_t,
    /// Enable use of large/huge pages. If this flag is set to non-zero and page size is zero, the
    /// allocator will try to enable huge pages and auto detect the configuration. If this is set to
    /// non-zero and [`Self::page_size`] is also non-zero, the allocator will assume huge pages have
    /// been configured and enabled prior to initializing the allocator.
    pub enable_huge_pages: c_int,
    /// Enable use of transparent huge pages, advising the kernel to back large memory mappings with
    /// huge pages without requiring a preallocated huge page pool. Unlike
    /// [`Self::enable_huge_pages`] this does not affect page size or accounting, and unused memory
    /// ranges can still be decommitted. Ignored if [`Self::enable_huge_pages`] is in effect or if
    /// the platform has no transparent huge page support. After initialization the config value
    /// reflects if transparent huge pages are actually used.
    pub enable_thp: c_int,
    /// Disable decommitting unused pages when the allocator determines the memory pressure is low
    /// and there are enough active pages cached. If set to 1, keep all pages committed.
    pub disable_decommit: c_int,
    /// Allocated page name for systems supporting it, to be able to distinguish among anonymous
    /// regions.
    pub page_name: *const c_char,
    /// Allocated huge page name for systems supporting it, to be able to distinguish among
    /// anonymous regions.
    pub huge_page_name: *const c_char,
    /// Unmap all memory on finalize if set to 1. Normally you can let the OS unmap all pages when
    /// the process exits, but if using rpmalloc in a dynamic library you might want to unmap all
    /// pages when the dynamic library unloads to avoid process memory leaks and bloat.
    pub unmap_on_finalize: c_int,
    /// Disable the transparent huge page feature on a per-process basis, rather than system-wide
    /// (which is done via `/sys/kernel/mm/transparent_hugepage/enabled`).
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub disable_thp: c_int,
}

extern "C" {
    /// Initialize the allocator, optionally with a custom memory mapping interface
    ///
    /// Passing a null `memory_interface` selects the built-in one. Note that the allocator also
    /// initializes itself lazily on the first allocation, so calling this is only required to
    /// install a custom interface or configuration.
    pub fn rpmalloc_initialize(memory_interface: *mut rpmalloc_interface_t) -> c_int;

    /// Initialize the allocator with a custom memory mapping interface and configuration
    ///
    /// `config` is an in/out parameter: it is read for the requested configuration and written back
    /// with the configuration that is actually in effect.
    pub fn rpmalloc_initialize_config(
        memory_interface: *mut rpmalloc_interface_t,
        config: *mut rpmalloc_config_t,
    ) -> c_int;

    /// Get the allocator configuration currently in effect
    pub fn rpmalloc_config() -> *const rpmalloc_config_t;

    /// Finalize allocator
    pub fn rpmalloc_finalize();

    /// Initialize allocator for calling thread
    pub fn rpmalloc_thread_initialize();

    /// Finalize allocator for calling thread
    pub fn rpmalloc_thread_finalize();

    /// Perform deferred deallocations pending for the calling thread heap
    pub fn rpmalloc_thread_collect();

    /// Query if allocator is initialized for calling thread
    pub fn rpmalloc_is_thread_initialized() -> c_int;

    /// Get per-thread statistics
    pub fn rpmalloc_thread_statistics(stats: *mut rpmalloc_thread_statistics_t);

    /// Get global statistics
    pub fn rpmalloc_global_statistics(stats: *mut rpmalloc_global_statistics_t);

    /// Dump all statistics in human readable format to file (should be a `FILE*`)
    pub fn rpmalloc_dump_statistics(file: *mut c_void);

    /// Allocate a memory block of at least the given size
    pub fn rpmalloc(size: size_t) -> *mut c_void;

    /// Allocate a zero initialized memory block of at least the given size
    pub fn rpzalloc(size: size_t) -> *mut c_void;

    /// Free the given memory block
    pub fn rpfree(ptr: *mut c_void);

    /// Allocate a memory block of at least the given size and zero initialize it
    pub fn rpcalloc(num: size_t, size: size_t) -> *mut c_void;

    /// Reallocate the given block to at least the given size
    pub fn rprealloc(ptr: *mut c_void, size: size_t) -> *mut c_void;

    /// Reallocate the given block to at least the given size and alignment, with optional control
    /// flags (see [`RPMALLOC_NO_PRESERVE`] and [`RPMALLOC_GROW_OR_FAIL`])
    ///
    /// Alignment must be a power of two and a multiple of `size_of::<*const c_void>()`, and should
    /// ideally be less than the memory page size. It must not exceed [`RPMALLOC_MAX_ALIGNMENT`].
    pub fn rpaligned_realloc(
        ptr: *mut c_void,
        alignment: size_t,
        size: size_t,
        oldsize: size_t,
        flags: c_uint,
    ) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment
    ///
    /// Alignment must be a power of two and a multiple of `size_of::<*const c_void>()`, and should
    /// ideally be less than the memory page size. It must not exceed [`RPMALLOC_MAX_ALIGNMENT`].
    pub fn rpaligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;

    /// Allocate a zero initialized memory block of at least the given size and alignment
    ///
    /// Alignment must be a power of two and a multiple of `size_of::<*const c_void>()`, and should
    /// ideally be less than the memory page size. It must not exceed [`RPMALLOC_MAX_ALIGNMENT`].
    pub fn rpaligned_zalloc(alignment: size_t, size: size_t) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment, and zero initialize it
    ///
    /// Alignment must be a power of two and a multiple of `size_of::<*const c_void>()`, and should
    /// ideally be less than the memory page size. It must not exceed [`RPMALLOC_MAX_ALIGNMENT`].
    pub fn rpaligned_calloc(alignment: size_t, num: size_t, size: size_t) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment
    ///
    /// Alignment must be a power of two and a multiple of `size_of::<*const c_void>()`, and should
    /// ideally be less than the memory page size. It must not exceed [`RPMALLOC_MAX_ALIGNMENT`].
    pub fn rpmemalign(alignment: size_t, size: size_t) -> *mut c_void;

    /// Allocate a memory block of at least the given size and alignment
    ///
    /// Alignment must be a power of two and a multiple of `size_of::<*const c_void>()`, and should
    /// ideally be less than the memory page size. It must not exceed [`RPMALLOC_MAX_ALIGNMENT`].
    pub fn rpposix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;

    /// Query the usable size of the given memory block (from the given pointer to the end of block)
    pub fn rpmalloc_usable_size(ptr: *mut c_void) -> size_t;

    /// Dummy empty function for forcing linker symbol inclusion
    pub fn rpmalloc_linker_reference();
}
