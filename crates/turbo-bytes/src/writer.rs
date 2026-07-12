use turbo_core::Result;

/// A trait for writing bytes and binary primitives into a stream or buffer.
pub trait ByteWriter {
    /// Writes a single byte into the destination.
    fn write_u8(&mut self, val: u8) -> Result<()>;

    /// Writes a slice of bytes into the destination.
    fn write_bytes(&mut self, src: &[u8]) -> Result<()>;

    /// Writes a `u16` in little-endian format.
    #[inline]
    fn write_u16_le(&mut self, val: u16) -> Result<()> {
        self.write_bytes(&val.to_le_bytes())
    }

    /// Writes a `u16` in big-endian format.
    #[inline]
    fn write_u16_be(&mut self, val: u16) -> Result<()> {
        self.write_bytes(&val.to_be_bytes())
    }

    /// Writes a `u32` in little-endian format.
    #[inline]
    fn write_u32_le(&mut self, val: u32) -> Result<()> {
        self.write_bytes(&val.to_le_bytes())
    }

    /// Writes a `u32` in big-endian format.
    #[inline]
    fn write_u32_be(&mut self, val: u32) -> Result<()> {
        self.write_bytes(&val.to_be_bytes())
    }

    /// Writes a `u64` in little-endian format.
    #[inline]
    fn write_u64_le(&mut self, val: u64) -> Result<()> {
        self.write_bytes(&val.to_le_bytes())
    }

    /// Writes a `u64` in big-endian format.
    #[inline]
    fn write_u64_be(&mut self, val: u64) -> Result<()> {
        self.write_bytes(&val.to_be_bytes())
    }

    /// Writes an `i32` in little-endian format.
    #[inline]
    fn write_i32_le(&mut self, val: i32) -> Result<()> {
        self.write_bytes(&val.to_le_bytes())
    }

    /// Writes an `i32` in big-endian format.
    #[inline]
    fn write_i32_be(&mut self, val: i32) -> Result<()> {
        self.write_bytes(&val.to_be_bytes())
    }

    /// Writes an `i64` in little-endian format.
    #[inline]
    fn write_i64_le(&mut self, val: i64) -> Result<()> {
        self.write_bytes(&val.to_le_bytes())
    }

    /// Writes an `i64` in big-endian format.
    #[inline]
    fn write_i64_be(&mut self, val: i64) -> Result<()> {
        self.write_bytes(&val.to_be_bytes())
    }

    /// Writes an `f32` in little-endian format.
    #[inline]
    fn write_f32_le(&mut self, val: f32) -> Result<()> {
        self.write_bytes(&val.to_le_bytes())
    }

    /// Writes an `f32` in big-endian format.
    #[inline]
    fn write_f32_be(&mut self, val: f32) -> Result<()> {
        self.write_bytes(&val.to_be_bytes())
    }

    /// Writes an `f64` in little-endian format.
    #[inline]
    fn write_f64_le(&mut self, val: f64) -> Result<()> {
        self.write_bytes(&val.to_le_bytes())
    }

    /// Writes an `f64` in big-endian format.
    #[inline]
    fn write_f64_be(&mut self, val: f64) -> Result<()> {
        self.write_bytes(&val.to_be_bytes())
    }
}

// Blanket implementation for mutable references to writer
impl<W: ByteWriter + ?Sized> ByteWriter for &mut W {
    #[inline]
    fn write_u8(&mut self, val: u8) -> Result<()> {
        (**self).write_u8(val)
    }

    #[inline]
    fn write_bytes(&mut self, src: &[u8]) -> Result<()> {
        (**self).write_bytes(src)
    }
}
