use criterion::{black_box, criterion_group, criterion_main, Criterion};
use turbo_collections::{Arena, DenseMap};

fn bench_arena_ops(c: &mut Criterion) {
    c.bench_function("arena_insert_remove", |b| {
        let mut arena = Arena::new();
        b.iter(|| {
            let idx = arena.insert(black_box(42));
            let val = arena.remove(black_box(idx));
            black_box(val);
        })
    });

    let mut arena = Arena::new();
    let mut indices = Vec::new();
    for i in 0..1000 {
        indices.push(arena.insert(i));
    }

    c.bench_function("arena_lookup", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &idx in &indices {
                if let Some(&val) = arena.get(idx) {
                    sum += val;
                }
            }
            black_box(sum);
        })
    });

    c.bench_function("arena_iter", |b| {
        b.iter(|| {
            let mut sum = 0;
            for (_, &val) in arena.iter() {
                sum += val;
            }
            black_box(sum);
        })
    });
}

fn bench_dense_map_ops(c: &mut Criterion) {
    c.bench_function("dense_map_insert_remove", |b| {
        let mut map = DenseMap::new();
        b.iter(|| {
            let idx = map.insert(black_box(42));
            let val = map.remove(black_box(idx));
            black_box(val);
        })
    });

    let mut map = DenseMap::new();
    let mut indices = Vec::new();
    for i in 0..1000 {
        indices.push(map.insert(i));
    }

    c.bench_function("dense_map_lookup", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &idx in &indices {
                if let Some(&val) = map.get(idx) {
                    sum += val;
                }
            }
            black_box(sum);
        })
    });

    c.bench_function("dense_map_iter", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &val in map.iter() {
                sum += val;
            }
            black_box(sum);
        })
    });
}

criterion_group!(benches, bench_arena_ops, bench_dense_map_ops);
criterion_main!(benches);
