use turbo_bytes::{BufferPool, ByteBuffer, ByteReader, ByteView, ByteWriter, Cursor};
use turbo_core::alloc::GlobalAlloc;

fn main() -> turbo_core::Result<()> {
    // 1. ByteBuffer Basics
    println!("--- 1. ByteBuffer Basics ---");
    let mut buf = ByteBuffer::with_capacity(8, GlobalAlloc)?;
    buf.extend_from_slice(b"Turbo")?;
    buf.push(b'!')?;
    println!("ByteBuffer contents: {:?}", String::from_utf8_lossy(&buf));
    println!(
        "ByteBuffer len: {}, capacity: {}",
        buf.len(),
        buf.capacity()
    );

    // 2. Zero-Copy ByteView
    println!("\n--- 2. Zero-Copy ByteView ---");
    let view = ByteView::new(buf);
    let slice_view = view.slice(0..5)?;
    println!(
        "Sliced view contents: {:?}",
        String::from_utf8_lossy(&slice_view)
    );
    println!(
        "Original view length: {}, slice length: {}",
        view.len(),
        slice_view.len()
    );

    // 3. Cursor & Binary Read/Write
    println!("\n--- 3. Cursor & Binary Read/Write ---");
    let mut storage = [0u8; 16];
    {
        let mut cursor = Cursor::new(&mut storage[..]);
        cursor.write_u32_be(0x12345678)?;
        cursor.write_f64_le(1.23456789)?;
        println!("Cursor written bytes: {}", cursor.position());
    }

    {
        let mut cursor = Cursor::new(storage);
        let val_u32 = cursor.read_u32_be()?;
        let val_f64 = cursor.read_f64_le()?;
        println!("Read u32: 0x{:X}", val_u32);
        println!("Read f64: {}", val_f64);
    }

    // 4. BufferPool Recycling
    println!("\n--- 4. BufferPool Recycling ---");
    let pool = BufferPool::new();
    {
        let mut pooled_buffer = pool.acquire(1024)?;
        pooled_buffer.extend_from_slice(b"Cached memory is reusable!")?;
        println!(
            "Pooled buffer len: {}, capacity: {}",
            pooled_buffer.len(),
            pooled_buffer.capacity()
        );
        // pooled_buffer returned to pool automatically when dropped
    }

    // Acquire again - it should fetch the same buffer from the pool (already cleared)
    let recycled_buffer = pool.acquire(1024)?;
    println!(
        "Recycled buffer len (should be 0): {}",
        recycled_buffer.len()
    );
    println!("Recycled buffer capacity: {}", recycled_buffer.capacity());

    Ok(())
}
