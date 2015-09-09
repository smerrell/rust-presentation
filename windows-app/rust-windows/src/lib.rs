extern crate libc;

use std::ffi::CStr;
use std::str;
use libc::c_char;

#[no_mangle]
pub extern "C" fn foo(value1: u32, value2: u32) -> u32 {
    println!("Hello from rust!");
    value1 + value2
}

#[no_mangle]
pub extern "C" fn say_hello(name: *const c_char) {
    let c_value = unsafe { CStr::from_ptr(name).to_bytes() };
    match str::from_utf8(c_value) {
        Ok(name) => println!("Hello, {}", name),
        Err(err) => {
            println!("Uh oh! {}", err);
        }
    }
}

#[test]
fn it_works() {
}
