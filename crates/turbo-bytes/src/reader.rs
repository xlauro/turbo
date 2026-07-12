use turbo_core::Result;

/// A trait for reading bytes and binary primitives from a stream or buffer.
pub trait ByteReader {
    /// Reads a single byte from the source.
    fn read_u8(&mut self) -> Result<u8>;

    /// Reads exact number of bytes from the source into the destination buffer.
    fn read_bytes(&mut self, dest: &mut [u8]) -> Result<()>;

    /// Reads a `u16` in little-endian format.
    #[inline]
    fn read_u16_le(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read_bytes(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Reads a `u16` in big-endian format.
    #[inline]
    fn read_u16_be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read_bytes(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Reads a `u32` in little-endian format.
    #[inline]
    fn read_u32_le(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    /// Reads a `u32` in big-endian format.
    #[inline]
    fn read_u32_be(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    /// Reads a `u64` in little-endian format.
    #[inline]
    fn read_u64_le(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    /// Reads a `u64` in big-endian format.
    #[inline]
    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    /// Reads an `i32` in little-endian format.
    #[inline]
    fn read_i32_le(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    /// Reads an `i32` in big-endian format.
    #[inline]
    fn read_i32_be(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    /// Reads an `i64` in little-endian format.
    #[inline]
    fn read_i64_le(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    /// Reads an `i64` in big-endian format.
    #[inline]
    fn read_i64_be(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    /// Reads an `f32` in little-endian format.
    #[inline]
    fn read_f32_le(&mut self) -> Result<f32> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    /// Reads an `f32` in big-endian format.
    #[inline]
    fn read_f32_be(&mut self) -> Result<f32> {
        let mut buf = [0u8; 4];
        self.read_bytes(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    /// Reads an `f64` in little-endian format.
    #[inline]
    fn read_f64_le(&mut self) -> Result<f64> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    /// Reads an `f64` in big-endian format.
    #[inline]
    fn read_f64_be(&mut self) -> Result<f64> {
        let mut buf = [0u8; 8];
        self.read_bytes(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }
}

// Blanket implementation for mutable references to reader
impl<R: ByteReader + ?Sized> ByteReader for &mut R {
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        (**self).read_u8()
    }

    #[inline]
    fn read_bytes(&mut self, dest: &mut [u8]) -> Result<()> {
        (**self).read_bytes(dest)
    }
}
