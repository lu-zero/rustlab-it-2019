// main.rs
use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    static hi: *const c_char;
    fn hello_c();
}

fn main() {
    unsafe {
        let from_c = CStr::from_ptr(hi);
        hello_c();
        println!("{}", from_c.to_string_lossy());
    }
}
