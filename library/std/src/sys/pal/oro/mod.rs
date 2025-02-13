#![deny(unsafe_op_in_unsafe_fn)]

pub mod args;
pub mod env;
pub mod fs;
pub mod io;
pub mod net;
pub mod os;
pub mod pipe;
pub mod process;
pub mod stdio;
pub mod thread;
pub mod time;
pub mod terminate;
pub mod spin_mutex;

mod common;
pub use common::*;

mod start;

// This function is needed by the panic runtime. The symbol is named in
// pre-link args for the target specification, so keep that in sync.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_abort() {
    abort_internal();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // NOTE(qix-): Super naive implementation, but it should work for now.
    // NOTE(qix-): Needed for now since libc hasn't been adapted for Oro yet.
    unsafe {
        let mut i = 0;
        while i < n {
            *dest.add(i) = *src.add(i);
            i += 1;
        }
    }

    dest
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // NOTE(qix-): Super naive implementation, but it should work for now.
    // NOTE(qix-): Needed for now since libc hasn't been adapted for Oro yet.
    unsafe {
        let mut i = 0;
        while i < n {
            *dest.add(i) = *src.add(i);
            i += 1;
        }
    }

    dest
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    // NOTE(qix-): Super naive implementation, but it should work for now.
    // NOTE(qix-): Needed for now since libc hasn't been adapted for Oro yet.
    unsafe {
        let mut i = 0;
        while i < n {
            *s.add(i) = c as u8;
            i += 1;
        }
    }

    s
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    // NOTE(qix-): Super naive implementation, but it should work for now.
    // NOTE(qix-): Needed for now since libc hasn't been adapted for Oro yet.
    unsafe {
        let mut i = 0;
        while i < n {
            if *s1.add(i) != *s2.add(i) {
                return *s1.add(i) as i32 - *s2.add(i) as i32;
            }
            i += 1;
        }
    }
    0
}
