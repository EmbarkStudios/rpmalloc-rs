use super::*;

#[test]
fn init() {
    let result = unsafe { rpmalloc_initialize() };
    assert_eq!(result, 0);
}

#[test]
fn simple_alloc() {
    let result = unsafe { rpmalloc_initialize() };
    assert_eq!(result, 0);

    let ptr = unsafe { rpmalloc(100) };
    assert!(!ptr.is_null());

    let usable_size = unsafe { rpmalloc_usable_size(ptr) };
    assert!(usable_size >= 100);

    unsafe { rpfree(ptr) };
}
