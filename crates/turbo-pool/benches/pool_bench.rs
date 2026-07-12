use criterion::{black_box, criterion_group, criterion_main, Criterion};
use turbo_pool::{DefaultRecycler, ObjectPool, Slab};

fn bench_slab_vs_box(c: &mut Criterion) {
    c.bench_function("heap_box_alloc", |b| {
        b.iter(|| {
            let x = Box::new(black_box(42));
            black_box(x);
        })
    });

    c.bench_function("slab_insert_remove", |b| {
        let mut slab = Slab::new();
        b.iter(|| {
            let k = slab.insert(black_box(42));
            let v = slab.remove(black_box(k));
            black_box(v);
        })
    });
}

fn bench_pool_vs_heap(c: &mut Criterion) {
    c.bench_function("heap_vector_alloc", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(32);
            v.push(black_box(42));
            black_box(v);
        })
    });

    c.bench_function("object_pool_checkout", |b| {
        let pool = ObjectPool::new(|| Vec::with_capacity(32), DefaultRecycler);
        b.iter(|| {
            let mut v = pool.checkout();
            v.push(black_box(42));
            black_box(v);
        })
    });
}

criterion_group!(benches, bench_slab_vs_box, bench_pool_vs_heap);
criterion_main!(benches);
