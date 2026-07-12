use crate::buffer::ByteBuffer;
use core::ops::{Deref, Range};
use std::sync::Arc;
use turbo_core::alloc::GlobalAlloc;
use turbo_core::{turbo_ensure, Error, Result};

/// A thread-safe, reference-counted zero-copy view of a byte buffer.
///
/// Cloning a `ByteView` is cheap because it only increments an atomic reference count,
/// sharing the underlying memory allocation. Slicing a `ByteView` returns a new
/// `ByteView` pointing to a sub-region of the same allocation.
#[derive(Clone)]
pub struct ByteView {
    buffer: Arc<ByteBuffer<GlobalAlloc>>,
    offset: usize,
    len: usize,
}

impl ByteView {
    /// Creates a new `ByteView` wrapping the provided `ByteBuffer`.
    pub fn new(buffer: ByteBuffer<GlobalAlloc>) -> Self {
        let len = buffer.len();
        Self {
            buffer: Arc::new(buffer),
            offset: 0,
            len,
        }
    }

    /// Creates a `ByteView` by copying the contents of a byte slice.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let mut buffer = ByteBuffer::with_capacity(slice.len(), GlobalAlloc)?;
        buffer.extend_from_slice(slice)?;
        Ok(Self::new(buffer))
    }

    /// Returns the length of the view in bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the view is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the start offset of this view relative to the underlying buffer.
    #[inline]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Returns a sub-slice of the view.
    ///
    /// This is a zero-copy operation that returns a new `ByteView` sharing the same buffer.
    ///
    /// # Errors
    /// * Returns `Error::OutOfBounds` if the range is outside the bounds of this view.
    pub fn slice(&self, range: Range<usize>) -> Result<Self> {
        turbo_ensure!(
            range.start <= range.end && range.end <= self.len,
            Error::OutOfBounds {
                index: range.end,
                len: self.len
            }
        );

        Ok(Self {
            buffer: Arc::clone(&self.buffer),
            offset: self.offset + range.start,
            len: range.end - range.start,
        })
    }

    /// Returns the contents of this view as a byte slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[self.offset..self.offset + self.len]
    }
}

impl Deref for ByteView {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl AsRef<[u8]> for ByteView {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl core::fmt::Debug for ByteView {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ByteView")
            .field("offset", &self.offset)
            .field("len", &self.len)
            .field("data", &self.as_slice())
            .finish()
    }
}
