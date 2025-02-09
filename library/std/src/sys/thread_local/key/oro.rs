pub type Key = u64;

#[inline]
pub fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    //todo!("std::sys::thread_local::key::oro::create()");
    0
}

#[inline]
pub unsafe fn set(_key: Key, _value: *mut u8) {
    //todo!("std::sys::thread_local::key::oro::set()");

}

#[inline]
pub unsafe fn get(_key: Key) -> *mut u8 {
    //todo!("std::sys::thread_local::key::oro::get()");
    crate::ptr::null_mut()
}

#[inline]
pub unsafe fn destroy(_key: Key) {
    //todo!("std::sys::thread_local::key::oro::destroy()");
}
