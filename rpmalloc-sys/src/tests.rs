use super::*;

use std::ptr;

#[test]
fn init() {
    let result = unsafe { rpmalloc_initialize(ptr::null_mut()) };
    assert_eq!(result, 0);
}

#[test]
fn simple_alloc() {
    let result = unsafe { rpmalloc_initialize(ptr::null_mut()) };
    assert_eq!(result, 0);

    let ptr = unsafe { rpmalloc(100) };
    assert!(!ptr.is_null());

    let usable_size = unsafe { rpmalloc_usable_size(ptr) };
    assert!(usable_size >= 100);

    unsafe { rpfree(ptr) };
}

extern "C" {
    fn rpmalloc_sys_sizeof(index: c_int) -> size_t;
    fn rpmalloc_sys_offsetof_config_unmap_on_finalize() -> size_t;
    fn rpmalloc_sys_offsetof_interface_error_callback() -> size_t;
}

/// The FFI structs are written to by the C side, so a layout mismatch corrupts the caller's stack
/// rather than failing to link. Compare against the C definitions directly.
#[test]
fn layout_matches_c() {
    use std::mem::size_of;

    let sizes = [
        (
            "rpmalloc_global_statistics_t",
            size_of::<rpmalloc_global_statistics_t>(),
        ),
        (
            "rpmalloc_thread_statistics_t",
            size_of::<rpmalloc_thread_statistics_t>(),
        ),
        ("rpmalloc_interface_t", size_of::<rpmalloc_interface_t>()),
        ("rpmalloc_config_t", size_of::<rpmalloc_config_t>()),
    ];

    for (index, (name, rust_size)) in sizes.iter().enumerate() {
        let c_size = unsafe { rpmalloc_sys_sizeof(index as c_int) };
        assert_eq!(*rust_size, c_size, "{} size mismatch", name);
    }

    // The trailing fields are the ones a padding or `cfg` mistake would silently shift.
    let config = rpmalloc_config_t::default();
    let base = &config as *const _ as usize;
    assert_eq!(
        &config.unmap_on_finalize as *const _ as usize - base,
        unsafe { rpmalloc_sys_offsetof_config_unmap_on_finalize() },
        "rpmalloc_config_t::unmap_on_finalize offset mismatch"
    );

    let interface = rpmalloc_interface_t::default();
    let base = &interface as *const _ as usize;
    assert_eq!(
        &interface.error_callback as *const _ as usize - base,
        unsafe { rpmalloc_sys_offsetof_interface_error_callback() },
        "rpmalloc_interface_t::error_callback offset mismatch"
    );
}
