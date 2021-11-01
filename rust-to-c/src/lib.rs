#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sum_vec(vec: *const i32, len: i32) -> i32 {
    let rust_vec = unsafe { std::slice::from_raw_parts(vec, len as usize) };
    rust_vec.iter().sum()
}
