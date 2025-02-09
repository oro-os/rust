use oro::alloc::HeapAllocator;
use crate::alloc::{GlobalAlloc, Layout, System};

static ALLOCATOR: HeapAllocator = HeapAllocator::new();

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: This is a forwarding call; safety must be upheld by the caller.
        unsafe { ALLOCATOR.alloc(layout) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: This is a forwarding call; safety must be upheld by the caller.
        unsafe { ALLOCATOR.dealloc(ptr, layout) }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: This is a forwarding call; safety must be upheld by the caller.
        unsafe { ALLOCATOR.realloc(ptr, layout, new_size) }
    }
}
