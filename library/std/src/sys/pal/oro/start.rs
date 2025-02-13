use crate::ffi::{c_char, c_int};
use crate::ptr;

extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[no_mangle]
#[allow(unused)]
pub extern "C" fn _start() -> ! {
    // SAFETY: This is the only time `init()` is called.
    unsafe {
        crate::sys::thread_local::key::init();
    }
    // SAFETY: The FFI is 'us', controlled entirely by the stdlib.
    unsafe {
        let _ = main(0, ptr::null());
    }
    // SAFETY: We're aware of what `terminate` does. This is safe and expected.
    unsafe {
		super::terminate::terminate()
    };
}
