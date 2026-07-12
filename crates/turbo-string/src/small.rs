use core::fmt;
use core::ops::Deref;
use turbo_bytes::ByteBuffer;
use turbo_core::alloc::GlobalAlloc;
use turbo_core::Result;

/// A memory-efficient string implementation utilizing Small String Optimization (SSO).
///
/// Strings up to 22 bytes are stored inline on the stack (without heap allocation).
/// Larger strings automatically spill over to a heap-allocated buffer.
#[derive(Clone)]
pub enum SmallString {
    /// Inline representation for small strings.
    Inline {
        /// Inline byte array storage.
        bytes: [u8; 22],
        /// Length of the inline string.
        len: u8,
    },
    /// Heap-allocated representation for larger strings.
    Heap(ByteBuffer<GlobalAlloc>),
}

impl SmallString {
    /// Creates a new, empty `SmallString`.
    #[inline]
    pub const fn new() -> Self {
        Self::Inline {
            bytes: [0u8; 22],
            len: 0,
        }
    }

    /// Creates a `SmallString` from a string slice.
    ///
    /// If the string slice length is 22 bytes or less, it is stored inline.
    /// Otherwise, it is allocated on the heap.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self> {
        let len = s.len();
        if len <= 22 {
            let mut bytes = [0u8; 22];
            bytes[..len].copy_from_slice(s.as_bytes());
            Ok(Self::Inline {
                bytes,
                len: len as u8,
            })
        } else {
            let mut buf = ByteBuffer::with_capacity(len, GlobalAlloc)?;
            buf.extend_from_slice(s.as_bytes())?;
            Ok(Self::Heap(buf))
        }
    }

    /// Returns the length of the string in bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        match self {
            Self::Inline { len, .. } => *len as usize,
            Self::Heap(buf) => buf.len(),
        }
    }

    /// Returns `true` if the string contains no bytes.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the string slice representation of the string.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Inline { bytes, len } => {
                // SAFETY: We only write valid UTF-8 to bytes, and len is <= 22.
                unsafe { core::str::from_utf8_unchecked(&bytes[..*len as usize]) }
            }
            Self::Heap(buf) => {
                // SAFETY: We only write valid UTF-8 to the underlying ByteBuffer.
                unsafe { core::str::from_utf8_unchecked(buf.as_slice()) }
            }
        }
    }

    /// Appends a character to the back of the string.
    pub fn push(&mut self, ch: char) -> Result<()> {
        let mut char_bytes = [0u8; 4];
        let char_str = ch.encode_utf8(&mut char_bytes);
        self.push_str(char_str)
    }

    /// Appends a string slice to the back of the string.
    pub fn push_str(&mut self, s: &str) -> Result<()> {
        if s.is_empty() {
            return Ok(());
        }

        match self {
            Self::Inline { bytes, len } => {
                let current_len = *len as usize;
                let new_len = current_len + s.len();
                if new_len <= 22 {
                    bytes[current_len..new_len].copy_from_slice(s.as_bytes());
                    *len = new_len as u8;
                    Ok(())
                } else {
                    // Spill to Heap
                    let mut buf = ByteBuffer::with_capacity(new_len, GlobalAlloc)?;
                    buf.extend_from_slice(&bytes[..current_len])?;
                    buf.extend_from_slice(s.as_bytes())?;
                    *self = Self::Heap(buf);
                    Ok(())
                }
            }
            Self::Heap(buf) => buf.extend_from_slice(s.as_bytes()),
        }
    }

    /// Clears the contents of the string, resetting its length to `0`.
    ///
    /// Does not release heap allocation if currently in `Heap` state.
    pub fn clear(&mut self) {
        match self {
            Self::Inline { bytes, len } => {
                *bytes = [0u8; 22];
                *len = 0;
            }
            Self::Heap(buf) => {
                buf.clear();
            }
        }
    }
}

impl Default for SmallString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for SmallString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for SmallString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for SmallString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl fmt::Display for SmallString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for SmallString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl PartialEq for SmallString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for SmallString {}

impl PartialEq<str> for SmallString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for SmallString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd for SmallString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl Ord for SmallString {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl core::hash::Hash for SmallString {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
