use turbo_bytes::{BufferPool, ByteBuffer, ByteReader, ByteView, ByteWriter, Cursor};
use turbo_core::alloc::GlobalAlloc;

#[test]
fn test_byte_buffer_basic() {
    let mut buf = ByteBuffer::with_capacity(16, GlobalAlloc).unwrap();
    assert_eq!(buf.len(), 0);
    assert!(buf.capacity() >= 16);

    buf.push(10).unwrap();
    buf.push(20).unwrap();
    assert_eq!(buf.len(), 2);
    assert_eq!(buf[0], 10);
    assert_eq!(buf[1], 20);

    buf.extend_from_slice(&[30, 40, 50]).unwrap();
    assert_eq!(buf.len(), 5);
    assert_eq!(&buf[..], &[10, 20, 30, 40, 50]);

    buf.resize_capacity(8).unwrap();
    assert!(buf.capacity() >= 8);

    buf.clear();
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_byte_view_zero_copy() {
    let mut buf = ByteBuffer::new(GlobalAlloc);
    buf.extend_from_slice(b"hello world").unwrap();

    let view = ByteView::new(buf);
    assert_eq!(view.len(), 11);
    assert_eq!(view.as_ref(), b"hello world");

    // Zero-copy sub-slice view
    let sub = view.slice(6..11).unwrap();
    assert_eq!(sub.len(), 5);
    assert_eq!(sub.as_ref(), b"world");
    assert_eq!(sub.offset(), 6);

    // Verify it is Send/Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ByteView>();
}

#[test]
fn test_cursor_read_write_binary() {
    let mut memory = [0u8; 32];

    // Writing
    {
        let mut cursor = Cursor::new(&mut memory[..]);
        cursor.write_u8(0xFF).unwrap();
        cursor.write_u16_le(0x1234).unwrap();
        cursor.write_u32_be(0xABCDEF12).unwrap();
        cursor.write_f64_le(123.456789).unwrap();
        assert_eq!(cursor.position(), 1 + 2 + 4 + 8);
    }

    // Reading
    {
        let cursor_storage = memory; // copy backing slice
        let mut cursor = Cursor::new(cursor_storage);
        assert_eq!(cursor.read_u8().unwrap(), 0xFF);
        assert_eq!(cursor.read_u16_le().unwrap(), 0x1234);
        assert_eq!(cursor.read_u32_be().unwrap(), 0xABCDEF12);
        assert_eq!(cursor.read_f64_le().unwrap(), 123.456789);
    }
}

#[test]
fn test_buffer_pool_recyling() {
    let pool = BufferPool::new();

    // Acquire buffer
    let mut pooled1 = pool.acquire(1024).unwrap();
    assert!(pooled1.capacity() >= 4096); // matches Bucket 0 size (4KB)
    pooled1.push(42).unwrap();
    assert_eq!(pooled1.len(), 1);

    // Return to pool (via Drop)
    drop(pooled1);

    // Acquire again - should hit cache
    let pooled2 = pool.acquire(1024).unwrap();
    assert!(pooled2.capacity() >= 4096);
    // Verified it was cleared
    assert_eq!(pooled2.len(), 0);
}
