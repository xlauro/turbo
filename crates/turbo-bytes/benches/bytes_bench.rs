use criterion::{black_box, criterion_group, criterion_main, Criterion};
use turbo_bytes::{BufferPool, ByteBuffer, ByteWriter, Cursor};
use turbo_core::alloc::GlobalAlloc;

fn bench_pool_vs_alloc(c: &mut Criterion) {
    let pool = BufferPool::new();

    c.bench_function("pool_acquire_and_recycle_4k", |b| {
        b.iter(|| {
            let buf = pool.acquire(black_box(1024)).unwrap();
            black_box(buf); // dropped immediately, returning to pool
        })
    });

    c.bench_function("global_alloc_new_4k", |b| {
        b.iter(|| {
            let buf = ByteBuffer::with_capacity(black_box(4096), GlobalAlloc).unwrap();
            black_box(buf); // dropped immediately, calling system dealloc
        })
    });
}

fn bench_binary_writes(c: &mut Criterion) {
    c.bench_function("cursor_write_u32_le_loop", |b| {
        b.iter(|| {
            let mut storage = [0u8; 4096];
            let mut cursor = Cursor::new(&mut storage[..]);
            for i in 0..1000 {
                let _ = cursor.write_u32_le(black_box(i));
            }
        })
    });
}

criterion_group!(benches, bench_pool_vs_alloc, bench_binary_writes);
criterion_main!(benches);
