use std::ffi::*;

#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> i32 {
    a + b
}