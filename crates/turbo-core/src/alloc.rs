use crate::error::Error;
use core::alloc::Layout;
use core::ptr::NonNull;

/// Core allocator trait for the Turbo ecosystem.
///
/// This trait provides a stable interface for custom memory allocators (arenas, pools, slabs).
/// It mimics the nightly `core::alloc::Allocator` API but remains compatible with stable Rust.
pub trait TurboAlloc {
    /// Attempts to allocate a block of memory according to the specified layout.
    ///
    /// On success, returns a `NonNull<[u8]>` pointer to the allocated memory.
    /// On failure, returns `Error::AllocError`.
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, Error>;

    /// Deallocates the memory referenced by `ptr` with the specified `layout`.
    ///
    /// # Safety
    /// * `ptr` must denote a block of memory currently allocated via this allocator.
    /// * `layout` must fit the memory block pointed to by `ptr`.
    unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout);

    /// Behaves like `alloc`, but returns zero-initialized memory.
    fn alloc_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        let ptr = self.alloc(layout)?;
        // Safety: ptr is valid for writes of size layout.size()
        unsafe {
            core::ptr::write_bytes(ptr.as_ptr() as *mut u8, 0, layout.size());
        }
        Ok(ptr)
    }

    /// Attempts to grow or shrink the memory block referenced by `ptr`.
    ///
    /// # Safety
    /// * `ptr` must denote a block of memory currently allocated via this allocator.
    /// * `old_layout` must fit the memory block pointed to by `ptr`.
    /// * `new_layout.size()` must be greater than or equal to `old_layout.size()`.
    unsafe fn realloc(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, Error> {
        if new_layout.size() == old_layout.size() {
            return Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()));
        }

        let new_ptr = self.alloc(new_layout)?;
        let copy_size = core::cmp::min(old_layout.size(), new_layout.size());

        // Safety:
        // - new_ptr is valid for writes of size new_layout.size()
        // - ptr is valid for reads of size old_layout.size()
        // - copy_size is the minimum, so it is safe to read/write.
        // - The memory blocks do not overlap because new_ptr is newly allocated.
        core::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr() as *mut u8, copy_size);
        self.dealloc(ptr, old_layout);

        Ok(new_ptr)
    }
}

/// A tracking/debug allocator wrapper that monitors total allocated bytes.
///
/// This is extremely useful for profiling, diagnostics, ensuring no memory leaks,
/// and collecting runtime metrics. It wraps any other implementation of [`TurboAlloc`].
pub struct TrackingAlloc<A: TurboAlloc> {
    inner: A,
    allocated: core::sync::atomic::AtomicUsize,
}

impl<A: TurboAlloc> TrackingAlloc<A> {
    /// Creates a new `TrackingAlloc` wrapping the provided allocator.
    pub const fn new(inner: A) -> Self {
        Self {
            inner,
            allocated: core::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Returns the total number of bytes currently allocated and tracked by this allocator.
    pub fn allocated_bytes(&self) -> usize {
        self.allocated.load(core::sync::atomic::Ordering::Relaxed)
    }
}

impl<A: TurboAlloc> TurboAlloc for TrackingAlloc<A> {
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        let ptr = self.inner.alloc(layout)?;
        self.allocated
            .fetch_add(layout.size(), core::sync::atomic::Ordering::Relaxed);
        Ok(ptr)
    }

    unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout) {
        self.inner.dealloc(ptr, layout);
        self.allocated
            .fetch_sub(layout.size(), core::sync::atomic::Ordering::Relaxed);
    }
}

/// An implementation of [`TurboAlloc`] that delegates memory operations to the standard heap allocator.
///
/// This allocator is only available when the `std` or `alloc` features are active.
#[cfg(any(feature = "std", feature = "alloc"))]
pub struct GlobalAlloc;

#[cfg(any(feature = "std", feature = "alloc"))]
impl TurboAlloc for GlobalAlloc {
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, Error> {
        // Safety: layout size must be non-zero (guaranteed by Layout API)
        unsafe {
            let ptr = alloc_crate::alloc::alloc(layout);
            if ptr.is_null() {
                Err(Error::AllocError)
            } else {
                Ok(NonNull::slice_from_raw_parts(
                    NonNull::new_unchecked(ptr),
                    layout.size(),
                ))
            }
        }
    }

    unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout) {
        // Safety: deallocating ptr must follow safety contract of global allocator
        alloc_crate::alloc::dealloc(ptr.as_ptr(), layout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::alloc::Layout;

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn test_global_alloc() {
        let alloc = GlobalAlloc;
        let layout = Layout::from_size_align(128, 8).unwrap();

        let ptr = alloc.alloc(layout).unwrap();
        assert_eq!(ptr.len(), 128);

        // Test writing/reading
        let slice = unsafe { core::slice::from_raw_parts_mut(ptr.as_ptr() as *mut u8, 128) };
        slice[0] = 42;
        slice[127] = 24;
        assert_eq!(slice[0], 42);
        assert_eq!(slice[127], 24);

        // Test realloc to a larger size
        let larger_layout = Layout::from_size_align(256, 8).unwrap();
        let realloc_ptr = unsafe {
            alloc
                .realloc(
                    NonNull::new(ptr.as_ptr() as *mut u8).unwrap(),
                    layout,
                    larger_layout,
                )
                .unwrap()
        };
        assert_eq!(realloc_ptr.len(), 256);
        let larger_slice =
            unsafe { core::slice::from_raw_parts_mut(realloc_ptr.as_ptr() as *mut u8, 256) };
        assert_eq!(larger_slice[0], 42); // old contents copied

        // Clean up
        unsafe {
            alloc.dealloc(
                NonNull::new(realloc_ptr.as_ptr() as *mut u8).unwrap(),
                larger_layout,
            );
        }
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn test_tracking_alloc() {
        let tracker = TrackingAlloc::new(GlobalAlloc);
        let layout = Layout::from_size_align(64, 8).unwrap();

        assert_eq!(tracker.allocated_bytes(), 0);

        let ptr = tracker.alloc(layout).unwrap();
        assert_eq!(tracker.allocated_bytes(), 64);

        unsafe {
            tracker.dealloc(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);
        }
        assert_eq!(tracker.allocated_bytes(), 0);
    }
}
