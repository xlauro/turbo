use criterion::{black_box, criterion_group, criterion_main, Criterion};
use turbo_hash::HashMap;

fn bench_hash_map_insertion(c: &mut Criterion) {
    c.bench_function("std_hash_map_insert", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for i in 0..100 {
                map.insert(black_box(i), black_box(i * 10));
            }
            black_box(map);
        })
    });

    c.bench_function("turbo_hash_map_insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..100 {
                let _ = map.insert(black_box(i), black_box(i * 10));
            }
            black_box(map);
        })
    });
}

fn bench_hash_map_lookup(c: &mut Criterion) {
    let mut std_map = std::collections::HashMap::new();
    let mut turbo_map = HashMap::new();
    for i in 0..100 {
        std_map.insert(i, i * 10);
        let _ = turbo_map.insert(i, i * 10);
    }

    c.bench_function("std_hash_map_lookup", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..100 {
                if let Some(&val) = std_map.get(black_box(&i)) {
                    sum += val;
                }
            }
            black_box(sum);
        })
    });

    c.bench_function("turbo_hash_map_lookup", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..100 {
                if let Some(&val) = turbo_map.get(black_box(&i)) {
                    sum += val;
                }
            }
            black_box(sum);
        })
    });
}

criterion_group!(benches, bench_hash_map_insertion, bench_hash_map_lookup);
criterion_main!(benches);
