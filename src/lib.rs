pub mod ffi {
    pub use rpmalloc_sys::*;
}

use std::alloc::{GlobalAlloc, Layout};
//use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct RpMalloc;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

unsafe impl GlobalAlloc for RpMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if !INITIALIZED.load(Ordering::Relaxed) {
            let result = ffi::rpmalloc_initialize();
            assert_eq!(result, 0);
            INITIALIZED.store(true, Ordering::Relaxed);
        }

        ffi::rpaligned_alloc(layout.align(), layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::rpfree(ptr as *mut ffi::c_void)
    }
}
