/// Marker trait indicating that a type is safely representable as a raw byte slice.
///
/// This trait is useful for zero-copy deserialization, casting byte arrays to structs,
/// and fast serialization.
///
/// # Safety
/// Implementing this trait guarantees that the type has:
/// - A defined representation (e.g., `#[repr(C)]` or `#[repr(transparent)]`).
/// - No internal pointers, references, or cell-like structures (it must be `'static` and have plain data).
/// - No padding bytes (uninitialized memory), or is safe to read/write under all states.
///
/// # Examples
/// ```
/// use turbo_core::ZeroCopy;
///
/// #[derive(Copy, Clone)]
/// #[repr(C)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// unsafe impl ZeroCopy for Point {}
/// ```
pub unsafe trait ZeroCopy: Copy + Send + Sync + 'static {}

unsafe impl ZeroCopy for u8 {}
unsafe impl ZeroCopy for i8 {}
unsafe impl ZeroCopy for u16 {}
unsafe impl ZeroCopy for i16 {}
unsafe impl ZeroCopy for u32 {}
unsafe impl ZeroCopy for i32 {}
unsafe impl ZeroCopy for u64 {}
unsafe impl ZeroCopy for i64 {}
unsafe impl ZeroCopy for f32 {}
unsafe impl ZeroCopy for f64 {}
unsafe impl ZeroCopy for usize {}
unsafe impl ZeroCopy for isize {}

/// Trait to estimate the heap-allocated memory footprint of a data structure.
///
/// This is used to monitor allocations, track buffer size overheads, and enforce limits.
pub trait EstimateSize {
    /// Returns the total heap size allocated by this data structure in bytes.
    fn heap_size(&self) -> usize;

    /// Returns the total size of the structure, including the stack-allocated size of the object itself.
    fn total_size(&self) -> usize {
        core::mem::size_of_val(self) + self.heap_size()
    }
}

/// Blanket implementation of `EstimateSize` for all types implementing `ZeroCopy`.
///
/// Since zero-copy types are simple, plain data structures with stable layouts and no internal
/// heap pointers, their heap size is always `0`.
impl<T: ZeroCopy> EstimateSize for T {
    fn heap_size(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_marker() {
        fn assert_zero_copy<T: ZeroCopy>() {}
        assert_zero_copy::<u8>();
        assert_zero_copy::<u32>();
        assert_zero_copy::<f64>();
        assert_zero_copy::<usize>();
    }

    #[test]
    fn test_estimate_size() {
        let val: u32 = 42;
        assert_eq!(val.heap_size(), 0);
        assert_eq!(val.total_size(), core::mem::size_of::<u32>());

        struct Dummy {
            heap_allocated: usize,
        }

        impl EstimateSize for Dummy {
            fn heap_size(&self) -> usize {
                self.heap_allocated
            }
        }

        let dummy = Dummy {
            heap_allocated: 100,
        };
        assert_eq!(dummy.heap_size(), 100);
        assert_eq!(dummy.total_size(), core::mem::size_of::<Dummy>() + 100);
    }
}
