use std::os::raw::*;

#[no_mangle]
extern "C" fn hello_rust() -> c_int {
    println!("Hello from Rust!");
    0
}
