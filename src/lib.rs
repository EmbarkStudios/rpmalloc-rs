pub mod ffi {
    pub use rpmalloc_sys::*;
}

use std::alloc::{GlobalAlloc, Layout};
use std::mem::MaybeUninit;

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
    pub fn global_stats() -> ffi::rpmalloc_global_statistics_t {
        let mut stats: MaybeUninit<ffi::rpmalloc_global_statistics_t> = MaybeUninit::uninit();
        unsafe {
            ffi::rpmalloc_global_statistics(stats.as_mut_ptr());
            stats.assume_init()
        }
    }

    pub fn thread_stats() -> ffi::rpmalloc_thread_statistics_t {
        let mut stats: MaybeUninit<ffi::rpmalloc_thread_statistics_t> = MaybeUninit::uninit();
        unsafe {
            ffi::rpmalloc_thread_statistics(stats.as_mut_ptr());
            stats.assume_init()
        }
    }
}
