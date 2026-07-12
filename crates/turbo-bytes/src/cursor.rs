use crate::reader::ByteReader;
use crate::writer::ByteWriter;
use turbo_core::{turbo_ensure, Error, Result};

/// A cursor wrapper that tracks position over an underlying byte storage.
pub struct Cursor<T> {
    inner: T,
    pos: usize,
}

impl<T> Cursor<T> {
    /// Creates a new `Cursor` initialized at position `0`.
    pub const fn new(inner: T) -> Self {
        Self { inner, pos: 0 }
    }

    /// Returns the current read/write position of the cursor.
    #[inline]
    pub const fn position(&self) -> usize {
        self.pos
    }

    /// Sets the position of the cursor.
    #[inline]
    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Consumes this cursor, returning the underlying storage.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Returns a reference to the underlying storage.
    #[inline]
    pub const fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the underlying storage.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: AsRef<[u8]>> ByteReader for Cursor<T> {
    fn read_u8(&mut self) -> Result<u8> {
        let slice = self.inner.as_ref();
        turbo_ensure!(
            self.pos < slice.len(),
            Error::OutOfBounds {
                index: self.pos,
                len: slice.len()
            }
        );
        let byte = slice[self.pos];
        self.pos += 1;
        Ok(byte)
    }

    fn read_bytes(&mut self, dest: &mut [u8]) -> Result<()> {
        let slice = self.inner.as_ref();
        let end = self
            .pos
            .checked_add(dest.len())
            .ok_or_else(|| Error::OutOfBounds {
                index: self.pos + dest.len(),
                len: slice.len(),
            })?;
        turbo_ensure!(
            end <= slice.len(),
            Error::OutOfBounds {
                index: end,
                len: slice.len()
            }
        );
        dest.copy_from_slice(&slice[self.pos..end]);
        self.pos = end;
        Ok(())
    }
}

impl<T: AsMut<[u8]>> ByteWriter for Cursor<T> {
    fn write_u8(&mut self, val: u8) -> Result<()> {
        let slice = self.inner.as_mut();
        turbo_ensure!(
            self.pos < slice.len(),
            Error::OutOfBounds {
                index: self.pos,
                len: slice.len()
            }
        );
        slice[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    fn write_bytes(&mut self, src: &[u8]) -> Result<()> {
        let slice = self.inner.as_mut();
        let end = self
            .pos
            .checked_add(src.len())
            .ok_or_else(|| Error::OutOfBounds {
                index: self.pos + src.len(),
                len: slice.len(),
            })?;
        turbo_ensure!(
            end <= slice.len(),
            Error::OutOfBounds {
                index: end,
                len: slice.len()
            }
        );
        slice[self.pos..end].copy_from_slice(src);
        self.pos = end;
        Ok(())
    }
}
