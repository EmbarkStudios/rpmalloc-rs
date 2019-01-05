#![allow(non_snake_case, non_camel_case_types)]

mod tests;

pub use libc::{c_int, c_uint, c_void, size_t};
pub type c_bool = c_int;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_global_statistics_t {
    /// Current amount of virtual memory mapped (only if ENABLE_STATISTICS=1)
    pub mapped: size_t,
    /// Current amount of memory in global caches for small and medium sizes (<64KiB)
    pub cached: size_t,
    /// Total amount of memory mapped (only if ENABLE_STATISTICS=1)
    pub mapped_total: size_t,
    /// Total amount of memory unmapped (only if ENABLE_STATISTICS=1)
    pub unmapped_total: size_t,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct rpmalloc_thread_statistics_t {
    /// Current number of bytes available for allocation from active spans
    pub active: size_t,
    /// Current number of bytes available in thread size class caches
    pub sizecache: size_t,
    /// Current number of bytes available in thread span caches
    pub spancache: size_t,
    /// Current number of bytes in pending deferred deallocations
    pub deferred: size_t,
    /// Total number of bytes transitioned from thread cache to global cache
    pub thread_to_global: size_t,
    /// Total number of bytes transitioned from global cache to thread cache
    pub global_to_thread: size_t,
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
    void* (*memory_map)(size_t size, size_t* offset);
    //! Unmap the memory pages starting at address and spanning the given number of bytes.
    //  If release is set to non-zero, the unmap is for an entire span range as returned by
    //  a previous call to memory_map and that the entire range should be released. The
    //  release argument holds the size of the entire span range. If release is set to 0,
    //  the unmap is a partial decommit of a subset of the mapped memory range.
    //  If you set a memory_unmap function, you must also set a memory_map function or
    //  else the default implementation will be used for both.
    void (*memory_unmap)(void* address, size_t size, size_t offset, size_t release);
    //! Size of memory pages. The page size MUST be a power of two. All memory mapping
    //  requests to memory_map will be made with size set to a multiple of the page size.
    size_t page_size;
    //! Size of a span of memory blocks. MUST be a power of two, and in [4096,262144]
    //  range (unless 0 - set to 0 to use the default span size).
    size_t span_size;
    //! Number of spans to map at each request to map new virtual memory blocks. This can
    //  be used to minimize the system call overhead at the cost of virtual memory address
    //  space. The extra mapped pages will not be written until actually used, so physical
    //  committed memory should not be affected in the default implementation. Will be
    //  aligned to a multiple of spans that match memory page size in case of huge pages.
    size_t span_map_count;
    //! Enable use of large/huge pages
    int enable_huge_pages;
    //! Debug callback if memory guards are enabled. Called if a memory overwrite is detected
    void (*memory_overwrite)(void* address);
} rpmalloc_config_t;
*/

extern "C" {
    pub fn rpmalloc_initialize() -> c_int;
    //extern int rpmalloc_initialize_config(const rpmalloc_config_t* config);
    //extern const rpmalloc_config_t* rpmalloc_config(void);
    pub fn rpmalloc_finalize();
    pub fn rpmalloc_thread_initialize();
    pub fn rpmalloc_thread_finalize();
    pub fn rpmalloc_thread_collect();
    pub fn rpmalloc_is_thread_initialized() -> c_bool;
    pub fn rpmalloc_thread_statistics(stats: *mut rpmalloc_thread_statistics_t);
    pub fn rpmalloc_global_statistics(stats: *mut rpmalloc_global_statistics_t);
    pub fn rpmalloc(size: size_t) -> *mut c_void;
    pub fn rpfree(ptr: *mut c_void);
    pub fn rpcalloc(num: size_t, size: size_t) -> *mut c_void;
    pub fn rprealloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn rpaligned_realloc(
        ptr: *mut c_void,
        alignment: size_t,
        size: size_t,
        oldsize: size_t,
        flags: c_uint,
    ) -> *mut c_void;
    pub fn rpaligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    pub fn rpmemalign(alignment: size_t, size: size_t) -> *mut c_void;
    //extern int rpposix_memalign(void **memptr, size_t alignment, size_t size);
    pub fn rpmalloc_usable_size(ptr: *mut c_void) -> size_t;
}
