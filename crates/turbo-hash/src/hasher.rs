use core::hash::{BuildHasher, Hasher};

/// A high-performance non-cryptographic hasher using the rustc FxHash algorithm.
///
/// This hasher is extremely fast for small keys (e.g., integers, short strings)
/// but does not provide protection against hash collision attacks (HashDoS).
pub struct FxHasher {
    hash: usize,
}

#[cfg(target_pointer_width = "32")]
const K: usize = 0x9e3779b9;
#[cfg(target_pointer_width = "64")]
const K: usize = 0x517cc1b727220a95;

impl FxHasher {
    /// Creates a new `FxHasher` initialized with a zero hash state.
    #[inline]
    pub const fn new() -> Self {
        Self { hash: 0 }
    }
}

impl Default for FxHasher {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut remainder = bytes;
        let usize_bytes = core::mem::size_of::<usize>();

        // Process in chunks of pointer width (4 or 8 bytes)
        while remainder.len() >= usize_bytes {
            let mut chunk = 0usize;
            // SAFETY: remainder is guaranteed to have at least usize_bytes.
            unsafe {
                core::ptr::copy_nonoverlapping(
                    remainder.as_ptr(),
                    &mut chunk as *mut usize as *mut u8,
                    usize_bytes,
                );
            }
            self.write_usize(chunk);
            remainder = &remainder[usize_bytes..];
        }

        // Process leftover bytes
        for &byte in remainder {
            self.write_u8(byte);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash = (self.hash.rotate_left(5) ^ (i as usize)).wrapping_mul(K);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash = (self.hash.rotate_left(5) ^ (i as usize)).wrapping_mul(K);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = (self.hash.rotate_left(5) ^ (i as usize)).wrapping_mul(K);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        #[cfg(target_pointer_width = "32")]
        {
            self.write_u32(i as u32);
            self.write_u32((i >> 32) as u32);
        }
        #[cfg(target_pointer_width = "64")]
        {
            self.hash = (self.hash.rotate_left(5) ^ (i as usize)).wrapping_mul(K);
        }
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash = (self.hash.rotate_left(5) ^ i).wrapping_mul(K);
    }
}

/// A builder for [`FxHasher`].
#[derive(Default, Clone, Copy, Debug)]
pub struct BuildFxHasher;

impl BuildHasher for BuildFxHasher {
    type Hasher = FxHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FxHasher::new()
    }
}
