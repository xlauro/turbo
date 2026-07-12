use core::fmt;
use core::ops::{Deref, Range};
use turbo_bytes::ByteView;
use turbo_core::{turbo_ensure, Error, Result};

/// A thread-safe, reference-counted zero-copy string slice.
///
/// Wraps a [`ByteView`] from `turbo-bytes` and guarantees that the underlying data
/// is valid UTF-8. Cloning and slicing are cheap operations that reuse the parent memory.
#[derive(Clone)]
pub struct StringView {
    inner: ByteView,
}

impl StringView {
    /// Creates a new `StringView` from a `ByteView`.
    ///
    /// # Errors
    /// * Returns `Error::InvalidData` if the bytes are not valid UTF-8.
    pub fn new(view: ByteView) -> Result<Self> {
        core::str::from_utf8(&view).map_err(|_| Error::InvalidData {
            reason: "Invalid UTF-8 sequence in ByteView",
        })?;
        Ok(Self { inner: view })
    }

    /// Creates a new `StringView` by copying the contents of a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self> {
        let byte_view = ByteView::from_slice(s.as_bytes())?;
        Ok(Self { inner: byte_view })
    }

    /// Returns the length of the string view in bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the string view is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the start offset of this view relative to the underlying buffer.
    #[inline]
    pub const fn offset(&self) -> usize {
        self.inner.offset()
    }

    /// Returns a string slice of the contents of this view.
    #[inline]
    pub fn as_str(&self) -> &str {
        // SAFETY: We validate UTF-8 on creation and slicing.
        unsafe { core::str::from_utf8_unchecked(self.inner.as_slice()) }
    }

    /// Returns a sub-slice of this view.
    ///
    /// This is a zero-copy operation that returns a new `StringView` sharing the same buffer.
    ///
    /// # Errors
    /// * Returns `Error::OutOfBounds` if the range is outside the bounds of this view.
    /// * Returns `Error::InvalidData` if the range boundaries do not lie on UTF-8 char boundaries.
    pub fn slice(&self, range: Range<usize>) -> Result<Self> {
        let s = self.as_str();

        turbo_ensure!(
            range.start <= range.end && range.end <= s.len(),
            Error::OutOfBounds {
                index: range.end,
                len: s.len()
            }
        );

        turbo_ensure!(
            s.is_char_boundary(range.start) && s.is_char_boundary(range.end),
            Error::InvalidData {
                reason: "Slice range boundaries are not on character boundaries"
            }
        );

        let sub_byte_view = self.inner.slice(range)?;
        Ok(Self {
            inner: sub_byte_view,
        })
    }
}

impl Deref for StringView {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for StringView {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for StringView {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl fmt::Display for StringView {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for StringView {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl PartialEq for StringView {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for StringView {}

impl PartialEq<str> for StringView {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for StringView {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd for StringView {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl Ord for StringView {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl core::hash::Hash for StringView {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
