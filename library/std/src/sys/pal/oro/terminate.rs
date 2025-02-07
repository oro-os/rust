/// Terminates the current thread.
///
/// If the current thread is the last thread in module instance, the module
/// instance is unmounted from all rings and subsequently destroyed.
///
/// If the module instance was the last on a given ring, the ring is also destroyed.
///
/// Note that this does not guarantee that references to the module instance are invalidated;
/// in some cases the kernel may still respond to operations pertaining to this module instance,
/// invoked by other module instances that were previously interacting with this module instance,
/// in order to allow them to gracefully handle the termination of this module instance.
///
/// # Thread Cleanup
/// All resources allocated by the thread are freed. This includes, but is not limited to, memory
/// allocations and any ports.
///
/// Any ports that applications wish to continue using must be explicitly transferred to another
/// thread or module instance prior to calling this function.
///
/// # Safety
/// This function is inherently unsafe as it immediately terminates the current thread.
pub unsafe fn terminate() -> ! {
    // SAFETY: Safety considerations of termination have been forwarded to caller.
    unsafe {
        let _ = oro::syscall::set_raw(
            oro::id::iface::KERNEL_THREAD_V0,
            0, // self
            oro::key!("status"),
            oro::key!("term"), // (not a key, but a value)
        );
    }

    // That didn't work, try to force an abort.
    core::intrinsics::abort()
}
