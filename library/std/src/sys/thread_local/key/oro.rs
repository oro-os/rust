use core::{arch::asm, sync::atomic::{AtomicUsize, Ordering::SeqCst}, alloc::Layout, ptr::NonNull};
use alloc::vec::Vec;
use crate::sys::pal::spin_mutex::Mutex;

// TODO(qix-): This very crudely uses a lock to allocate the destructors,
// TODO(qix-): but mutexes (RwLocks) are not yet implemented for Oro.

static NUM_KEYS: AtomicUsize = AtomicUsize::new(0);
static DESTRUCTORS: Mutex<Vec<Option<unsafe extern "C" fn(*mut u8)>>> = Mutex::new(Vec::new());

pub type Key = usize;

#[doc(hidden)]
static TLS_INIT: usize = 0;

#[inline]
pub fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    // We now have a lock on the destructors.
    let key = NUM_KEYS.fetch_add(1, SeqCst);

    if let Some(dtor) = dtor {
        let mut destructors = DESTRUCTORS.lock();

        if key >= destructors.len() {
            destructors.resize_with(key + 1, || None);
        }

        destructors[key] = Some(dtor);
    }

    key
}

/// Initializes the current thread for TLS.
///
/// # Safety
/// This function must be called exactly once per thread.
pub unsafe fn init() {
    // Initialize simply to a single `0` value.
    // This allows initialization not to incur a heap allocation
    // right off the bat for the common case of no TLS being used.
    // SAFETY: Safe for thread_id = 0.
    unsafe {
        debug_assert!(::oro::tls::tls_base(0).expect("tls error").is_null(), "tls init() called but TLS already initialized");
        ::oro::tls::set_tls_base(0, NonNull::new_unchecked(::core::ptr::from_ref(&TLS_INIT).cast_mut().cast())).expect("failed to set tls base");
    }
}

/// Gets the `usize` value for the given offset.
///
/// # Safety
/// This is only safe if
/// - [`init()`] has been called, in all cases.
/// - the offset is `0`, or
/// - the offset minus 1 is less than the current allocated key slots
///   for the thread.
unsafe fn value_at_offset(offset: usize) -> usize {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            let v: usize;
            // SAFETY: Safety considerations offloaded to the caller.
            unsafe {
                asm!(
                    "mov {o}, fs:[{i}]",
                    o = out(reg) v,
                    i = in(reg) offset,
                    options(nostack),
                );
            }

            return v;
        }
        else {
            // SAFETY: Safety considerations offloaded to the caller.
            unsafe {
                return *(::oro::tls::tls_base().cast::<usize>().wrapping_add(offset));
            }
        }
    }
}

/// Sets the value for the given key.
///
/// Any existing value for the key is overwritten; non-null
/// values are deallocated.
///
/// # Safety
/// [`init()`] must be called on the current thread before this function is called.
#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    // Get number of allocated keys.
    // SAFETY: Calling with `0`, and init considerations offloaded to caller.
    let num_allocated = unsafe { value_at_offset(0) };

    if key >= num_allocated {
        // SAFETY: Allocation has non-zero size, guaranteed.
        let new_block = unsafe {::alloc::alloc::alloc_zeroed(
            Layout::array::<usize>(key + 1).expect("tls layout failed"),
        )};

        assert!(
            !new_block.is_null(),
            "failed to allocate TLS block",
        );

        // Get a pointer to the old block.
        // SAFETY: Safety considerations offloaded to the caller. Further, thread_id of 0
        // SAFETY: is always safe.
        let current_block_ptr = unsafe { ::oro::tls::tls_base(0).expect("no tls support").cast::<usize>() };

        debug_assert!(!current_block_ptr.is_null());

        // Copy the old block to the new block.
        // SAFETY: Both pointers are valid and no overlap is possible.
        unsafe {
            ::core::ptr::copy_nonoverlapping(current_block_ptr, new_block.cast(), num_allocated + 1);
        }

        // Set the new allocated size.
        // SAFETY: The pointer is valid and should be properly aligned (we check).
        unsafe {
            debug_assert!(new_block.cast::<usize>().is_aligned());
            new_block.cast::<usize>().write(key + 1);
        }

        // Set the new block.
        // SAFETY: Assuming that the user has not themselves incurred UB
        // SAFETY: by modifying the TLS base pointer, this is safe, especially
        // SAFETY: with thread_id = 0.
        unsafe {
            ::oro::tls::set_tls_base(0, NonNull::new_unchecked(new_block)).expect("failed to set new tls base");
        }

        // Deallocate the old block (if it's not `TLS_INIT`).
        if current_block_ptr != ::core::ptr::from_ref(&TLS_INIT) {
            // SAFETY: Assuming that the user has not themselves incurred UB
            // SAFETY: by modifying the TLS base pointer, this is safe as we are
            // SAFETY: the ones who allocated it.
            unsafe {
                ::alloc::alloc::dealloc(
                    current_block_ptr.cast_mut().cast(),
                    Layout::array::<usize>(num_allocated + 1).expect("tls layout failed"),
                );
            }
        }
    }

    // Set the value.
    let old_value: usize;
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            // SAFETY: Safety considerations offloaded to the caller.
            unsafe {
                asm!(
                    "mov {o}, fs:[{i}]",
                    "mov fs:[{i}], {v}",
                    o = out(reg) old_value,
                    i = in(reg) key + 1,
                    v = in(reg) value.expose_provenance(),
                    options(nostack),
                );
            }
        }
        else {
            // SAFETY: Safety considerations offloaded to the caller.
            unsafe {
                old_value = *(::oro::tls::tls_base().cast::<usize>().wrapping_add(key + 1));
                *(::oro::tls::tls_base().cast::<usize>().wrapping_add(key + 1)) = value.expose_provenance();
            }
        }
    }

    // Call the destructor.
    // TODO(qix-): This is fine for now but technically racey; this will benefit
    // TODO(qix-): from a RwLock wrapper around the destructors.
    if old_value != 0 {
        let destructors = DESTRUCTORS.lock();
        let dtor = destructors.get(key).cloned();
        drop(destructors);
        if let Some(Some(dtor)) = dtor {
            // SAFETY: Safety considerations offloaded to the caller.
            // SAFETY: We're making sure we're passing a valid pointer.
            unsafe {
                dtor(::core::ptr::with_exposed_provenance_mut(old_value));
            }
        }
    }
}

/// Gets the value for the given key, or `null` if the key is not set
/// or is out of bounds.
///
/// # Safety
/// [`init()`] must be called on the current thread before this function is called.
#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    // Get the number of keys allocated.
    // SAFETY: Calling with `0`, and init considerations offloaded to caller.
    let num_allocated = unsafe { value_at_offset(0) };

    if key >= num_allocated {
        return ::core::ptr::null_mut();
    }

    // Get the value.
    // SAFETY: We've verified that the key is in bounds.
    let v = unsafe {value_at_offset(key + 1)};

    if v == 0 {
        ::core::ptr::null_mut()
    } else {
        ::core::ptr::with_exposed_provenance_mut(v)
    }
}

/// Destroys the key, removing the destructor if it exists.
///
/// # Safety
/// Caller must be aware that no values currently allocated with
/// the key, if any, will be deallocated after this call.
#[inline]
pub unsafe fn destroy(key: Key) {
    let mut destructors = DESTRUCTORS.lock();
    if let Some(dtor) = destructors.get_mut(key) {
        let _ = dtor.take();
    }
}
