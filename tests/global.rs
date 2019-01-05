use rpmalloc::*;

#[global_allocator]
static ALLOC: RpMalloc = RpMalloc;

#[test]
fn test() {
    // do some string allocations
    let s1 = "hej".to_string();
    let s2 = format!("test: {}", s1);

    // do a big vector allocation
    let mut v = Vec::<String>::with_capacity(100_000);
    v.push(s2);
}
