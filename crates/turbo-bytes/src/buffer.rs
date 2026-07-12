use core::alloc::Layout;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::NonNull;
use turbo_core::alloc::GlobalAlloc;
use turbo_core::{Error, Result, TurboAlloc};

/// A high-performance, custom-allocated, resizable byte buffer.
///
/// Under the hood, this delegates memory allocations to an implementation of
/// [`TurboAlloc`], allowing allocations to be arena-backed, pooled, or global.
pub struct ByteBuffer<A: TurboAlloc = GlobalAlloc> {
    ptr: NonNull<u8>,
    capacity: usize,
    len: usize,
    allocator: A,
}

unsafe impl<A: TurboAlloc + Send> Send for ByteBuffer<A> {}
unsafe impl<A: TurboAlloc + Sync> Sync for ByteBuffer<A> {}

impl<A: TurboAlloc> ByteBuffer<A> {
    /// Creates a new, empty `ByteBuffer` using the provided allocator.
    ///
    /// This does not allocate memory.
    pub const fn new(allocator: A) -> Self {
        Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
            allocator,
        }
    }

    /// Creates a new `ByteBuffer` with at least the specified capacity.
    pub fn with_capacity(capacity: usize, allocator: A) -> Result<Self> {
        let mut buf = Self::new(allocator);
        if capacity > 0 {
            buf.resize_capacity(capacity)?;
        }
        Ok(buf)
    }

    /// Returns the number of bytes in the buffer.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer contains no elements.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the total capacity of the buffer.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a raw pointer to the underlying buffer.
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    /// Returns an unsafe mutable raw pointer to the underlying buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Resizes the buffer capacity to exactly `new_capacity`.
    ///
    /// # Errors
    /// * Returns `Error::AllocError` if the allocation fails.
    /// * Returns `Error::CapacityOverflow` if capacity calculation overflows.
    pub fn resize_capacity(&mut self, new_capacity: usize) -> Result<()> {
        if new_capacity == self.capacity {
            return Ok(());
        }

        if new_capacity == 0 {
            if self.capacity > 0 {
                // SAFETY: self.ptr is currently allocated and layout is valid.
                unsafe {
                    let layout = Layout::from_size_align_unchecked(self.capacity, 1);
                    self.allocator.dealloc(self.ptr, layout);
                }
                self.ptr = NonNull::dangling();
                self.capacity = 0;
            }
            self.len = 0;
            return Ok(());
        }

        let new_layout =
            Layout::from_size_align(new_capacity, 1).map_err(|_| Error::CapacityOverflow {
                limit: usize::MAX,
                requested: new_capacity,
            })?;

        let new_ptr = if self.capacity == 0 {
            self.allocator.alloc(new_layout)?
        } else {
            // SAFETY: self.ptr is currently allocated, layout is valid, and new_capacity > 0.
            unsafe {
                let old_layout = Layout::from_size_align_unchecked(self.capacity, 1);
                self.allocator.realloc(self.ptr, old_layout, new_layout)?
            }
        };

        self.ptr = NonNull::new(new_ptr.as_ptr() as *mut u8).ok_or(Error::AllocError)?;
        self.capacity = new_capacity;

        if self.len > new_capacity {
            self.len = new_capacity;
        }

        Ok(())
    }

    /// Reserves capacity for at least `additional` more bytes to be inserted.
    pub fn reserve(&mut self, additional: usize) -> Result<()> {
        let required = self
            .len
            .checked_add(additional)
            .ok_or(Error::CapacityOverflow {
                limit: usize::MAX,
                requested: self.len + additional,
            })?;

        if required > self.capacity {
            let new_cap = core::cmp::max(self.capacity.saturating_mul(2), required);
            self.resize_capacity(new_cap)?;
        }
        Ok(())
    }

    /// Appends a byte to the back of the buffer.
    pub fn push(&mut self, byte: u8) -> Result<()> {
        self.reserve(1)?;
        // SAFETY: self.ptr is allocated for at least self.len + 1 bytes.
        unsafe {
            core::ptr::write(self.ptr.as_ptr().add(self.len), byte);
        }
        self.len += 1;
        Ok(())
    }

    /// Appends all bytes from a slice to the buffer.
    pub fn extend_from_slice(&mut self, slice: &[u8]) -> Result<()> {
        if slice.is_empty() {
            return Ok(());
        }
        self.reserve(slice.len())?;
        // SAFETY: both buffers are valid for copy of slice.len() bytes and do not overlap.
        unsafe {
            core::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                self.ptr.as_ptr().add(self.len),
                slice.len(),
            );
        }
        self.len += slice.len();
        Ok(())
    }

    /// Clears the buffer, resetting length to `0`. Does not deallocate memory.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Sets the length of the buffer.
    ///
    /// # Safety
    /// * `new_len` must be less than or equal to the buffer capacity.
    /// * The elements from current length up to `new_len` must be initialized.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    /// Returns a slice of the active elements.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        if self.len == 0 {
            &[]
        } else {
            // SAFETY: self.ptr is valid for reads up to self.len.
            unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
        }
    }

    /// Returns a mutable slice of the active elements.
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        if self.len == 0 {
            &mut []
        } else {
            // SAFETY: self.ptr is valid for writes up to self.len.
            unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
        }
    }
}

impl<A: TurboAlloc> Drop for ByteBuffer<A> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            // SAFETY: self.ptr is currently allocated and layout is valid.
            unsafe {
                let layout = Layout::from_size_align_unchecked(self.capacity, 1);
                self.allocator.dealloc(self.ptr, layout);
            }
        }
    }
}

impl<A: TurboAlloc + Clone> Clone for ByteBuffer<A> {
    fn clone(&self) -> Self {
        let mut new_buf = Self::new(self.allocator.clone());
        if self.len > 0 {
            new_buf.resize_capacity(self.len).unwrap();
            // SAFETY: both ptrs are valid for copy of self.len bytes and do not overlap.
            unsafe {
                core::ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_buf.ptr.as_ptr(), self.len);
            }
            new_buf.len = self.len;
        }
        new_buf
    }
}

impl<A: TurboAlloc> Deref for ByteBuffer<A> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<A: TurboAlloc> DerefMut for ByteBuffer<A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<A: TurboAlloc> AsRef<[u8]> for ByteBuffer<A> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<A: TurboAlloc> AsMut<[u8]> for ByteBuffer<A> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_slice_mut()
    }
}

impl<A: TurboAlloc, I: core::slice::SliceIndex<[u8]>> Index<I> for ByteBuffer<A> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<A: TurboAlloc, I: core::slice::SliceIndex<[u8]>> IndexMut<I> for ByteBuffer<A> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl<A: TurboAlloc> crate::writer::ByteWriter for ByteBuffer<A> {
    #[inline]
    fn write_u8(&mut self, val: u8) -> Result<()> {
        self.push(val)
    }

    #[inline]
    fn write_bytes(&mut self, src: &[u8]) -> Result<()> {
        self.extend_from_slice(src)
    }
}
