#![crate_type = "cdylib"]
#![crate_name = "rusttest"]

#[no_mangle]
pub extern fn callable_from_c(x: i32) -> i32 {
    x % 3
}