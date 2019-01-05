use rpmalloc::*;

#[global_allocator]
static ALLOC: RpMalloc = RpMalloc;

#[test]
fn test() {
    //let s1 = "hej";
    //   let s2 = format!("test: {}", s1);
}
