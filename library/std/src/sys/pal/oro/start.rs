use crate::ffi::{c_char, c_int};
use crate::ptr;

extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[no_mangle]
#[allow(unused)]
pub extern "C" fn _start() -> ! {
    // SAFETY: We're aware of what `terminate` does. This is safe and expected.
    unsafe {
        let _ = main(0, ptr::null());
		super::terminate::terminate()
    };
}
