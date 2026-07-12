use crate::small::SmallString;
use turbo_bytes::ByteBuffer;
use turbo_core::alloc::GlobalAlloc;
use turbo_core::Result;

/// A high-performance string builder designed for efficient text accumulation and formatting.
///
/// Wraps a [`ByteBuffer`] and implements [`core::fmt::Write`], allowing usage of standard
/// macros like [`core::write!`].
pub struct StringBuilder {
    buffer: ByteBuffer<GlobalAlloc>,
}

impl StringBuilder {
    /// Creates a new, empty `StringBuilder`.
    ///
    /// Does not allocate heap memory.
    pub fn new() -> Self {
        Self {
            buffer: ByteBuffer::new(GlobalAlloc),
        }
    }

    /// Creates a new `StringBuilder` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Result<Self> {
        let buffer = ByteBuffer::with_capacity(capacity, GlobalAlloc)?;
        Ok(Self { buffer })
    }

    /// Returns the length of the built string in bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the builder contains no bytes.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Appends a character to the builder.
    pub fn push(&mut self, ch: char) -> Result<()> {
        let mut char_bytes = [0u8; 4];
        let char_str = ch.encode_utf8(&mut char_bytes);
        self.push_str(char_str)
    }

    /// Appends a string slice to the builder.
    pub fn push_str(&mut self, s: &str) -> Result<()> {
        self.buffer.extend_from_slice(s.as_bytes())
    }

    /// Clears the contents of the builder, resetting its length to `0`.
    ///
    /// Keeps the pre-allocated memory.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Consumes the builder and returns the accumulated text as a [`SmallString`].
    pub fn into_string(self) -> Result<SmallString> {
        // Retrieve string slice from buffer
        // SAFETY: We only write valid UTF-8 via push and push_str.
        let slice = unsafe { core::str::from_utf8_unchecked(self.buffer.as_slice()) };
        SmallString::from_str(slice)
    }

    /// Returns a string slice referencing the current built contents.
    pub fn as_str(&self) -> &str {
        // SAFETY: We only write valid UTF-8.
        unsafe { core::str::from_utf8_unchecked(self.buffer.as_slice()) }
    }
}

impl Default for StringBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Write for StringBuilder {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.push_str(s).map_err(|_| core::fmt::Error)
    }
}

impl core::fmt::Display for StringBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
