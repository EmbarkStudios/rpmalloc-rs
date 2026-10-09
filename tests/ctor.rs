//! Regression test for the macOS crash that made Traverse disable rpmalloc on that platform.
//!
//! A static constructor - `__DATA,__mod_init_func` on Apple, `.init_array` elsewhere - that
//! allocates is the trigger. rpmalloc 1.4 with `ENABLE_PRELOAD` installs its own constructor to
//! call `rpmalloc_initialize()`, and on Apple it keeps the thread heap in a `pthread_key_t` that
//! is only created there. A constructor that runs earlier therefore allocates against
//! `pthread_getspecific(0)`, which is a reserved Darwin TSD slot holding an unrelated non-null
//! pointer, so the "not initialized yet" check passes and the allocator walks a bogus heap:
//! SIGSEGV before `main`, or `Invalid mmap size` / `Span block count corrupted` with `asserts`.
//!
//! Registering the constructor through the `__mod_init_func` section directly, rather than via the
//! `ctor` crate, is what pins it ahead of rpmalloc's own constructor; link order decides otherwise.

#![allow(unsafe_code)]

use std::collections::BTreeMap;
use std::sync::Mutex;

#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

/// Stand-in for the global feature-toggle registry that `#[ctor]`s populated in the original crash.
static REGISTRY: Mutex<BTreeMap<String, Vec<u64>>> = Mutex::new(BTreeMap::new());

extern "C" fn register_toggles() {
    let mut registry = REGISTRY.lock().unwrap();
    for i in 0..16u64 {
        // Spread over several size classes so more than one of rpmalloc's free lists is touched.
        registry.insert(format!("toggle-{}", i), (0..(i + 1) * 64).collect());
    }
}

#[used]
#[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
#[cfg_attr(
    not(any(target_vendor = "apple", target_os = "windows")),
    link_section = ".init_array"
)]
static REGISTER_TOGGLES: extern "C" fn() = register_toggles;

#[test]
fn allocations_from_a_constructor_survive_until_main() {
    let registry = REGISTRY.lock().unwrap();
    assert_eq!(registry.len(), 16);
    for (name, payload) in registry.iter() {
        assert!(
            payload.iter().copied().eq(0..payload.len() as u64),
            "{} payload was corrupted",
            name
        );
    }
}

#[test]
fn allocating_across_threads_after_constructors() {
    // Cross-thread frees exercise the deferred free path fed by the constructor-owned heap.
    let handles = (0..8)
        .map(|i| {
            std::thread::spawn(move || {
                (0..512)
                    .map(|j| vec![i as u8; j * 16 + 1])
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        drop(handle.join().unwrap());
    }
}
