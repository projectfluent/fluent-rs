
static mut INTL_EXAMPLE_CONSTRUCTS: u32 = 0;
fn increment_constructs() {
    unsafe {
        INTL_EXAMPLE_CONSTRUCTS += 1;
    }
}

fn get_constructs_count() -> u32 {
    unsafe { INTL_EXAMPLE_CONSTRUCTS }
}

#[test]
fn test_memoizable() {}
